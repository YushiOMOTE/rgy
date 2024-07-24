use log::*;

use crate::hardware::Stream;

use super::{length_counter::LengthCounter, sweep::Sweep, util::Envelop, wave_buf::WaveIndex};

use bitfield_struct::bitfield;

#[derive(Debug, Clone)]
pub struct Tone {
    power: bool,
    sweep: Option<Sweep>,
    nr10: Nr10,
    nr11: Nr11,
    nr12: Nr12,
    nr13: Nr13,
    nr14: Nr14,
    length_counter: LengthCounter,
    freq: Freq,
    dac: bool,
}

#[bitfield(u8)]
struct Nr10 {
    #[bits(3)]
    shift: usize,
    subtract: bool,
    #[bits(3)]
    freq: usize,
    #[bits(1)]
    _unused: u8,
}

#[bitfield(u8)]
struct Nr11 {
    #[bits(6)]
    length: usize,
    #[bits(2)]
    wave_duty: usize,
}

#[bitfield(u8)]
struct Nr12 {
    #[bits(3)]
    count: usize,
    increase: bool,
    #[bits(4)]
    init: usize,
}

#[bitfield(u8)]
struct Nr13 {
    #[bits(8)]
    freq_low: usize,
}

#[bitfield(u8)]
struct Nr14 {
    #[bits(3)]
    freq_high: usize,
    #[bits(3)]
    _unused: u8,
    enable_length: bool,
    trigger: bool,
}

#[bitfield(u16)]
struct Freq {
    #[bits(8)]
    low: usize,
    #[bits(3)]
    high: usize,
    #[bits(5)]
    _unused: u16,
}

impl Freq {
    fn from_value(value: usize) -> Self {
        Self::from_bits(value as u16)
    }

    fn value(&self) -> usize {
        self.into_bits() as usize
    }

    fn hz(&self) -> usize {
        131072 / (2048 - self.value())
    }
}

impl Tone {
    pub fn new(with_sweep: bool) -> Self {
        Self {
            power: false,
            sweep: if with_sweep { Some(Sweep::new()) } else { None },
            nr10: Nr10::default(),
            nr11: Nr11::default(),
            nr12: Nr12::default(),
            nr13: Nr13::default(),
            nr14: Nr14::default(),
            length_counter: LengthCounter::type64(),
            freq: Freq::default(),
            dac: false,
        }
    }

    /// Read NR10 register (0xff10)
    pub fn read_sweep(&self) -> u8 {
        self.nr10.into_bits() | 0x80
    }

    /// Write NR10 register (0xff10)
    pub fn write_sweep(&mut self, value: u8) {
        if !self.power {
            return;
        }

        info!("write NR10: {:02x}", value);
        self.nr10 = Nr10::from_bits(value);
        if let Some(sweep) = &mut self.sweep {
            sweep.update_params(self.nr10.freq(), self.nr10.subtract(), self.nr10.shift());
        }
    }

    /// Read NR11/NR21 register (0xff11/0xff16)
    pub fn read_wave(&self) -> u8 {
        self.nr11.into_bits() | 0x3f
    }

    /// Write NR11/NR21 register (0xff11/0xff16)
    pub fn write_wave(&mut self, value: u8) {
        let reg = Nr11::from_bits(value);

        self.length_counter.load(reg.length());

        if !self.power {
            return;
        }

        self.nr11 = reg;
    }

    /// Read NR12/NR22 register (0xff12/0xff17)
    pub fn read_envelop(&self) -> u8 {
        self.nr12.into_bits()
    }

    /// Write NR12/NR22 register (0xff12/0xff17)
    pub fn write_envelop(&mut self, value: u8) {
        if !self.power {
            return;
        }

        self.nr12 = Nr12::from_bits(value);

        self.dac = self.nr12.init() > 0 || self.nr12.increase();
        if !self.dac {
            self.length_counter.deactivate();
        }
    }

    /// Read NR13/NR23 register (0xff13/0xff18)
    pub fn read_freq_low(&self) -> u8 {
        // Write only
        0xff
    }

    /// Write NR13/NR23 register (0xff13/0xff18)
    pub fn write_freq_low(&mut self, value: u8) {
        if !self.power {
            return;
        }

        self.nr13 = Nr13::from_bits(value);
        self.freq = self.freq.with_low(self.nr13.freq_low());
    }

    /// Read NR14/NR24 register (0xff14/0xff19)
    pub fn read_freq_high(&self) -> u8 {
        // Fix write-only bits to high
        self.nr14.into_bits() | 0xbf
    }

    /// Write NR14/NR24 register (0xff14/0xff19)
    pub fn write_freq_high(&mut self, value: u8) -> bool {
        if !self.power {
            return false;
        }

        self.nr14 = Nr14::from_bits(value);

        self.freq = self.freq.with_high(self.nr14.freq_high());

        self.length_counter
            .update(self.nr14.trigger(), self.nr14.enable_length());

        if let Some(sweep) = self.sweep.as_mut() {
            sweep.trigger(
                self.freq.value(),
                self.nr10.freq(),
                self.nr10.subtract(),
                self.nr10.shift(),
            );
        }

        self.nr14.trigger()
    }

    /// Create stream from the current data
    pub fn create_stream(&self) -> ToneStream {
        ToneStream::new(self.clone())
    }

    pub fn power_on(&mut self) {
        self.power = true;

        if let Some(sweep) = self.sweep.as_mut() {
            sweep.power_on();
        }
        self.length_counter.power_on();
    }

    pub fn power_off(&mut self) {
        self.power = false;

        if let Some(sweep) = self.sweep.as_mut() {
            sweep.power_off();
        }

        self.nr10 = Nr10::from_bits(0);
        self.nr11 = Nr11::from_bits(0);
        self.nr12 = Nr12::from_bits(0);
        self.nr13 = Nr13::from_bits(0);
        self.nr14 = Nr14::from_bits(0);
        self.freq = Freq::from_bits(0);

        self.length_counter.power_off();

        self.dac = false;
    }

    pub fn step(&mut self, cycles: usize) {
        if let Some(sweep) = self.sweep.as_mut() {
            if let Some(new_freq) = sweep.step(cycles) {
                self.freq = Freq::from_value(new_freq);
            }
        }
        self.length_counter.step(cycles);
    }

    pub fn is_active(&self) -> bool {
        let sweep_disabling_channel = if let Some(sweep) = self.sweep.as_ref() {
            sweep.disabling_channel()
        } else {
            false
        };

        self.length_counter.is_active() && self.dac && !sweep_disabling_channel
    }

    fn freq(&self) -> Freq {
        match &self.sweep {
            Some(sweep) => Freq::from_value(sweep.freq()),
            None => self.freq,
        }
    }
}

pub struct ToneStream {
    tone: Tone,
    env: Envelop,
    index: WaveIndex,
}

impl ToneStream {
    fn new(tone: Tone) -> Self {
        let env = Envelop::new(tone.nr12.init(), tone.nr12.count(), tone.nr12.increase());

        Self {
            tone,
            env,
            index: WaveIndex::new(4_194_304, 8),
        }
    }

    fn step(&mut self, rate: usize) {
        self.tone.length_counter.step_with_rate(rate);
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

        if !self.tone.length_counter.is_active() {
            return 0;
        }

        // Envelop
        let amp = self.env.amp(rate);

        // Sweep
        let freq = self.tone.freq().hz();

        // Square wave generation
        let duty = match self.tone.nr11.wave_duty() {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => 5,
            _ => unreachable!(),
        };

        self.index.set_source_clock_rate(rate);
        self.index.update_index(freq * 8);

        if self.index.index() <= duty {
            0
        } else {
            amp as u16
        }
    }

    fn on(&self) -> bool {
        self.tone.length_counter.is_active()
    }
}
