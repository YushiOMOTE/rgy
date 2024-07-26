use crate::cpu::CPU_FREQ_HZ;

use super::{
    clock_divider::ClockDivider, dac::Dac, envelope::Envelope, length_counter::LengthCounter,
    timer::Timer,
};

use bitfield_struct::bitfield;

const NOISE_FREQ_HZ: usize = 262_144;

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
            divider: ClockDivider::new(CPU_FREQ_HZ, NOISE_FREQ_HZ),
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

    pub fn step(&mut self, cycles: usize) {
        self.length_counter.step(cycles);
        self.envelope.step(cycles);

        let times = self.divider.step(cycles);

        for _ in 0..times {
            self.update();
        }
    }

    pub fn step_with_rate(&mut self, rate: usize) {
        self.length_counter.step_with_rate(rate);
        self.envelope.step_with_rate(rate);

        self.divider.set_source_clock_rate(rate);

        let times = self.divider.step(1);

        for _ in 0..times {
            self.update();
        }
    }

    fn update(&mut self) {
        if !self.is_active() {
            return;
        }
        if !self.timer.tick() {
            return;
        }

        self.reload_timer();

        self.lfsr.update();

        self.dac.write(if self.lfsr.high() {
            self.envelope.amp()
        } else {
            0
        });
    }

    fn reload_timer(&mut self) {
        self.timer.set_interval(self.timer_interval());
    }

    fn timer_interval(&self) -> usize {
        let divider = self.nr43.div_freq();
        let shift = self.nr43.shift_freq();

        if divider == 0 {
            (5 << shift) / 10
        } else {
            divider << shift
        }
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

        self.nr41 = Nr41::default();
        self.nr42 = Nr42::default();
        self.nr43 = Nr43::default();
        self.nr44 = Nr44::default();

        self.length_counter.power_off();

        self.lfsr.reset();

        self.dac.power_off();
    }

    pub fn amp(&self) -> isize {
        self.dac.amp()
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
            value: 0,
            short: false,
        }
    }

    fn reset(&mut self) {
        self.value = 0;
        self.short = false;
    }

    fn high(&self) -> bool {
        // Inverted bit 1
        self.value & 1 == 0
    }

    fn trigger(&mut self, short: bool) {
        self.value = 0;
        self.short = short;
    }

    fn update(&mut self) {
        // NXOR bit 0 and 1.
        let bit = !((self.value & 0x01) ^ ((self.value & 0x02) >> 1)) & 1;

        // Put XOR-ed result in bit 15 (and bit 7 in short mode)
        if self.short {
            self.value = ((self.value >> 1) & 0x7f7f) | (bit << 7) | (bit << 15);
        } else {
            self.value = ((self.value >> 1) & 0x7fff) | (bit << 15);
        }
    }
}
