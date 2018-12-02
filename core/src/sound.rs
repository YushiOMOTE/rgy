use std::rc::Rc;
use std::cell::RefCell;

use crate::mmu::{MemHandler, MemRead, MemWrite, Mmu};

pub trait Stream {
    fn poll(&mut self) -> Option<f32>;
}

pub trait Speaker {
    fn sample_rate(&self) -> f32;

    fn play(&self, stream: Box<Stream>);

    fn stop(&self);
}

pub struct Sound {
    inner: Rc<RefCell<Inner>>,
}

impl Sound {
    pub fn new() -> Sound {
        Sound {
            inner: Rc::new(RefCell::new(Inner::new())),
        }
    }

    pub fn handler(&self) -> SoundMemHandler {
        SoundMemHandler::new(self.inner.clone())
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
struct Inner {
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

impl Inner {
    fn new() -> Inner {
        Inner {
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
        }
    }

    fn on_read(&mut self, mmu: &Mmu, addr: u16) -> MemRead {
        MemRead::PassThrough
    }

    fn on_write(&mut self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        if addr == 0xff10 {
            self.sweep_time = ((value >> 4) & 0x7) as usize;
            self.sweep_sub = value & 0x08 != 0;
            self.sweep_shift = (value & 0x07) as usize;
        } else if addr == 0xff11 {
            self.wave_duty = (value >> 6).into();
            self.sound_len = (value & 0x3f) as usize;
        } else if addr == 0xff12 {
            self.env_init = (value >> 4) as usize;
            self.env_inc = value & 0x08 != 0;
            self.env_count = (value & 0x7) as usize;
        } else if addr == 0xff13 {
            self.freq = (self.freq & !0xff) | value as usize;
        } else if addr == 0xff14 {
            self.counter = value & 0x40 != 0;
            self.freq = (self.freq & !0x700) | (((value & 0x7) as usize) << 8);
            if value & 0x80 != 0 {
                debug!("Play: {:#?}", self);
                self.play_tone1();
            }
        }

        MemWrite::Block
    }

    fn play_tone1(&mut self) {
        let freq = 131072f32 / (2048f32 - self.freq as f32);

        debug!("Freq: {}", freq);
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
