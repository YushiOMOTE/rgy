use alloc::rc::Rc;
use alloc::{vec, vec::Vec};
use hashbrown::HashMap;

/// The variants to control memory read access from the CPU.
pub enum MemRead {
    /// Replaces the value passed from the memory to the CPU.
    Replace(u8),
    /// Shows the actual value passed from the memory to the CPU.
    PassThrough,
}

/// The variants to control memory write access from the CPU.
pub enum MemWrite {
    /// Replaces the value to be written by the CPU to the memory.
    Replace(u8),
    /// Allows to write the original value from the CPU to the memory.
    PassThrough,
    /// Discard the write access from the CPU.
    Block,
}

/// The handler to intercept memory access from the CPU.
pub trait MemHandler {
    /// The function is called when the CPU attempts to read from the memory.
    fn on_read(&self, mmu: &Mmu, addr: u16) -> MemRead;

    /// The function is called when the CPU attempts to write to the memory.
    fn on_write(&self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite;
}

/// The handle of a memory handler.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Handle(u64);

/// The memory management unit (MMU)
///
/// This unit holds a memory byte array which represents address space of the memory.
/// It provides the logic to intercept access from the CPU to the memory byte array,
/// and to modify the memory access behaviour.
pub struct Mmu {
    ram: Vec<u8>,
    handles: HashMap<Handle, (u16, u16)>,
    handlers: HashMap<u16, Vec<(Handle, Rc<dyn MemHandler>)>>,
    hdgen: u64,
}

impl Mmu {
    /// Create a new MMU instance.
    pub fn new() -> Mmu {
        Mmu {
            ram: vec![0u8; 0x10000],
            handles: HashMap::new(),
            handlers: HashMap::new(),
            hdgen: 0,
        }
    }

    fn next_handle(&mut self) -> Handle {
        let handle = self.hdgen;

        self.hdgen += 1;

        Handle(handle)
    }

    /// Add a new memory handler.
    pub fn add_handler<T>(&mut self, range: (u16, u16), handler: T) -> Handle
    where
        T: MemHandler + 'static,
    {
        let handle = self.next_handle();
        let handler = Rc::new(handler);

        self.handles.insert(handle.clone(), range);

        for i in range.0..=range.1 {
            if self.handlers.contains_key(&i) {
                match self.handlers.get_mut(&i) {
                    Some(v) => v.push((handle.clone(), handler.clone())),
                    None => {}
                }
            } else {
                self.handlers
                    .insert(i, vec![(handle.clone(), handler.clone())]);
            }
        }

        handle
    }

    /// Remove a memory handler.
    #[allow(unused)]
    pub fn remove_handler<T>(&mut self, handle: &Handle)
    where
        T: MemHandler + 'static,
    {
        let range = match self.handles.remove(&handle) {
            Some(range) => range,
            None => return,
        };

        for i in range.0..range.1 {
            match self.handlers.get_mut(&i) {
                Some(v) => v.retain(|(hd, _)| hd != handle),
                None => {}
            }
        }
    }

    /// Reads one byte from the given address in the memory.
    pub fn get8(&self, addr: u16) -> u8 {
        if let Some(handlers) = self.handlers.get(&addr) {
            for (_, handler) in handlers {
                match handler.on_read(self, addr) {
                    MemRead::Replace(alt) => return alt,
                    MemRead::PassThrough => {}
                }
            }
        }

        if addr >= 0xe000 && addr <= 0xfdff {
            // echo ram
            self.ram[addr as usize - 0x2000]
        } else {
            self.ram[addr as usize]
        }
    }

    /// Writes one byte at the given address in the memory.
    pub fn set8(&mut self, addr: u16, v: u8) {
        if let Some(handlers) = self.handlers.get(&addr) {
            for (_, handler) in handlers {
                match handler.on_write(self, addr, v) {
                    MemWrite::Replace(alt) => {
                        self.ram[addr as usize] = alt;
                        return;
                    }
                    MemWrite::PassThrough => {}
                    MemWrite::Block => return,
                }
            }
        }

        if addr >= 0xe000 && addr <= 0xfdff {
            // echo ram
            self.ram[addr as usize - 0x2000] = v
        } else {
            self.ram[addr as usize] = v
        }
    }

    /// Reads two bytes from the given addresss in the memory.
    pub fn get16(&self, addr: u16) -> u16 {
        let l = self.get8(addr);
        let h = self.get8(addr + 1);
        (h as u16) << 8 | l as u16
    }

    /// Writes two bytes at the given address in the memory.
    pub fn set16(&mut self, addr: u16, v: u16) {
        self.set8(addr, v as u8);
        self.set8(addr + 1, (v >> 8) as u8);
    }
}
