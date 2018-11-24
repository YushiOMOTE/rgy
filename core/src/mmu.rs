use std::cell::Cell;

pub struct Mmu {
    ram: Vec<Cell<u8>>,
}

impl Mmu {
    pub fn get8<T: Into<usize>>(&self, addr: T) -> u8 {
        self.ram[addr.into()].get()
    }

    pub fn set8<T: Into<usize>>(&self, addr: T, v: u8) {
        self.ram[addr.into()].set(v)
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
