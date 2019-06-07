use log::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::device::HardwareHandle;
use crate::mmu::{MemHandler, MemRead, MemWrite, Mmu};

pub struct Sound {
    inner: Rc<RefCell<Inner>>,
}

impl Sound {
    pub fn new(hw: HardwareHandle) -> Sound {
        Sound {
            inner: Rc::new(RefCell::new(Inner::new(hw))),
        }
    }

    pub fn handler(&self) -> SoundMemHandler {
        SoundMemHandler::new(self.inner.clone())
    }
}

#[derive(Debug, Clone)]
enum WaveDuty {
    P125,
    P250,
    P500,
    P750,
}

impl From<WaveDuty> for u8 {
    fn from(s: WaveDuty) -> u8 {
        match s {
            WaveDuty::P125 => 0,
            WaveDuty::P250 => 1,
            WaveDuty::P500 => 2,
            WaveDuty::P750 => 3,
        }
    }
}

impl From<u8> for WaveDuty {
    fn from(s: u8) -> WaveDuty {
        match s {
            0 => WaveDuty::P125,
            1 => WaveDuty::P250,
            2 => WaveDuty::P500,
            3 => WaveDuty::P750,
            _ => unreachable!(),
        }
    }
}

impl From<WaveDuty> for f32 {
    fn from(s: WaveDuty) -> f32 {
        match s {
            WaveDuty::P125 => 0.125,
            WaveDuty::P250 => 0.25,
            WaveDuty::P500 => 0.5,
            WaveDuty::P750 => 0.75,
        }
    }
}

#[derive(Debug)]
struct Tone {
    sweep_time: usize,
    sweep_sub: bool,
    sweep_shift: usize,
    sound_len: usize,
    wave_duty: WaveDuty,
    env_init: usize,
    env_inc: bool,
    env_count: usize,
    counter: bool,
    freq: usize,
}

struct Inner {
    hw: HardwareHandle,
    tone: Tone,
}

impl Inner {
    fn new(hw: HardwareHandle) -> Inner {
        let tone = Tone {
            sweep_time: 0,
            sweep_sub: false,
            sweep_shift: 0,
            sound_len: 0,
            wave_duty: WaveDuty::P125,
            env_init: 0,
            env_inc: false,
            env_count: 0,
            counter: false,
            freq: 0,
        };

        Inner { hw, tone }
    }

    fn on_read(&mut self, mmu: &Mmu, addr: u16) -> MemRead {
        MemRead::PassThrough
    }

    fn on_write(&mut self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        if addr == 0xff10 {
            self.tone.sweep_time = ((value >> 4) & 0x7) as usize;
            self.tone.sweep_sub = value & 0x08 != 0;
            self.tone.sweep_shift = (value & 0x07) as usize;
        } else if addr == 0xff11 {
            self.tone.wave_duty = (value >> 6).into();
            self.tone.sound_len = (value & 0x3f) as usize;
        } else if addr == 0xff12 {
            self.tone.env_init = (value >> 4) as usize;
            self.tone.env_inc = value & 0x08 != 0;
            self.tone.env_count = (value & 0x7) as usize;
        } else if addr == 0xff13 {
            self.tone.freq = (self.tone.freq & !0xff) | value as usize;
        } else if addr == 0xff14 {
            self.tone.counter = value & 0x40 != 0;
            self.tone.freq = (self.tone.freq & !0x700) | (((value & 0x7) as usize) << 8);
            if value & 0x80 != 0 {
                debug!("Play: {:#?}", self.tone);
                self.play_tone1();
            }
        }

        MemWrite::Block
    }

    fn play_tone1(&mut self) {
        let amp = self.tone.env_init as f32 / 15.0;
        let env_count = self.tone.env_count as f32;
        let diff = amp / 15.0 as f32;
        let diff = if self.tone.env_inc { diff } else { diff * -1.0 };
        let freq = 131072f32 / (2048f32 - self.tone.freq as f32);
        let wave_duty = self.tone.wave_duty.clone();

        debug!("Freq: {}", freq);

        let mut clock = 0f32;
        let mut env_clock = 0f32;
        let mut amp = amp;

        self.hw.get().borrow_mut().sound_play(Box::new(move |rate| {
            // Envelope
            env_clock += 1.0;
            if env_clock >= rate * env_count / 64.0 {
                env_clock = 0.0;
                amp += diff;
                amp = if amp < 0.0 {
                    0.0
                } else if amp > 1.0 {
                    1.0
                } else {
                    amp
                };
            }

            let cycle = rate / freq;

            clock = (clock + 1.0) % cycle;

            let duty: f32 = wave_duty.clone().into();
            let duty = cycle * duty;

            if clock <= duty {
                Some(amp)
            } else {
                Some(-amp)
            }
        }));
    }

    fn play_tone2(&mut self) {}

    fn play_wave(&mut self) {}

    fn play_noise(&mut self) {}
}

pub struct SoundMemHandler {
    inner: Rc<RefCell<Inner>>,
}

impl SoundMemHandler {
    fn new(inner: Rc<RefCell<Inner>>) -> SoundMemHandler {
        SoundMemHandler { inner }
    }
}

impl MemHandler for SoundMemHandler {
    fn on_read(&self, mmu: &Mmu, addr: u16) -> MemRead {
        self.inner.borrow_mut().on_read(mmu, addr)
    }

    fn on_write(&self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        self.inner.borrow_mut().on_write(mmu, addr, value)
    }
}
