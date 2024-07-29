#[derive(Debug, Clone)]
pub struct Dac {
    power: bool,
    digital_amp: usize,
    analog_amp: isize,
}

impl Dac {
    pub fn new() -> Self {
        Self {
            power: false,
            digital_amp: 0,
            analog_amp: 0,
        }
    }

    pub fn write(&mut self, amp: usize) {
        if !self.power {
            return;
        }

        assert!(amp < 16);

        self.digital_amp = amp;

        // [0, 15] digital amp is mapped to [-8, 8]
        self.analog_amp = match amp {
            0..=7 => amp as isize - 8,
            8..=15 => amp as isize - 7,
            _ => unreachable!(),
        };
    }

    pub fn amp(&self) -> isize {
        self.analog_amp
    }

    pub fn pcm(&self) -> usize {
        self.digital_amp
    }

    pub fn on(&self) -> bool {
        self.power
    }

    pub fn power_on(&mut self) {
        self.power = true;
    }

    pub fn power_off(&mut self) {
        self.power = false;
        self.analog_amp = 0;
    }
}
