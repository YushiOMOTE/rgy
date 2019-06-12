use log::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::device::IoHandler;
use crate::hardware::{HardwareHandle, SoundId};
use crate::mmu::{MemRead, MemWrite, Mmu};

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

#[derive(Debug, Clone)]
struct Wave {
    enable: bool,
    sound_len: usize,
    volume: Arc<AtomicUsize>,
    counter: bool,
    freq: Arc<AtomicUsize>,
    wave: [u8; 16],
}

impl Wave {
    fn new() -> Self {
        Self {
            enable: false,
            sound_len: 0,
            volume: Arc::new(AtomicUsize::new(0)),
            counter: false,
            freq: Arc::new(AtomicUsize::new(0)),
            wave: [0; 16],
        }
    }
}

pub struct Sound {
    hw: HardwareHandle,
    tone1: Tone,
    tone2: Tone,
    wave: Wave,
}

impl Sound {
    pub fn new(hw: HardwareHandle) -> Self {
        Self {
            hw,
            tone1: Tone::new(),
            tone2: Tone::new(),
            wave: Wave::new(),
        }
    }

    fn play_tone(&mut self, id: SoundId, tone: Tone, sweep: bool) {
        let amp = tone.env_init as f32 / 15.0;
        let env_count = tone.env_count as f32;
        let diff = amp / 15.0 as f32;
        let diff = if tone.env_inc { diff } else { diff * -1.0 };
        let mut freq = 131072f32 / (2048f32 - tone.freq as f32);
        let wave_duty = tone.wave_duty.clone();
        let counter = tone.counter;
        let len = tone.sound_len as f32;

        let sweep_sub = tone.sweep_sub;
        let sweep_shift = tone.sweep_shift as f32;

        let mut clock = 0f32;
        let mut env_clock = 0f32;
        let mut sweep_clock = 0f32;
        let mut sweep_step = 0f32;
        let mut elapsed = 0f32;
        let mut amp = amp;

        // Sweep time in seconds
        let sweep_time: f32 = (tone.sweep_time as f32) * 0.0078;

        self.hw.get().borrow_mut().sound_play(
            id,
            Box::new(move |rate| {
                if counter {
                    if elapsed >= rate * (64.0 - len) / 256.0 {
                        return None;
                    } else {
                        elapsed += 1.0;
                    }
                }

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

                if sweep && sweep_time != 0.0 {
                    sweep_clock += 1.0;
                    if sweep_clock >= rate * sweep_time {
                        sweep_clock = 0.0;

                        let new_freq = if sweep_sub {
                            // min to avoid zero division
                            (freq - freq / (2f32.powf(sweep_shift))).max(1.0)
                        } else {
                            (freq + freq / (2f32.powf(sweep_shift)))
                        };

                        freq = new_freq;
                    }
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
            }),
        );
    }

    fn play_wave(&mut self, wave: Wave) {
        if !wave.enable {
            return;
        }

        debug!("Play wave: {:#?}", wave);

        let amp = wave.volume.clone();
        let freq = wave.freq.clone();
        let len = wave.sound_len as f32;
        let wavebuf = wave.wave.clone();
        let counter = wave.counter;

        let mut clock = 0f32;
        let mut elapsed = 0f32;

        self.hw.get().borrow_mut().sound_play(
            SoundId::Wave,
            Box::new(move |rate| {
                if counter {
                    if elapsed >= rate * (256.0 - len) / 256.0 {
                        return None;
                    } else {
                        elapsed += 1.0;
                    }
                }

                let f = 65536f32 / (2048f32 - freq.load(Ordering::SeqCst) as f32);
                let cycle = rate / f;

                clock = (clock + 1.0) % cycle;

                let idx = (clock / (cycle / 32.0)) as usize % wavebuf.len();
                let level = if idx % 2 == 0 {
                    (wavebuf[idx / 2] % 0xf) as f32
                } else {
                    (wavebuf[idx / 2] >> 4) as f32
                };

                let a = (amp.load(Ordering::SeqCst) as f32) / 100.0;
                let v = (-1.0 + level * 0.125) * a;

                assert_eq!(true, v >= -1.0 && v <= 1.0);

                // 0.5 to emphaise tone
                Some(v * 0.5)
            }),
        );
    }

    fn stop_wave(&self) {
        self.hw.get().borrow_mut().sound_stop(SoundId::Wave);
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
            self.tone1.sound_len = (value & 0x3f) as usize;
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
                self.play_tone(SoundId::Tone1, self.tone1.clone(), true);
            }
        } else if addr == 0xff16 {
            self.tone2.wave_duty = (value >> 6).into();
            self.tone2.sound_len = (value & 0x3f) as usize;
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
                self.play_tone(SoundId::Tone2, self.tone2.clone(), false);
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
            debug!("Sound buffer: {:02x}", value);
            self.wave.wave[(addr - 0xff30) as usize] = value;
        }

        MemWrite::Block
    }
}
