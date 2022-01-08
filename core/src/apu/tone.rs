use log::*;

use crate::hardware::Stream;

use super::util::{Counter, Envelop, WaveIndex};

#[derive(Debug, Clone)]
pub struct Tone {
    sweep: Option<Sweep>,
    sweep_raw: u8,
    sweep_time: usize,
    sweep_sub: bool,
    sweep_shift: usize,
    wave_raw: u8,
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
            sweep: if with_sweep { Some(Sweep::new()) } else { None },
            sweep_raw: 0,
            sweep_time: 0,
            sweep_sub: false,
            sweep_shift: 0,
            wave_raw: 0,
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
        self.sweep_raw | 0x80
    }

    /// Write NR10 register (0xff10)
    pub fn write_sweep(&mut self, value: u8) {
        debug!("write NR10: {:02x}", value);
        self.sweep_raw = value;
        self.sweep_time = ((value >> 4) & 0x7) as usize;
        self.sweep_sub = value & 0x08 != 0;
        self.sweep_shift = (value & 0x07) as usize;
        if let Some(sweep) = &mut self.sweep {
            sweep.update_time_shift(self.sweep_time, self.sweep_shift);
        }
    }

    /// Read NR11/NR21 register (0xff11/0xff16)
    pub fn read_wave(&self) -> u8 {
        self.wave_raw | 0x3f
    }

    /// Write NR11/NR21 register (0xff11/0xff16)
    pub fn write_wave(&mut self, value: u8) {
        self.wave_raw = value;
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
        if let Some(sweep) = self.sweep.as_mut() {
            sweep.trigger(self.freq, self.sweep_time, self.sweep_sub, self.sweep_shift);
        }
        trigger
    }

    /// Create stream from the current data
    pub fn create_stream(&self) -> ToneStream {
        ToneStream::new(self.clone())
    }

    pub fn clear(&mut self) {
        core::mem::swap(self, &mut Tone::new(self.sweep.is_some()));
    }

    pub fn step(&mut self, cycles: usize) {
        if let Some(sweep) = self.sweep.as_mut() {
            sweep.step(cycles);
        }
        self.counter.step(cycles);
    }

    pub fn is_active(&self) -> bool {
        let sweep_active = if let Some(sweep) = self.sweep.as_ref() {
            sweep.is_active()
        } else {
            true
        };

        self.counter.is_active() && self.dac && sweep_active
    }

    fn real_freq(&self) -> usize {
        let raw_freq = match &self.sweep {
            Some(sweep) => sweep.freq(),
            None => self.freq,
        };
        131072 / (2048 - raw_freq)
    }
}

#[derive(Clone, Debug)]
struct Sweep {
    enable: bool,
    active: bool,
    rate: usize,
    cycles: usize,
    freq: usize,
    shadow_freq: usize,
    count: usize,
    timer: usize,
    subtract: bool,
    period: usize,
    shift: usize,
}

impl Sweep {
    fn new() -> Self {
        Self {
            enable: false,
            active: false,
            rate: 4_194_304,
            cycles: 0,
            freq: 0,
            shadow_freq: 0,
            count: 0,
            timer: 0,
            subtract: false,
            period: 0,
            shift: 0,
        }
    }

    fn trigger(&mut self, freq: usize, period: usize, subtract: bool, shift: usize) {
        self.freq = freq;
        self.shadow_freq = freq;
        self.timer = if period == 0 { 8 } else { period };
        self.enable = period > 0 || shift > 0;
        self.period = period;
        self.shift = shift;
        self.subtract = subtract;
        self.active = true;

        debug!("trigger: {:?}", self);

        if self.shift > 0 && self.overflow() {
            debug!("disabled immediately: {:?}", self);
            self.disable();
        }
    }

    fn update_time_shift(&mut self, period: usize, shift: usize) {
        debug!("update period/shift {}/{}, {:?}", period, shift, self);
        self.period = period;
        self.shift = shift;
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn step_with_rate(&mut self, rate: usize) {
        self.rate = rate;
        self.step(1);
    }

    fn step(&mut self, cycles: usize) {
        self.cycles += cycles;
        if self.cycles < self.interval() {
            return;
        }
        self.cycles -= self.interval();

        if !self.enable {
            return;
        }

        self.count += 1;
        if self.count < self.timer {
            return;
        }
        self.count -= self.timer;

        self.tick();
    }

    fn tick(&mut self) {
        if !self.enable || self.period == 0 {
            return;
        }

        if self.overflow() {
            self.disable();
            return;
        }

        let new_freq = self.calculate();

        if self.shift > 0 {
            debug!("update freq={}", self.freq);
            self.freq = new_freq;
            self.shadow_freq = new_freq;
            if self.overflow() {
                self.disable();
            }
        }
    }

    fn calculate(&self) -> usize {
        let p = self.shadow_freq >> self.shift;

        let new_freq = if self.subtract {
            self.shadow_freq.saturating_sub(p)
        } else {
            self.shadow_freq.saturating_add(p)
        };

        new_freq
    }

    fn overflow(&self) -> bool {
        self.calculate() >= 2048
    }

    fn disable(&mut self) {
        if self.enable {
            self.active = false;
        }
        self.enable = false;
    }

    fn freq(&self) -> usize {
        self.freq
    }

    fn interval(&self) -> usize {
        self.rate / 128
    }
}

pub struct ToneStream {
    tone: Tone,
    env: Envelop,
    index: WaveIndex,
}

impl ToneStream {
    fn new(tone: Tone) -> Self {
        let env = Envelop::new(tone.env_init, tone.env_count, tone.env_inc);

        Self {
            tone,
            env,
            index: WaveIndex::new(),
        }
    }

    fn step(&mut self, rate: usize) {
        self.tone.counter.step_with_rate(rate);
        if let Some(sweep) = &mut self.tone.sweep {
            sweep.step_with_rate(rate);
        }
    }
}

impl Stream for ToneStream {
    fn max(&self) -> u16 {
        unreachable!()
    }

    fn next(&mut self, rate: u32) -> u16 {
        let rate = rate as usize;

        self.step(rate);

        if !self.tone.counter.is_active() {
            return 0;
        }

        // Envelop
        let amp = self.env.amp(rate);

        // Sweep
        let freq = self.tone.real_freq();

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
        self.tone.counter.is_active()
    }
}
