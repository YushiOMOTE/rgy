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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_set() {
        let mut m = Hram::new();
        for i in 0..0x7f {
            m.set8(0xff80 + i, i as u8);
        }
        for i in 0..0x7f {
            assert_eq!(m.get8(0xff80 + i), i as u8);
        }
    }

    #[test]
    #[should_panic]
    fn test_set_too_low() {
        let mut m = Hram::new();
        m.set8(0xff7f, 0);
    }

    #[test]
    #[should_panic]
    fn test_set_too_high() {
        let mut m = Hram::new();
        m.set8(0xffff, 0);
    }

    #[test]
    #[should_panic]
    fn test_get_too_low() {
        let m = Hram::new();
        m.get8(0xff7f);
    }

    #[test]
    #[should_panic]
    fn test_get_too_high() {
        let m = Hram::new();
        m.get8(0xffff);
    }
}
