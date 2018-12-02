use std::io::prelude::*;
use std::fs::File;
use std::rc::Rc;
use std::collections::HashMap;

pub trait ReadHandler {
    fn on_read(&self, mmu: &Mmu, addr: u16) -> Option<u8>;
}

pub trait WriteHandler {
    fn on_write(&self, mmu: &Mmu, addr: u16, value: u8) -> Option<u8>;
}

pub struct Mmu {
    ram: Vec<u8>,
    rdhooks: HashMap<u16, Rc<ReadHandler>>,
    wrhooks: HashMap<u16, Rc<WriteHandler>>,
}

impl Mmu {
    pub fn new() -> Mmu {
        Mmu {
            ram: vec![0u8; 0x10000],
            rdhooks: HashMap::new(),
            wrhooks: HashMap::new(),
        }
    }

    pub fn add_rdhooks<T: ReadHandler + 'static>(&mut self, range: (u16, u16), handler: T) {
        let handler = Rc::new(handler);
        for i in range.0..range.1 {
            self.rdhooks.insert(i, handler.clone());
        }
    }

    pub fn add_wrhooks<T: WriteHandler + 'static>(&mut self, range: (u16, u16), handler: T) {
        let handler = Rc::new(handler);
        for i in range.0..range.1 {
            self.wrhooks.insert(i, handler.clone());
        }
    }

    pub fn remove_rdhook(&mut self, range: (u16, u16)) {
        for i in range.0..range.1 {
            self.rdhooks.remove(&i);
        }
    }

    pub fn remove_wrhook(&mut self, range: (u16, u16)) {
        for i in range.0..range.1 {
            self.wrhooks.remove(&i);
        }
    }

    pub fn load(&mut self) {
        let mut f = File::open("boot.bin").expect("Couldn't open file");
        let mut buf = vec![0; 256];

        f.read(buf.as_mut_slice()).expect("Couldn't read file");

        for i in 0..buf.len() {
            self.set8(i as u16, buf[i]);
        }
        for i in 0..48 {
            self.set8(i as u16 + 0x104, buf[i + 0xa8]);
        }
    }

    pub fn get8(&self, addr: u16) -> u8 {
        if let Some(handler) = self.rdhooks.get(&addr) {
            if let Some(alt) = handler.on_read(self, addr) {
                return alt;
            }
        }

        self.ram[addr as usize]
    }

    pub fn set8(&mut self, addr: u16, v: u8) {
        if let Some(handler) = self.wrhooks.get(&addr) {
            if let Some(alt) = handler.on_write(self, addr, v) {
                self.ram[addr as usize] = v;

                return;
            }
        }

        self.ram[addr as usize] = v
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
