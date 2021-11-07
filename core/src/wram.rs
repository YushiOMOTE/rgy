use log::*;

/// Handles work ram access between 0xc000 - 0xdfff
pub struct Wram {
    n: usize,
    bank: [[u8; 0x1000]; 8],
}

impl Wram {
    pub fn new() -> Self {
        Self {
            n: 1,
            bank: [[0; 0x1000]; 8],
        }
    }

    pub fn select_bank(&mut self, n: u8) {
        self.n = (n as usize & 0x7).max(1);
        info!("WRAM bank selected: {:02x}", self.n);
    }

    pub fn get_bank(&self) -> u8 {
        self.n as u8
    }

    pub fn get8(&self, addr: u16) -> u8 {
        match addr {
            0xc000..=0xcfff => self.bank[0][addr as usize - 0xc000],
            0xd000..=0xdfff => self.bank[self.n][addr as usize - 0xd000],
            0xe000..=0xfdff => self.get8(addr - 0xe000 + 0xc000),
            _ => unreachable!("read attemp to wram addr={:04x}", addr),
        }
    }

    pub fn set8(&mut self, addr: u16, v: u8) {
        match addr {
            0xc000..=0xcfff => self.bank[0][addr as usize - 0xc000] = v,
            0xd000..=0xdfff => self.bank[self.n][addr as usize - 0xd000] = v,
            0xe000..=0xfdff => self.set8(addr - 0xe000 + 0xc000, v),
            _ => unreachable!("write attemp to wram addr={:04x} v={:02x}", addr, v),
        }
    }
}
