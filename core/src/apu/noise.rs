use super::util::{Counter, Envelop};
use crate::hardware::Stream;
use log::*;

#[derive(Debug, Clone)]
pub struct Noise {
    sound_len: usize,

    envelop: u8,
    env_init: usize,
    env_inc: bool,
    env_count: usize,

    poly_counter: u8,
    shift_freq: usize,
    step: bool,
    div_freq: usize,

    select: u8,
    counter: bool,
    freq: usize,
}

impl Noise {
    pub fn new() -> Self {
        Self {
            sound_len: 0,

            envelop: 0,
            env_init: 0,
            env_inc: false,
            env_count: 0,

            poly_counter: 0,
            shift_freq: 0,
            step: false,
            div_freq: 0,

            select: 0,
            counter: false,
            freq: 0,
        }
    }

    /// Read NR41 register (0xff20)
    pub fn read_len(&self) -> u8 {
        // Write-only?
        0xff
    }

    /// Write NR41 register (0xff20)
    pub fn write_len(&mut self, value: u8) {
        self.sound_len = (value & 0x1f) as usize;
        debug!("Noise: length = {}", self.sound_len);
    }

    /// Read NR42 register (0xff21)
    pub fn read_envelop(&self) -> u8 {
        self.envelop
    }

    /// Write NR42 register (0xff21)
    pub fn write_envelop(&mut self, value: u8) {
        self.envelop = value;
        self.env_init = (value >> 4) as usize;
        self.env_inc = value & 0x08 != 0;
        self.env_count = (value & 0x7) as usize;
    }

    /// Read NR43 register (0xff22)
    pub fn read_poly_counter(&self) -> u8 {
        self.poly_counter
    }

    /// Write NR43 register (0xff22)
    pub fn write_poly_counter(&mut self, value: u8) {
        self.poly_counter = value;
        self.shift_freq = (value >> 4) as usize;
        self.step = value & 0x08 != 0;
        self.div_freq = (value & 0x7) as usize;
    }

    /// Read NR44 register (0xff23)
    pub fn read_select(&self) -> u8 {
        self.select | 0xbf
    }

    /// Write NR44 register (0xff23)
    pub fn write_select(&mut self, value: u8) -> bool {
        self.select = value;
        self.counter = value & 0x40 != 0;
        value & 0x80 != 0
    }

    /// Create stream from the current data
    pub fn create_stream(&self) -> NoiseStream {
        NoiseStream::new(self.clone())
    }

    /// Create counter
    pub fn create_counter(&self) -> Counter {
        Counter::new(self.counter, self.sound_len, 64)
    }

    pub fn clear(&mut self) {
        core::mem::swap(self, &mut Noise::new());
    }
}

pub struct NoiseStream {
    noise: Noise,
    env: Envelop,
    counter: Counter,
    wave: RandomWave,
}

impl NoiseStream {
    pub fn new(noise: Noise) -> Self {
        let env = Envelop::new(noise.env_init, noise.env_count, noise.env_inc);
        let counter = noise.create_counter();
        let wave = RandomWave::new(noise.step);

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

        self.counter.proceed(rate);

        if self.counter.is_expired() {
            return 0;
        }

        // Envelop
        let amp = self.env.amp(rate);

        // Noise: 524288 Hz / r / 2 ^ (s+1)
        let r = self.noise.div_freq;
        let s = self.noise.shift_freq as u32;
        let freq = if r == 0 {
            // For r = 0, assume r = 0.5 instead
            524288 * 5 / 10 / 2usize.pow(s + 1)
        } else {
            524288 / self.noise.div_freq / 2usize.pow(s + 1)
        };

        if self.wave.high(rate, freq) {
            amp as u16
        } else {
            0
        }
    }

    fn on(&self) -> bool {
        !self.counter.is_expired()
    }
}

struct LFSR {
    value: u16,
    short: bool,
}

impl LFSR {
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
    lfsr: LFSR,
    clock: usize,
}

impl RandomWave {
    fn new(short: bool) -> Self {
        Self {
            lfsr: LFSR::new(short),
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
