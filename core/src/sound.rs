use alloc::boxed::Box;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use log::*;

use crate::device::IoHandler;
use crate::hardware::{HardwareHandle, Stream, StreamId};
use crate::mmu::{MemRead, MemWrite, Mmu};

trait AtomicHelper {
    type Item;

    fn get(&self) -> Self::Item;
    fn set(&self, v: Self::Item);
}

impl AtomicHelper for AtomicUsize {
    type Item = usize;

    fn get(&self) -> Self::Item {
        self.load(Ordering::SeqCst)
    }

    fn set(&self, v: Self::Item) {
        self.store(v, Ordering::SeqCst)
    }
}

impl AtomicHelper for AtomicBool {
    type Item = bool;

    fn get(&self) -> Self::Item {
        self.load(Ordering::SeqCst)
    }

    fn set(&self, v: Self::Item) {
        self.store(v, Ordering::SeqCst)
    }
}

#[derive(Clone, Debug)]
struct Control {
    volume: Arc<AtomicUsize>,
    enable: Arc<AtomicBool>,
}

impl Control {
    fn new() -> Self {
        Self {
            volume: Arc::new(AtomicUsize::new(0)),
            enable: Arc::new(AtomicBool::new(false)),
        }
    }
}

struct Sweep {
    enable: bool,
    freq: usize,
    time: usize,
    sub: bool,
    shift: usize,
    clock: usize,
}

impl Sweep {
    fn new(enable: bool, freq: usize, time: usize, sub: bool, shift: usize) -> Self {
        Self {
            enable,
            freq,
            time,
            sub,
            shift,
            clock: 0,
        }
    }

    fn freq(&mut self, rate: usize) -> usize {
        if !self.enable || self.time == 0 || self.shift == 0 {
            return self.freq;
        }

        let interval = rate * self.time / 128;

        self.clock += 1;
        if self.clock >= interval {
            self.clock -= interval;

            let p = self.freq / 2usize.pow(self.shift as u32);

            self.freq = if self.sub {
                self.freq.saturating_sub(p)
            } else {
                self.freq.saturating_add(p)
            };

            if self.freq >= 2048 || self.freq == 0 {
                self.enable = false;
                self.freq = 0;
            }
        }

        self.freq
    }
}

struct Envelop {
    amp: usize,
    count: usize,
    inc: bool,
    clock: usize,
}

impl Envelop {
    fn new(amp: usize, count: usize, inc: bool) -> Self {
        Self {
            amp,
            count,
            inc,
            clock: 0,
        }
    }

    fn amp(&mut self, rate: usize) -> usize {
        if self.amp == 0 {
            return 0;
        }

        let interval = rate * self.count / 64;

        self.clock += 1;
        if self.clock >= interval {
            self.clock -= interval;

            self.amp = if self.inc {
                self.amp.saturating_add(1).min(15)
            } else {
                self.amp.saturating_sub(1)
            };
        }

        self.amp
    }
}

struct Counter {
    enable: bool,
    count: usize,
    base: usize,
    clock: usize,
}

impl Counter {
    fn new(enable: bool, count: usize, base: usize) -> Self {
        Self {
            enable,
            count,
            base,
            clock: 0,
        }
    }

    fn stop(&mut self, rate: usize) -> bool {
        if !self.enable {
            return false;
        }

        let deadline = rate * (self.base - self.count) / 256;

        if self.clock >= deadline {
            true
        } else {
            self.clock += 1;
            false
        }
    }
}

struct WaveIndex {
    clock: usize,
    index: usize,
}

impl WaveIndex {
    fn new() -> Self {
        Self { clock: 0, index: 0 }
    }

    fn index(&mut self, rate: usize, freq: usize, max: usize) -> usize {
        self.clock += freq;

        if self.clock >= rate {
            self.clock -= rate;
            self.index = (self.index + 1) % max;
        }

        self.index
    }
}

struct LFSR {
    value: u16,
    short: bool,
}

impl LFSR {
    fn new(short: bool) -> Self {
        Self {
            value: 0xdead,
            short,
        }
    }

    fn high(&self) -> bool {
        self.value & 1 == 0
    }

    fn update(&mut self) {
        if self.short {
            self.value &= 0xff;
            let bit = (self.value & 0x0001)
                ^ ((self.value & 0x0004) >> 2)
                ^ ((self.value & 0x0008) >> 3)
                ^ ((self.value & 0x0010) >> 5);
            self.value = (self.value >> 1) | (bit << 7);
        } else {
            let bit = (self.value & 0x0001)
                ^ ((self.value & 0x0004) >> 2)
                ^ ((self.value & 0x0008) >> 3)
                ^ ((self.value & 0x0020) >> 5);
            self.value = (self.value >> 1) | (bit << 15);
        }
    }
}

struct RandomWave {
    lfsr: LFSR,
    clock: usize,
}

impl RandomWave {
    fn new(short: bool) -> Self {
        Self {
            lfsr: LFSR::new(short),
            clock: 0,
        }
    }

    fn high(&mut self, rate: usize, freq: usize) -> bool {
        self.clock += freq;

        if self.clock >= rate {
            self.clock -= rate;
            self.lfsr.update()
        }

        self.lfsr.high()
    }
}

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
            WaveDuty::P125 => 1,
            WaveDuty::P250 => 2,
            WaveDuty::P500 => 4,
            WaveDuty::P750 => 8,
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
    ctrl: Control,
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
            ctrl: Control::new(),
        }
    }
}

struct ToneStream {
    tone: Tone,
    sweep: Sweep,
    env: Envelop,
    counter: Counter,
    index: WaveIndex,
}

impl ToneStream {
    fn new(tone: Tone, sweep: bool) -> Self {
        let freq = 131072 / (2048 - tone.freq);
        let sweep = Sweep::new(
            sweep,
            freq,
            tone.sweep_time,
            tone.sweep_sub,
            tone.sweep_shift,
        );
        let env = Envelop::new(tone.env_init, tone.env_count, tone.env_inc);
        let counter = Counter::new(tone.counter, tone.sound_len, 64);

        Self {
            tone,
            sweep,
            env,
            counter,
            index: WaveIndex::new(),
        }
    }
}

impl Stream for ToneStream {
    fn next(&mut self, rate: u32) -> u16 {
        if !self.tone.ctrl.enable.get() {
            return 0;
        }

        let rate = rate as usize;

        // Stop counter
        if self.counter.stop(rate) {
            return 0;
        }

        // Envelop
        let amp = self.env.amp(rate);

        // Sweep
        let freq = self.sweep.freq(rate);

        // Square wave generation
        let index = self.index.index(rate, freq * 8, 8);
        if index < self.tone.wave_duty.into() {
            0
        } else {
            (amp * self.tone.ctrl.volume.get()) as u16
        }
    }
}

struct WaveStream {
    wave: Wave,
    counter: Counter,
    index: WaveIndex,
}

impl WaveStream {
    fn new(wave: Wave) -> Self {
        let counter = Counter::new(wave.counter, wave.sound_len, 256);

        Self {
            wave,
            counter,
            index: WaveIndex::new(),
        }
    }
}

impl Stream for WaveStream {
    fn next(&mut self, rate: u32) -> u16 {
        if !self.wave.ctrl.enable.get() {
            return 0;
        }

        let rate = rate as usize;

        // Stop counter
        if self.counter.stop(rate) {
            return 0;
        }

        let samples = self.wave.wavebuf.len() * 2;
        let freq = 65536 / (2048 - self.wave.freq.get());
        let index_freq = freq * samples;
        let index = self.index.index(rate, index_freq, samples);

        let amp = if index % 2 == 0 {
            self.wave.wavebuf[index / 2] >> 4
        } else {
            self.wave.wavebuf[index / 2] & 0xf
        };

        let amp = match self.wave.amp_shift.get() {
            0 => 0,
            1 => amp,
            2 => (amp >> 1),
            3 => (amp >> 2),
            _ => unreachable!(),
        };

        (self.wave.ctrl.volume.get() * amp as usize) as u16
    }
}

#[derive(Debug, Clone)]
struct Wave {
    enable: bool,
    sound_len: usize,
    amp_shift: Arc<AtomicUsize>,
    counter: bool,
    freq: Arc<AtomicUsize>,
    wavebuf: [u8; 16],
    ctrl: Control,
}

impl Wave {
    fn new() -> Self {
        Self {
            enable: false,
            sound_len: 0,
            amp_shift: Arc::new(AtomicUsize::new(0)),
            counter: false,
            freq: Arc::new(AtomicUsize::new(0)),
            wavebuf: [0; 16],
            ctrl: Control::new(),
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

    ctrl: Control,
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

            ctrl: Control::new(),
        }
    }
}

struct NoiseStream {
    noise: Noise,
    env: Envelop,
    counter: Counter,
    wave: RandomWave,
}

impl NoiseStream {
    fn new(noise: Noise) -> Self {
        let env = Envelop::new(noise.env_init, noise.env_count, noise.env_inc);
        let counter = Counter::new(noise.counter, noise.sound_len, 64);
        let wave = RandomWave::new(noise.step);

        Self {
            noise,
            env,
            counter,
            wave,
        }
    }
}

impl Stream for NoiseStream {
    fn next(&mut self, rate: u32) -> u16 {
        if !self.noise.ctrl.enable.get() {
            return 0;
        }

        let rate = rate as usize;

        // Stop counter
        if self.counter.stop(rate) {
            return 0;
        }

        // Envelop
        let amp = self.env.amp(rate);

        // Noise: 524288 Hz / r / 2 ^ (s+1)
        let r = self.noise.div_freq;
        let s = self.noise.shift_freq as u32;
        let freq = if r == 0 {
            // For r = 0, assume r = 0.5 instead
            524288 * 5 / 10 / 2usize.pow(s + 1)
        } else {
            524288 / self.noise.div_freq / 2usize.pow(s + 1)
        };

        if self.wave.high(rate, freq) {
            (amp * self.noise.ctrl.volume.get()) as u16
        } else {
            0
        }
    }
}

pub struct Sound {
    hw: HardwareHandle,
    tone1: Tone,
    tone2: Tone,
    wave: Wave,
    noise: Noise,
    enable: bool,
    so1_volume: usize,
    so2_volume: usize,
    so_mask: u8,
}

impl Sound {
    pub fn new(hw: HardwareHandle) -> Self {
        Self {
            hw,
            tone1: Tone::new(),
            tone2: Tone::new(),
            wave: Wave::new(),
            noise: Noise::new(),
            enable: false,
            so1_volume: 0,
            so2_volume: 0,
            so_mask: 0,
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

    fn update_volume(&mut self) {
        self.tone1.ctrl.enable.set(self.enable);
        self.tone2.ctrl.enable.set(self.enable);
        self.wave.ctrl.enable.set(self.enable);
        self.noise.ctrl.enable.set(self.enable);

        self.tone1.ctrl.volume.set(self.get_volume(1));
        self.tone2.ctrl.volume.set(self.get_volume(2));
        self.wave.ctrl.volume.set(self.get_volume(3));
        self.noise.ctrl.volume.set(self.get_volume(4));
    }

    fn get_volume(&self, id: u8) -> usize {
        let mask = 1 << id;
        let v1 = if self.so_mask & mask != 0 {
            self.so1_volume
        } else {
            0
        };
        let v2 = if self.so_mask & (mask << 4) != 0 {
            self.so2_volume
        } else {
            0
        };
        v1 + v2
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
            debug!("Wave amp shift: {:02x}", value);
            self.wave.amp_shift.set((value as usize >> 5) & 0x3);
        } else if addr == 0xff1d {
            debug!("Wave freq1: {:02x}", value);
            self.wave
                .freq
                .set((self.wave.freq.get() & !0xff) | value as usize);
        } else if addr == 0xff1e {
            debug!("Wave freq2: {:02x}", value);
            self.wave.counter = value & 0x40 != 0;
            self.wave
                .freq
                .set((self.wave.freq.get() & !0x700) | (((value & 0x7) as usize) << 8));
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
        } else if addr == 0xff24 {
            self.so1_volume = (value as usize & 0x70) >> 4;
            self.so2_volume = value as usize & 0x07;
            self.update_volume();
        } else if addr == 0xff25 {
            self.so_mask = value;
            self.update_volume();
        } else if addr == 0xff26 {
            self.enable = value & 0x80 != 0;
            if self.enable {
                info!("Sound master enabled");
            } else {
                info!("Sound master disabled");
            }
            self.update_volume();
        } else {
            info!("Write sound: {:04x} {:02x}", addr, value);
        }

        MemWrite::Block
    }
}
