use log::*;

use bitfield_struct::bitfield;

use super::{dac::Dac, length_counter::LengthCounter};
use crate::clock::{ClockDivider, Timer};

const RAM_SIZE: usize = 16;
const WAVE_SIZE: usize = RAM_SIZE * 2;
const WAVE_FREQ_HZ: usize = 2_097_152;

#[derive(Debug, Clone)]
pub struct Wave {
    power: bool,
    length_counter: LengthCounter,
    freq: Freq,
    ram: [u8; RAM_SIZE],
    index: Index,
    nr30: Nr30,
    nr31: Nr31,
    nr32: Nr32,
    nr33: Nr33,
    nr34: Nr34,
    divider: ClockDivider,
    timer: Timer,
    last_sample: u8,
    first_fetch: bool,
    dac: Dac,
}

#[bitfield(u8)]
struct Nr30 {
    #[bits(7)]
    _unused: u8,
    dac: bool,
}

#[bitfield(u8)]
struct Nr31 {
    #[bits(8)]
    length: usize,
}

#[bitfield(u8)]
struct Nr32 {
    #[bits(5)]
    _unused1: u8,
    #[bits(2)]
    amp_shift: usize,
    #[bits(1)]
    _unused2: u8,
}

#[bitfield(u8)]
struct Nr33 {
    #[bits(8)]
    freq_low: usize,
}

#[bitfield(u8)]
struct Nr34 {
    #[bits(3)]
    freq_high: usize,
    #[bits(3)]
    _unused: u8,
    length_enable: bool,
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
    fn value(&self) -> usize {
        self.into_bits() as usize
    }
}

trait ReadWaveRam {
    fn read(&self, index: Index) -> u8;
}

impl ReadWaveRam for [u8; RAM_SIZE] {
    fn read(&self, index: Index) -> u8 {
        if index.0 % 2 == 0 {
            self[index.0 / 2] >> 4
        } else {
            self[index.0 / 2] & 0xf
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Index(usize);

impl Index {
    fn next(&self) -> Self {
        Self((self.0 + 1) % WAVE_SIZE)
    }

    fn byte_index(&self) -> usize {
        self.0 / 2
    }
}

impl Wave {
    pub fn new() -> Self {
        Self {
            power: false,
            length_counter: LengthCounter::type256(),
            freq: Freq::default(),
            ram: [0; RAM_SIZE],
            index: Index(0),
            nr30: Nr30::default(),
            nr31: Nr31::default(),
            nr32: Nr32::default(),
            nr33: Nr33::default(),
            nr34: Nr34::default(),
            divider: ClockDivider::new(WAVE_FREQ_HZ),
            timer: Timer::enabled(),
            last_sample: 0,
            dac: Dac::new(),
            first_fetch: false,
        }
    }

    /// Read NR30 register (0xff1a)
    pub fn read_enable(&self) -> u8 {
        self.nr30.into_bits() | 0x7f
    }

    /// Write NR30 register (0xff1a)
    pub fn write_enable(&mut self, value: u8) {
        if !self.power {
            return;
        }

        self.nr30 = Nr30::from_bits(value);

        if self.nr30.dac() {
            self.dac.power_on();
        } else {
            self.dac.power_off();
            self.length_counter.deactivate();
        }
    }

    /// Read NR31 register (0xff1b)
    pub fn read_len(&self) -> u8 {
        // Write-only
        0xff
    }

    /// Write NR31 register (0xff1b)
    pub fn write_len(&mut self, value: u8) {
        self.nr31 = Nr31::from_bits(value);

        self.length_counter.load(self.nr31.length());
    }

    /// Read NR32 register (0xff1c)
    pub fn read_amp(&self) -> u8 {
        self.nr32.into_bits() | 0x9f
    }

    /// Write NR32 register (0xff1c)
    pub fn write_amp(&mut self, value: u8) {
        if !self.power {
            return;
        }

        debug!("Wave amp shift: {:02x}", value);
        self.nr32 = Nr32::from_bits(value);
    }

    /// Read NR33 register (0xff1d)
    pub fn read_freq_low(&self) -> u8 {
        // Write only
        0xff
    }

    /// Write NR33 register (0xff1d)
    pub fn write_freq_low(&mut self, value: u8) {
        if !self.power {
            return;
        }

        self.nr33 = Nr33::from_bits(value);
        self.freq = self.freq.with_low(self.nr33.freq_low());
    }

    /// Read NR34 register (0xff1e)
    pub fn read_freq_high(&self) -> u8 {
        // Mask write-only bits
        self.nr34.into_bits() | 0xbf
    }

    /// Write NR34 register (0xff1e)
    pub fn write_freq_high(&mut self, value: u8) -> bool {
        if !self.power {
            return false;
        }

        self.nr34 = Nr34::from_bits(value);

        self.freq = self.freq.with_high(self.nr34.freq_high());

        let trigger = value & 0x80 != 0;
        let length_enable = value & 0x40 != 0;
        let retrigger = self.length_counter.is_active();
        self.length_counter.update(trigger, length_enable);
        if self.dac.on() && trigger {
            if retrigger && !self.first_fetch {
                // Advance one tick on retrigger
                self.timer.tick();
                self.alter_waveram();
            }

            self.load_initial_timer();

            self.index = Index(0);
            self.first_fetch = true;
        }
        trigger
    }

    /// Read wave pattern buffer
    pub fn read_wave_buf(&self, offset: u16) -> u8 {
        let value = match self.adjust_waveram_index(offset - 0xff30) {
            Some(index) => self.ram[index],
            None => 0xff,
        };
        value
    }

    /// Write wave pattern buffer
    pub fn write_wave_buf(&mut self, offset: u16, value: u8) {
        if let Some(index) = self.adjust_waveram_index(offset - 0xff30) {
            self.ram[index] = value;
        }
    }

    pub fn step(&mut self, cycles: usize) {
        self.length_counter.step(cycles);

        let times = self.divider.step(cycles);

        for _ in 0..times {
            self.update();
        }
    }

    pub fn step_with_rate(&mut self, rate: usize) {
        self.length_counter.step_with_rate(rate);

        self.divider.set_source_clock_rate(rate);

        let times = self.divider.step(1);

        for _ in 0..times {
            self.update();
        }
    }

    fn adjust_waveram_index(&self, cpu_index: u16) -> Option<usize> {
        let apu_index = self.index.byte_index();

        if self.is_active() {
            if !self.first_fetch && self.is_sampling() {
                Some(apu_index)
            } else {
                None
            }
        } else {
            Some(cpu_index as usize)
        }
    }

    fn alter_waveram(&mut self) {
        if !self.is_sampling() {
            return;
        }

        let byte_offset = self.index.next().byte_index();

        match byte_offset {
            0..=3 => {
                self.ram[0] = self.ram[byte_offset];
            }
            4..=15 => {
                for i in 0..4 {
                    self.ram[i] = self.ram[(byte_offset / 4) * 4 + i];
                }
            }
            _ => unreachable!(),
        }
    }

    fn is_sampling(&self) -> bool {
        // Timer tick is 2 cycles. 2 ticks means 4 cycles.
        // Having this in CPU instruction means the instraction is happening
        // at the same time that APU is reading a sample from the Wave RAM.
        self.timer.remaining() == 2
    }

    fn update(&mut self) {
        if !self.is_active() {
            return;
        }
        if !self.timer.tick() {
            return;
        }

        self.reload_timer();

        let sample = if self.first_fetch {
            self.first_fetch = false;
            self.last_sample
        } else {
            let new_amp = self.ram.read(self.index);
            self.last_sample = new_amp;
            new_amp
        };

        self.index = self.index.next();

        self.dac.write(match self.nr32.amp_shift() {
            0 => 0,
            1 => sample,
            2 => sample >> 1,
            3 => sample >> 2,
            _ => unreachable!(),
        } as usize)
    }

    fn load_initial_timer(&mut self) {
        self.timer.reset();
        self.timer.set_interval(self.timer_interval() + 3);
    }

    fn reload_timer(&mut self) {
        self.timer.reset();
        self.timer.set_interval(self.timer_interval());
    }

    fn timer_interval(&self) -> usize {
        2048 - self.freq.value()
    }

    pub fn is_active(&self) -> bool {
        self.length_counter.is_active() && self.dac.on()
    }

    pub fn power_on(&mut self) {
        self.power = true;
        self.length_counter.power_on();
    }

    pub fn power_off(&mut self) {
        self.power = false;

        self.length_counter.power_off();

        self.index = Index(0);
        self.divider.reset();
        self.timer.reset();
        self.last_sample = 0;
        self.first_fetch = false;
        self.dac.power_off();

        self.nr30 = Nr30::default();
        self.nr31 = Nr31::default();
        self.nr32 = Nr32::default();
        self.nr33 = Nr33::default();
        self.nr34 = Nr34::default();

        self.freq = Freq::default();
    }

    pub fn amp(&self) -> isize {
        self.dac.amp()
    }
}
