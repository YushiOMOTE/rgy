pub struct Cgb {
    double_speed: bool,
    speed_switch: bool,
}

#[allow(unused)]
impl Cgb {
    pub fn new() -> Self {
        Self {
            double_speed: false,
            speed_switch: false,
        }
    }

    pub fn try_switch_speed(&mut self) {
        if self.speed_switch {
            self.double_speed = !self.double_speed;
            self.speed_switch = false;
        }
    }

    pub fn double_speed(&self) -> bool {
        self.double_speed
    }

    /// Read KEY1 register (0xff4d)
    pub fn read_speed_switch(&self) -> u8 {
        let mut v = 0;
        v |= if self.double_speed { 0x80 } else { 0x00 };
        v |= if self.speed_switch { 0x01 } else { 0x00 };
        v
    }

    /// Write KEY1 register (0xff4d)
    pub fn write_speed_switch(&mut self, value: u8) {
        self.speed_switch = value & 0x01 != 0;
    }
}
