use log::*;

use crate::hardware::Stream;

use super::{
    clock_divider::ClockDivider,
    timer::Timer,
    util::{Counter, Envelop, WaveIndex},
};

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
            sweep.update_time_shift(self.sweep_time, self.sweep_sub, self.sweep_shift);
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
            if let Some(new_freq) = sweep.step(cycles) {
                self.freq = new_freq;
            }
        }
        self.counter.step(cycles);
    }

    pub fn is_active(&self) -> bool {
        let sweep_disabling_channel = if let Some(sweep) = self.sweep.as_ref() {
            sweep.disabling_channel()
        } else {
            false
        };

        self.counter.is_active() && self.dac && !sweep_disabling_channel
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
    disabling_channel: bool,
    divider: ClockDivider,
    freq: usize,
    timer: Timer,
    subtract: bool,
    subtracted: bool,
    period: usize,
    shift: usize,
}

impl Sweep {
    fn new() -> Self {
        Self {
            enable: false,
            divider: ClockDivider::new(4_194_304, 128),
            freq: 0,
            timer: Timer::new(),
            subtract: false,
            period: 0,
            shift: 0,
            subtracted: false,
            disabling_channel: false,
        }
    }

    fn trigger(&mut self, freq: usize, period: usize, subtract: bool, shift: usize) {
        self.freq = freq;
        self.enable = period > 0 || shift > 0;
        self.disabling_channel = false;
        self.period = period;
        self.shift = shift;
        self.subtract = subtract;
        self.subtracted = false;

        self.reload_timer();

        debug!("trigger: {:x?}", self);

        if self.shift > 0 {
            // If shift is non-zero, calucation and overflow checks again on trigger
            // discarding the new frequency
            // self.subtracted = self.subtract;
            self.calculate();
        }
    }

    fn update_time_shift(&mut self, period: usize, subtract: bool, shift: usize) {
        debug!("update period/shift {}/{}, {:?}", period, shift, self);

        // Ending subtraction mode after calculation with subtraction disables the channel.
        if self.subtracted && self.subtract && !subtract {
            self.disable();
        }

        self.period = period;
        self.shift = shift;
        self.subtract = subtract;
    }

    fn step_with_rate(&mut self, rate: usize) {
        self.divider.set_source_clock_rate(rate);
        self.step(1);
    }

    fn step(&mut self, cycles: usize) -> Option<usize> {
        if !self.divider.step(cycles) {
            return None;
        }

        if !self.enable {
            return None;
        }

        if !self.timer.tick() {
            return None;
        }
        self.reload_timer();

        // Calculation happens only when period > 0
        if self.period == 0 {
            return None;
        }

        let new_freq = self.calculate();

        // Frequency update happens only when shift > 0
        if self.shift > 0 {
            self.freq = new_freq;

            // Calculation and overflow check actually happens AGAIN
            // but discarding the new frequency
            self.calculate();
        }

        Some(self.freq)
    }

    fn calculate(&mut self) -> usize {
        let new_freq = if self.subtract {
            // This it to detect subtract mode ends after subtraction
            // to disable channel.
            self.subtracted = true;

            self.freq.wrapping_sub(self.freq >> self.shift)
        } else {
            self.freq.wrapping_add(self.freq >> self.shift)
        };

        if new_freq >= 2048 {
            self.disable();
            self.freq
        } else {
            new_freq
        }
    }

    fn reload_timer(&mut self) {
        self.timer
            .set_interval(if self.period == 0 { 8 } else { self.period });
    }

    fn disable(&mut self) {
        self.enable = false;
        self.disabling_channel = true;
    }

    fn disabling_channel(&self) -> bool {
        self.disabling_channel
    }

    fn freq(&self) -> usize {
        self.freq
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
