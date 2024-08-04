use bitfield_struct::bitfield;

pub struct Mixer {
    power: bool,
    nr50: Nr50,
    nr51: Nr51,
}

#[bitfield(u8)]
struct Nr50 {
    #[bits(3)]
    right_volume: usize,
    vin_right_enable: bool,
    #[bits(3)]
    left_volume: usize,
    vin_left_enable: bool,
}

#[bitfield(u8)]
struct Nr51 {
    #[bits(1)]
    ch1_right: isize,
    #[bits(1)]
    ch2_right: isize,
    #[bits(1)]
    ch3_right: isize,
    #[bits(1)]
    ch4_right: isize,
    #[bits(1)]
    ch1_left: isize,
    #[bits(1)]
    ch2_left: isize,
    #[bits(1)]
    ch3_left: isize,
    #[bits(1)]
    ch4_left: isize,
}

impl Mixer {
    pub fn new() -> Self {
        Self {
            power: false,
            nr50: Nr50::default(),
            nr51: Nr51::default(),
        }
    }

    /// Read NR50 register (0xff24)
    pub fn read_ctrl(&self) -> u8 {
        self.nr50.into_bits()
    }

    /// Write NR50 register (0xff24)
    pub fn write_ctrl(&mut self, value: u8) {
        if !self.power {
            return;
        }

        self.nr50 = Nr50::from_bits(value);
    }

    /// Read NR51 register (0xff25)
    pub fn read_so_mask(&self) -> u8 {
        self.nr51.into_bits()
    }

    /// Write NR51 register (0xff25)
    pub fn write_so_mask(&mut self, value: u8) {
        if !self.power {
            return;
        }

        self.nr51 = Nr51::from_bits(value);
    }

    pub fn builder(&self) -> AmpBuilder {
        AmpBuilder {
            left: 0,
            right: 0,
            nr50: self.nr50.clone(),
            nr51: self.nr51.clone(),
        }
    }

    pub fn power_on(&mut self) {
        self.power = true;
    }

    pub fn power_off(&mut self) {
        self.power = false;

        self.nr50 = Nr50::default();
        self.nr51 = Nr51::default();
    }
}

pub struct AmpBuilder {
    left: isize,
    right: isize,
    nr50: Nr50,
    nr51: Nr51,
}

impl AmpBuilder {
    pub fn tone1(mut self, amp: isize) -> Self {
        self.left += amp * self.nr51.ch1_left();
        self.right += amp * self.nr51.ch1_right();
        self
    }

    pub fn tone2(mut self, amp: isize) -> Self {
        self.left += amp * self.nr51.ch2_left();
        self.right += amp * self.nr51.ch2_right();
        self
    }

    pub fn wave(mut self, amp: isize) -> Self {
        self.left += amp * self.nr51.ch3_left();
        self.right += amp * self.nr51.ch3_right();
        self
    }

    pub fn noise(mut self, amp: isize) -> Self {
        self.left += amp * self.nr51.ch4_left();
        self.right += amp * self.nr51.ch4_right();
        self
    }

    pub fn build(self) -> (isize, isize) {
        (
            self.left * (self.nr50.left_volume() + 1) as isize,
            self.right * (self.nr50.right_volume() + 1) as isize,
        )
    }
}
