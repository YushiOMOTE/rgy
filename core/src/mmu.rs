use std::io::prelude::*;
use std::fs::File;
use std::cell::RefCell;

pub struct Mmu {
    ram: RefCell<Vec<u8>>,
}

impl Mmu {
    pub fn new() -> Mmu {
        Mmu {
            ram: RefCell::new(vec![0u8; 0x10000]),
        }
    }

    pub fn load(&self) {
        let mut f = File::open("boot.bin").expect("Couldn't open file");
        let mut buf = vec![0; 256];
        let count = f.read(buf.as_mut_slice()).expect("Couldn't read file");

        for i in 0..buf.len() {
            self.set8(i, buf[i]);
            self.set8(i + 0x104, buf[i]);
        }
    }

    pub fn get8<T: Into<usize>>(&self, addr: T) -> u8 {
        self.ram.borrow_mut()[addr.into()]
    }

    pub fn set8<T: Into<usize>>(&self, addr: T, v: u8) {
        self.ram.borrow_mut()[addr.into()] = v
    }

    pub fn get16<T: Into<usize>>(&self, addr: T) -> u16 {
        let addr: usize = addr.into();
        let l = self.get8(addr);
        let h = self.get8(addr + 1);
        (h as u16) << 8 | l as u16
    }

    pub fn set16<T: Into<usize>>(&self, addr: T, v: u16) {
        let addr: usize = addr.into();
        self.set8(addr, v as u8);
        self.set8(addr + 1, (v >> 8) as u8);
    }
}
