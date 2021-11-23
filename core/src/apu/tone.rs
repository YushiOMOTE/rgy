use crate::hardware::Stream;

use super::util::{Counter, Envelop, WaveIndex};

#[derive(Debug, Clone)]
pub struct Tone {
    with_sweep: bool,
    sweep: u8,
    sweep_time: usize,
    sweep_sub: bool,
    sweep_shift: usize,
    wave: u8,
    wave_duty: usize,
    envelop: u8,
    env_init: usize,
    env_inc: bool,
    env_count: usize,
    counter: Counter,
    freq: usize,
    freq_high: u8,
    dac: bool,
}

impl Tone {
    pub fn new(with_sweep: bool) -> Self {
        Self {
            with_sweep,
            sweep: 0,
            sweep_time: 0,
            sweep_sub: false,
            sweep_shift: 0,
            wave: 0,
            wave_duty: 0,
            envelop: 0,
            env_init: 0,
            env_inc: false,
            env_count: 0,
            counter: Counter::type64(),
            freq: 0,
            freq_high: 0,
            dac: false,
        }
    }

    /// Read NR10 register (0xff10)
    pub fn read_sweep(&self) -> u8 {
        self.sweep | 0x80
    }

    /// Write NR10 register (0xff10)
    pub fn write_sweep(&mut self, value: u8) {
        self.sweep = value;
        self.sweep_time = ((value >> 4) & 0x7) as usize;
        self.sweep_sub = value & 0x08 != 0;
        self.sweep_shift = (value & 0x07) as usize;
    }

    /// Read NR11/NR21 register (0xff11/0xff16)
    pub fn read_wave(&self) -> u8 {
        self.wave | 0x3f
    }

    /// Write NR11/NR21 register (0xff11/0xff16)
    pub fn write_wave(&mut self, value: u8) {
        self.wave = value;
        self.wave_duty = (value >> 6).into();
        self.counter.load((value & 0x3f) as usize);
    }

    /// Read NR12/NR22 register (0xff12/0xff17)
    pub fn read_envelop(&self) -> u8 {
        self.envelop
    }

    /// Write NR12/NR22 register (0xff12/0xff17)
    pub fn write_envelop(&mut self, value: u8) {
        self.envelop = value;
        self.env_init = (value >> 4) as usize;
        self.env_inc = value & 0x08 != 0;
        self.env_count = (value & 0x7) as usize;
        self.dac = value & 0xf8 != 0;
        if !self.dac {
            self.counter.deactivate();
        }
    }

    /// Read NR13/NR23 register (0xff13/0xff18)
    pub fn read_freq_low(&self) -> u8 {
        // Write only
        0xff
    }

    /// Write NR13/NR23 register (0xff13/0xff18)
    pub fn write_freq_low(&mut self, value: u8) {
        self.freq = (self.freq & !0xff) | value as usize;
    }

    /// Read NR14/NR24 register (0xff14/0xff19)
    pub fn read_freq_high(&self) -> u8 {
        // Fix write-only bits to high
        self.freq_high | 0xbf
    }

    /// Write NR14/NR24 register (0xff14/0xff19)
    pub fn write_freq_high(&mut self, value: u8) -> bool {
        self.freq_high = value;
        self.freq = (self.freq & !0x700) | (((value & 0x7) as usize) << 8);
        let trigger = value & 0x80 != 0;
        let enable = value & 0x40 != 0;
        self.counter.update(trigger, enable);
        trigger
    }

    /// Create stream from the current data
    pub fn create_stream(&self) -> ToneStream {
        ToneStream::new(self.clone(), self.index() == 0)
    }

    pub fn clear(&mut self) {
        core::mem::swap(self, &mut Tone::new(self.with_sweep));
    }

    pub fn step(&mut self, cycles: usize) {
        self.counter.step(cycles);
    }

    pub fn is_active(&self) -> bool {
        self.counter.is_active() && self.dac
    }

    fn index(&self) -> usize {
        if self.with_sweep {
            1
        } else {
            2
        }
    }
}

struct Sweep {
    enable: bool,
    freq: usize,
    time: usize,
    sub: bool,
    shift: usize,
    clock: usize,
}

impl Sweep {
    fn new(enable: bool, freq: usize, time: usize, sub: bool, shift: usize) -> Self {
        Self {
            enable,
            freq,
            time,
            sub,
            shift,
            clock: 0,
        }
    }

    fn freq(&mut self, rate: usize) -> usize {
        if !self.enable || self.time == 0 || self.shift == 0 {
            return self.freq;
        }

        let interval = rate * self.time / 128;

        self.clock += 1;
        if self.clock >= interval {
            self.clock -= interval;

            let p = self.freq / 2usize.pow(self.shift as u32);

            self.freq = if self.sub {
                self.freq.saturating_sub(p)
            } else {
                self.freq.saturating_add(p)
            };

            if self.freq >= 2048 || self.freq == 0 {
                self.enable = false;
                self.freq = 0;
            }
        }

        self.freq
    }
}

pub struct ToneStream {
    tone: Tone,
    sweep: Sweep,
    env: Envelop,
    counter: Counter,
    index: WaveIndex,
}

impl ToneStream {
    fn new(tone: Tone, sweep: bool) -> Self {
        let freq = 131072 / (2048 - tone.freq);
        let sweep = Sweep::new(
            sweep,
            freq,
            tone.sweep_time,
            tone.sweep_sub,
            tone.sweep_shift,
        );
        let env = Envelop::new(tone.env_init, tone.env_count, tone.env_inc);
        let counter = tone.counter.clone();

        Self {
            tone,
            sweep,
            env,
            counter,
            index: WaveIndex::new(),
        }
    }
}

impl Stream for ToneStream {
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
        let amp = self.env.amp(rate);

        // Sweep
        let freq = self.sweep.freq(rate);

        // Square wave generation
        let duty = match self.tone.wave_duty {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => 5,
            _ => unreachable!(),
        };

        let index = self.index.index(rate, freq * 8, 8);
        if index <= duty {
            0
        } else {
            amp as u16
        }
    }

    fn on(&self) -> bool {
        self.counter.is_active()
    }
}
