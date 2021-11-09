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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_bank() {
        let m = Wram::new();
        assert_eq!(m.get_bank(), 1);
    }

    #[test]
    fn test_select_bank() {
        let mut m = Wram::new();

        // fill bank 0
        for i in 0..0x1000 {
            m.set8(0xc000 + i, 0xf)
        }

        // fill bank 1-7
        for b in 1..8 {
            m.select_bank(b);
            for i in 0..0x1000 {
                m.set8(0xd000 + i, b)
            }
        }

        // check each bank
        for b in 1..8 {
            m.select_bank(b);
            for i in 0..0x1000 {
                assert_eq!(m.get8(0xc000 + i), 0xf);
                assert_eq!(m.get8(0xd000 + i), b);
            }
        }
    }

    #[test]
    fn test_mirror() {
        let mut m = Wram::new();

        for v in 1..4 {
            for i in 0..0x1dff {
                m.set8(0xc000 + i, v);
            }
            for i in 0..0x1dff {
                assert_eq!(m.get8(0xe000 + i), v);
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_set_too_low() {
        let mut m = Wram::new();
        m.set8(0xbfff, 0);
    }

    #[test]
    #[should_panic]
    fn test_set_too_high() {
        let mut m = Wram::new();
        m.set8(0xfe00, 0);
    }

    #[test]
    #[should_panic]
    fn test_get_too_low() {
        let m = Wram::new();
        m.get8(0xbfff);
    }

    #[test]
    #[should_panic]
    fn test_get_too_high() {
        let m = Wram::new();
        m.get8(0xfe00);
    }
}
