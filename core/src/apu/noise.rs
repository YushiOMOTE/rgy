use crate::hardware::Stream;

use super::{length_counter::LengthCounter, util::Envelop};

use bitfield_struct::bitfield;

#[derive(Debug, Clone)]
pub struct Noise {
    power: bool,

    nr41: Nr41,
    nr42: Nr42,
    nr43: Nr43,
    nr44: Nr44,

    length_counter: LengthCounter,

    dac: bool,
}

#[bitfield(u8)]
struct Nr41 {
    #[bits(6)]
    length: usize,
    #[bits(2)]
    _unused: u8,
}

#[bitfield(u8)]
struct Nr42 {
    #[bits(3)]
    count: usize,
    increase: bool,
    #[bits(4)]
    init: usize,
}

#[bitfield(u8)]
struct Nr43 {
    #[bits(3)]
    div_freq: usize,
    step: bool,
    #[bits(4)]
    shift_freq: usize,
}

#[bitfield(u8)]
struct Nr44 {
    #[bits(6)]
    _unused: u8,
    enable_length: bool,
    trigger: bool,
}

impl Noise {
    pub fn new() -> Self {
        Self {
            power: false,

            nr41: Nr41::default(),
            nr42: Nr42::default(),
            nr43: Nr43::default(),
            nr44: Nr44::default(),

            length_counter: LengthCounter::type64(),

            dac: false,
        }
    }

    /// Read NR41 register (0xff20)
    pub fn read_len(&self) -> u8 {
        // Write-only?
        0xff
    }

    /// Write NR41 register (0xff20)
    pub fn write_len(&mut self, value: u8) {
        self.nr41 = Nr41::from_bits(value);

        self.length_counter.load(self.nr41.length());
    }

    /// Read NR42 register (0xff21)
    pub fn read_envelop(&self) -> u8 {
        self.nr42.into_bits()
    }

    /// Write NR42 register (0xff21)
    pub fn write_envelop(&mut self, value: u8) {
        if !self.power {
            return;
        }

        self.nr42 = Nr42::from_bits(value);

        self.dac = self.nr42.init() > 0 || self.nr42.increase();
        if !self.dac {
            self.length_counter.deactivate();
        }
    }

    /// Read NR43 register (0xff22)
    pub fn read_poly_counter(&self) -> u8 {
        self.nr42.into_bits()
    }

    /// Write NR43 register (0xff22)
    pub fn write_poly_counter(&mut self, value: u8) {
        if !self.power {
            return;
        }

        self.nr43 = Nr43::from_bits(value);
    }

    /// Read NR44 register (0xff23)
    pub fn read_select(&self) -> u8 {
        self.nr44.into_bits() | 0xbf
    }

    /// Write NR44 register (0xff23)
    pub fn write_select(&mut self, value: u8) -> bool {
        if !self.power {
            return false;
        }

        self.nr44 = Nr44::from_bits(value);

        self.length_counter
            .update(self.nr44.trigger(), self.nr44.enable_length());

        self.nr44.trigger()
    }

    /// Create stream from the current data
    pub fn create_stream(&self) -> NoiseStream {
        NoiseStream::new(self.clone())
    }

    pub fn step(&mut self, cycles: usize) {
        self.length_counter.step(cycles);
    }

    pub fn is_active(&self) -> bool {
        self.length_counter.is_active() && self.dac
    }

    pub fn power_on(&mut self) {
        self.power = true;

        self.length_counter.power_on();
    }

    pub fn power_off(&mut self) {
        self.power = false;

        self.nr41 = Nr41::default();
        self.nr42 = Nr42::default();
        self.nr43 = Nr43::default();
        self.nr44 = Nr44::default();

        self.length_counter.power_off();

        self.dac = false;
    }
}

pub struct NoiseStream {
    noise: Noise,
    env: Envelop,
    counter: LengthCounter,
    wave: RandomWave,
}

impl NoiseStream {
    pub fn new(noise: Noise) -> Self {
        let mut env = Envelop::new();
        env.update(noise.nr42.init(), noise.nr42.count(), noise.nr42.increase());

        let counter = noise.length_counter.clone();
        let wave = RandomWave::new(noise.nr43.step());

        Self {
            noise,
            env,
            counter,
            wave,
        }
    }
}

impl Stream for NoiseStream {
    fn max(&self) -> u16 {
        unreachable!()
    }

    fn next(&mut self, rate: u32) -> u16 {
        let rate = rate as usize;

        self.counter.step_with_rate(rate);

        if !self.counter.is_active() {
            return 0;
        }

        // Envelop
        self.env.step_with_rate(rate, 1);

        let amp = self.env.amp();

        // Noise: 524288 Hz / r / 2 ^ (s+1)
        let r = self.noise.nr43.div_freq();
        let s = self.noise.nr43.shift_freq() as u32;
        let freq = if r == 0 {
            // For r = 0, assume r = 0.5 instead
            524288 * 5 / 10 / 2usize.pow(s + 1)
        } else {
            524288 / self.noise.nr43.div_freq() / 2usize.pow(s + 1)
        };

        if self.wave.high(rate, freq) {
            amp as u16
        } else {
            0
        }
    }

    fn on(&self) -> bool {
        self.counter.is_active()
    }
}

struct Lfsr {
    value: u16,
    short: bool,
}

impl Lfsr {
    fn new(short: bool) -> Self {
        Self {
            value: 0xdead,
            short,
        }
    }

    fn high(&self) -> bool {
        self.value & 1 == 0
    }

    fn update(&mut self) {
        if self.short {
            self.value &= 0xff;
            let bit = (self.value & 0x0001)
                ^ ((self.value & 0x0004) >> 2)
                ^ ((self.value & 0x0008) >> 3)
                ^ ((self.value & 0x0010) >> 5);
            self.value = (self.value >> 1) | (bit << 7);
        } else {
            let bit = (self.value & 0x0001)
                ^ ((self.value & 0x0004) >> 2)
                ^ ((self.value & 0x0008) >> 3)
                ^ ((self.value & 0x0020) >> 5);
            self.value = (self.value >> 1) | (bit << 15);
        }
    }
}

struct RandomWave {
    lfsr: Lfsr,
    clock: usize,
}

impl RandomWave {
    fn new(short: bool) -> Self {
        Self {
            lfsr: Lfsr::new(short),
            clock: 0,
        }
    }

    fn high(&mut self, rate: usize, freq: usize) -> bool {
        self.clock += freq;

        if self.clock >= rate {
            self.clock -= rate;
            self.lfsr.update()
        }

        self.lfsr.high()
    }
}
