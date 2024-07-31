use super::{dac::Dac, envelope::Envelope, frame_sequencer::Frame, length_counter::LengthCounter};
use crate::{
    clock::{ClockDivider, Timer},
    cpu::CPU_FREQ_HZ,
};

use bitfield_struct::bitfield;

// LFSR runs at CPU rate.
const NOISE_FREQ_HZ: usize = CPU_FREQ_HZ;

#[derive(Debug, Clone)]
pub struct Noise {
    power: bool,

    nr41: Nr41,
    nr42: Nr42,
    nr43: Nr43,
    nr44: Nr44,

    length_counter: LengthCounter,
    divider: ClockDivider,
    timer: Timer,
    envelope: Envelope,
    lfsr: Lfsr,
    dac: Dac,
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
            divider: ClockDivider::new(NOISE_FREQ_HZ),
            timer: Timer::enabled(),
            envelope: Envelope::new(),
            lfsr: Lfsr::new(),

            dac: Dac::new(),
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

        if self.nr42.init() > 0 || self.nr42.increase() {
            self.dac.power_on();
        } else {
            self.dac.power_off();
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

        if self.nr44.trigger() {
            self.reload_timer();
            self.lfsr.trigger(self.nr43.step());
            self.envelope
                .update(self.nr42.init(), self.nr42.count(), self.nr42.increase());
        }

        self.nr44.trigger()
    }

    pub fn step(&mut self, cycles: usize, frame: Frame) {
        self.length_counter.step(frame);
        self.envelope.step(frame);

        let times = self.divider.step(cycles);

        for _ in 0..times {
            self.update();
        }

        self.write_amp();
    }

    fn update(&mut self) {
        if !self.timer.tick() {
            return;
        }

        self.reload_timer();

        self.lfsr.update();
    }

    fn write_amp(&mut self) {
        self.dac.write(if self.is_active() && self.lfsr.high() {
            self.envelope.amp()
        } else {
            0
        });
    }

    fn reload_timer(&mut self) {
        self.timer.reset();
        self.timer.set_interval(self.timer_interval());
    }

    fn timer_interval(&self) -> usize {
        let divider = self.nr43.div_freq();
        let shift = self.nr43.shift_freq();

        // Divisor code   Divisor
        // -----------------------
        //    0             8
        //    1            16
        //    2            32
        //    3            48
        //    4            64
        //    5            80
        //    6            96
        //    7           112
        let base = if divider == 0 { 8 } else { divider * 16 };

        base << shift
    }

    pub fn is_active(&self) -> bool {
        self.length_counter.is_active() && self.dac.on()
    }

    pub fn power_on(&mut self) {
        self.power = true;

        self.length_counter.power_on();

        self.reload_timer();
        self.divider.reset();
    }

    pub fn power_off(&mut self) {
        self.power = false;

        self.nr41 = Nr41::default();
        self.nr42 = Nr42::default();
        self.nr43 = Nr43::default();
        self.nr44 = Nr44::default();

        self.length_counter.power_off();

        self.dac.power_off();
    }

    pub fn amp(&self) -> isize {
        self.dac.amp()
    }

    pub fn pcm(&self) -> usize {
        self.dac.pcm()
    }
}

#[derive(Debug, Clone)]
struct Lfsr {
    value: u16,
    short: bool,
}

impl Lfsr {
    fn new() -> Self {
        Self {
            value: 0xff,
            short: false,
        }
    }

    fn high(&self) -> bool {
        // Inverted bit 1
        self.value & 1 == 0
    }

    fn trigger(&mut self, short: bool) {
        self.short = short;
        self.value = 0xff;
    }

    fn update(&mut self) {
        // Xor bit 0 and 1.
        let bit = (self.value & 1) ^ ((self.value & 2) >> 1) & 1;

        // Shift right.
        self.value >>= 1;

        // Put the bit at bit 14 (i.e. 15 bits shift register)
        self.value = (self.value & !0x4000) | (bit << 14);

        if self.short {
            // Put the bit at bit 6 (i.e. 7 bits shift register)
            self.value = (self.value & !0x0040) | (bit << 6);
        }
    }
}
