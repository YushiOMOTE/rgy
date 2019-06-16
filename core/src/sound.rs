use alloc::boxed::Box;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicUsize, Ordering};
use log::*;

use crate::device::IoHandler;
use crate::hardware::{HardwareHandle, Stream, StreamId};
use crate::mmu::{MemRead, MemWrite, Mmu};

#[derive(Debug, Clone, Copy)]
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

impl From<WaveDuty> for usize {
    fn from(s: WaveDuty) -> usize {
        match s {
            WaveDuty::P125 => 125,
            WaveDuty::P250 => 250,
            WaveDuty::P500 => 500,
            WaveDuty::P750 => 750,
        }
    }
}

#[derive(Debug, Clone)]
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

impl Tone {
    fn new() -> Self {
        Self {
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
}

struct ToneStream {
    tone: Tone,
    sweep: bool,

    stop_clock: usize,
    env_clock: usize,
    sweep_clock: usize,
    wave_clock: usize,

    amp: usize,
    freq: usize,
}

impl ToneStream {
    fn new(tone: Tone, sweep: bool) -> Self {
        let freq = tone.freq;
        let amp = tone.env_init;

        assert!(amp <= 15);

        Self {
            tone,
            sweep,
            stop_clock: 0,
            env_clock: 0,
            sweep_clock: 0,
            amp,
            freq: 131072 / (2048 - freq),
            wave_clock: 0,
        }
    }
}

impl Stream for ToneStream {
    fn next(&mut self, rate: u32) -> u16 {
        let rate = rate as usize;

        if self.amp == 0 {
            return 0;
        }

        // Stop/continuous
        if self.tone.counter {
            if self.stop_clock >= rate * (64 - self.tone.sound_len) / 256 {
                return 0;
            } else {
                self.stop_clock += 1;
            }
        }

        // Envelop
        self.env_clock += 1;
        if self.env_clock >= rate * self.tone.env_count / 64 {
            self.env_clock = 0;
            self.amp = if self.tone.env_inc {
                self.amp.saturating_add(1).min(15)
            } else {
                self.amp.saturating_sub(1)
            };
        }

        // Sweep
        if self.sweep && self.tone.sweep_time != 0 && self.tone.sweep_shift != 0 {
            self.sweep_clock += 1;
            if self.sweep_clock >= rate * self.tone.sweep_time / 128 {
                self.sweep_clock = 0;

                let f = self.freq;
                self.freq = if self.tone.sweep_sub {
                    (f - f / 2usize.pow(self.tone.sweep_shift as u32)).max(1)
                } else {
                    f + f / 2usize.pow(self.tone.sweep_shift as u32)
                };
            }
        }

        // Square wave generation
        self.wave_clock += self.freq;
        if self.wave_clock >= rate {
            self.wave_clock -= rate;
        }

        assert!(self.amp <= 15, "amp = {}", self.amp);

        if self.wave_clock <= usize::from(self.tone.wave_duty) * rate / 1000 {
            0
        } else {
            self.amp as u16
        }
    }
}

struct WaveStream {
    wave: Wave,

    stop_clock: usize,
    wave_clock: usize,
    wave_index: usize,
}

impl WaveStream {
    fn new(wave: Wave) -> Self {
        Self {
            wave,
            stop_clock: 0,
            wave_clock: 0,
            wave_index: 0,
        }
    }
}

impl Stream for WaveStream {
    fn next(&mut self, rate: u32) -> u16 {
        let rate = rate as usize;

        // Stop/continuous
        if self.wave.counter {
            if self.stop_clock >= rate * (256 - self.wave.sound_len) / 256 {
                return 0;
            } else {
                self.stop_clock += 1;
            }
        }

        let samples = self.wave.wavebuf.len() * 2;
        let freq = 65536 / (2048 - self.wave.freq.load(Ordering::SeqCst) as usize);
        let index_freq = freq * samples;

        self.wave_clock += index_freq;
        if self.wave_clock >= rate {
            self.wave_clock -= rate;
            self.wave_index = (self.wave_index + 1) % samples;
        }

        let amp = if self.wave_index % 2 == 0 {
            self.wave.wavebuf[self.wave_index / 2] >> 4
        } else {
            self.wave.wavebuf[self.wave_index / 2] & 0xf
        };

        match self.wave.volume.load(Ordering::SeqCst) {
            0 => 0,
            100 => amp as u16,
            50 => (amp >> 1) as u16,
            25 => (amp >> 2) as u16,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Wave {
    enable: bool,
    sound_len: usize,
    volume: Arc<AtomicUsize>,
    counter: bool,
    freq: Arc<AtomicUsize>,
    wavebuf: [u8; 16],
}

impl Wave {
    fn new() -> Self {
        Self {
            enable: false,
            sound_len: 0,
            volume: Arc::new(AtomicUsize::new(0)),
            counter: false,
            freq: Arc::new(AtomicUsize::new(0)),
            wavebuf: [0; 16],
        }
    }
}

#[derive(Debug, Clone)]
struct Noise {
    sound_len: usize,

    env_init: usize,
    env_inc: bool,
    env_count: usize,

    shift_freq: usize,
    step: bool,
    div_freq: usize,

    counter: bool,
    freq: usize,
}

impl Noise {
    fn new() -> Self {
        Self {
            sound_len: 0,

            env_init: 0,
            env_inc: false,
            env_count: 0,

            shift_freq: 0,
            step: false,
            div_freq: 0,

            counter: false,
            freq: 0,
        }
    }
}

struct NoiseStream {
    noise: Noise,
    stop_clock: usize,
    env_clock: usize,
    wave_clock: usize,
    amp: usize,
}

impl NoiseStream {
    fn new(noise: Noise) -> Self {
        let amp = noise.env_init;

        Self {
            noise,
            stop_clock: 0,
            env_clock: 0,
            wave_clock: 0,
            amp,
        }
    }
}

impl Stream for NoiseStream {
    fn next(&mut self, rate: u32) -> u16 {
        let rate = rate as usize;

        if self.amp == 0 {
            return 0;
        }

        // Stop/continuous
        if self.noise.counter {
            if self.stop_clock >= rate * (64 - self.noise.sound_len) / 256 {
                return 0;
            } else {
                self.stop_clock += 1;
            }
        }

        // Envelop
        self.env_clock += 1;
        if self.env_clock >= rate * self.noise.env_count / 64 {
            self.env_clock = 0;
            self.amp = if self.noise.env_inc {
                self.amp.saturating_add(1).min(15)
            } else {
                self.amp.saturating_sub(1)
            };
        }

        // Noise: 524288 Hz / r / 2^(s+1) ;For r=0 assume r=0.5 instead

        let freq = 524288
            / (self.noise.div_freq * 10).max(5)
            / 2usize.pow(self.noise.shift_freq as u32 + 1)
            / 10;

        self.wave_clock += freq;
        if self.wave_clock >= rate {
            self.wave_clock -= rate;
        }

        if self.wave_clock <= rate / (self.wave_clock % 991).max(10) {
            0
        } else {
            self.amp as u16
        }
    }
}

pub struct Sound {
    hw: HardwareHandle,
    tone1: Tone,
    tone2: Tone,
    wave: Wave,
    noise: Noise,
}

impl Sound {
    pub fn new(hw: HardwareHandle) -> Self {
        Self {
            hw,
            tone1: Tone::new(),
            tone2: Tone::new(),
            wave: Wave::new(),
            noise: Noise::new(),
        }
    }

    fn play_tone(&mut self, id: StreamId, tone: Tone, sweep: bool) {
        let stream = ToneStream::new(tone, sweep);

        self.hw.get().borrow_mut().sound_play(id, Box::new(stream));
    }

    fn play_wave(&mut self, wave: Wave) {
        if !wave.enable {
            return;
        }

        let stream = WaveStream::new(wave);
        self.hw
            .get()
            .borrow_mut()
            .sound_play(StreamId::Wave, Box::new(stream));
    }

    fn stop_wave(&self) {
        self.hw.get().borrow_mut().sound_stop(StreamId::Wave);
    }

    fn play_noise(&mut self, noise: Noise) {
        let stream = NoiseStream::new(noise);

        self.hw
            .get()
            .borrow_mut()
            .sound_play(StreamId::Noise, Box::new(stream));
    }
}

impl IoHandler for Sound {
    fn on_read(&mut self, _mmu: &Mmu, _addr: u16) -> MemRead {
        MemRead::PassThrough
    }

    fn on_write(&mut self, _mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        if addr == 0xff10 {
            self.tone1.sweep_time = ((value >> 4) & 0x7) as usize;
            self.tone1.sweep_sub = value & 0x08 != 0;
            self.tone1.sweep_shift = (value & 0x07) as usize;
        } else if addr == 0xff11 {
            self.tone1.wave_duty = (value >> 6).into();
            self.tone1.sound_len = (value & 0x1f) as usize;
        } else if addr == 0xff12 {
            self.tone1.env_init = (value >> 4) as usize;
            self.tone1.env_inc = value & 0x08 != 0;
            self.tone1.env_count = (value & 0x7) as usize;
        } else if addr == 0xff13 {
            self.tone1.freq = (self.tone1.freq & !0xff) | value as usize;
        } else if addr == 0xff14 {
            self.tone1.counter = value & 0x40 != 0;
            self.tone1.freq = (self.tone1.freq & !0x700) | (((value & 0x7) as usize) << 8);
            if value & 0x80 != 0 {
                self.play_tone(StreamId::Tone1, self.tone1.clone(), true);
            }
        } else if addr == 0xff16 {
            self.tone2.wave_duty = (value >> 6).into();
            self.tone2.sound_len = (value & 0x1f) as usize;
        } else if addr == 0xff17 {
            self.tone2.env_init = (value >> 4) as usize;
            self.tone2.env_inc = value & 0x08 != 0;
            self.tone2.env_count = (value & 0x7) as usize;
        } else if addr == 0xff18 {
            self.tone2.freq = (self.tone2.freq & !0xff) | value as usize;
        } else if addr == 0xff19 {
            self.tone2.counter = value & 0x40 != 0;
            self.tone2.freq = (self.tone2.freq & !0x700) | (((value & 0x7) as usize) << 8);
            if value & 0x80 != 0 {
                self.play_tone(StreamId::Tone2, self.tone2.clone(), false);
            }
        } else if addr == 0xff1a {
            debug!("Wave enable: {:02x}", value);
            self.wave.enable = value & 0x80 != 0;
            if self.wave.enable {
                self.play_wave(self.wave.clone());
            } else {
                self.stop_wave();
            }
        } else if addr == 0xff1b {
            debug!("Wave len: {:02x}", value);
            self.wave.sound_len = value as usize;
        } else if addr == 0xff1c {
            debug!("Wave volume: {:02x}", value);
            self.wave.volume.store(
                match (value >> 5) & 0x3 {
                    0x0 => 0,
                    0x1 => 100,
                    0x2 => 50,
                    0x3 => 25,
                    _ => unreachable!(),
                },
                Ordering::SeqCst,
            );
        } else if addr == 0xff1d {
            debug!("Wave freq1: {:02x}", value);
            self.wave.freq.store(
                (self.wave.freq.load(Ordering::SeqCst) & !0xff) | value as usize,
                Ordering::SeqCst,
            );
        } else if addr == 0xff1e {
            debug!("Wave freq2: {:02x}", value);
            self.wave.counter = value & 0x40 != 0;
            self.wave.freq.store(
                (self.wave.freq.load(Ordering::SeqCst) & !0x700) | (((value & 0x7) as usize) << 8),
                Ordering::SeqCst,
            );
            if value & 0x80 != 0 {
                self.play_wave(self.wave.clone());
            }
        } else if addr >= 0xff30 && addr <= 0xff3f {
            self.wave.wavebuf[(addr - 0xff30) as usize] = value;
        } else if addr == 0xff20 {
            self.noise.sound_len = (value & 0x1f) as usize;
        } else if addr == 0xff21 {
            self.noise.env_init = (value >> 4) as usize;
            self.noise.env_inc = value & 0x08 != 0;
            self.noise.env_count = (value & 0x7) as usize;
        } else if addr == 0xff22 {
            self.noise.shift_freq = (value >> 4) as usize;
            self.noise.step = value & 0x08 != 0;
            self.noise.div_freq = (value & 0x7) as usize;
        } else if addr == 0xff23 {
            self.noise.counter = value & 0x40 != 0;
            if value & 0x80 != 0 {
                self.play_noise(self.noise.clone());
            }
        }

        MemWrite::Block
    }
}
