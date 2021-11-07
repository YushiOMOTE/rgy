/// Handles high ram access between 0xff80 - 0xfffe
pub struct Hram {
    bank: [u8; 0x7f],
}

impl Hram {
    pub fn new() -> Self {
        Self { bank: [0; 0x7f] }
    }

    pub fn get8(&self, addr: u16) -> u8 {
        self.bank[addr as usize - 0xff80]
    }

    pub fn set8(&mut self, addr: u16, v: u8) {
        self.bank[addr as usize - 0xff80] = v;
    }
}
