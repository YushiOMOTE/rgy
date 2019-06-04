use log::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

pub enum MemRead {
    Replace(u8),
    PassThrough,
}

pub enum MemWrite {
    Replace(u8),
    PassThrough,
    Block,
}

pub trait MemHandler {
    fn on_read(&self, mmu: &Mmu, addr: u16) -> MemRead;

    fn on_write(&self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite;
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Handle(u64);

pub struct Mmu {
    ram: Vec<u8>,
    handles: HashMap<Handle, (u16, u16)>,
    handlers: HashMap<u16, Vec<(Handle, Rc<MemHandler>)>>,
    hdgen: u64,
    use_boot_rom: bool,
    boot_rom: Vec<u8>,
}

impl Mmu {
    pub fn new() -> Mmu {
        Mmu {
            ram: vec![0u8; 0x10000],
            handles: HashMap::new(),
            handlers: HashMap::new(),
            hdgen: 0,
            use_boot_rom: true,
            boot_rom: vec![0u8; 0x100],
        }
    }

    fn next_handle(&mut self) -> Handle {
        let handle = self.hdgen;

        self.hdgen += 1;

        Handle(handle)
    }

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

    pub fn setup(&mut self, rom_name: &str) {
        self.load_boot_rom();
        self.load_rom(rom_name);
    }

    fn load_boot_rom(&mut self) {
        let mut f = File::open("boot.bin").expect("Couldn't open file");
        let mut buf = vec![0; 0x100];

        f.read(buf.as_mut_slice()).expect("Couldn't read file");

        for i in 0..buf.len() {
            self.boot_rom[i] = buf[i];
        }
    }

    fn load_rom(&mut self, rom_name: &str) {
        let mut f = File::open(rom_name).expect("Couldn't open file");
        let mut buf = vec![0; 0x8000];

        f.read(buf.as_mut_slice()).expect("Couldn't read file");

        for i in 0..buf.len() {
            self.ram[i] = buf[i];
        }
    }

    pub fn get8(&self, addr: u16) -> u8 {
        if let Some(handlers) = self.handlers.get(&addr) {
            for (_, handler) in handlers {
                match handler.on_read(self, addr) {
                    MemRead::Replace(alt) => return alt,
                    MemRead::PassThrough => {}
                }
            }
        }

        if self.use_boot_rom && addr < 0x100 {
            self.boot_rom[addr as usize]
        } else {
            if addr >= 0xe000 && addr <= 0xfdff {
                // echo ram
                self.ram[addr as usize - 0x2000]
            } else {
                self.ram[addr as usize]
            }
        }
    }

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

        if addr == 0xff46 {
            info!("MMU: Trigger DMA transfer: {:02x}", v);
            let src = (v as u16) << 8;
            for i in 0..0xa0 {
                self.set8(0xfe00 + i, self.get8(src + i));
            }
        }

        if self.use_boot_rom && addr < 0x100 {
            self.boot_rom[addr as usize] = v
        } else if self.use_boot_rom && addr == 0xff50 {
            // Turn off boot rom
            self.use_boot_rom = false;

            info!("Boot rom turned off");
        } else {
            if addr >= 0xe000 && addr <= 0xfdff {
                // echo ram
                self.ram[addr as usize - 0x2000] = v
            } else {
                self.ram[addr as usize] = v
            }
        }
    }

    pub fn get16(&self, addr: u16) -> u16 {
        let l = self.get8(addr);
        let h = self.get8(addr + 1);
        (h as u16) << 8 | l as u16
    }

    pub fn set16(&mut self, addr: u16, v: u16) {
        self.set8(addr, v as u8);
        self.set8(addr + 1, (v >> 8) as u8);
    }
}
