use alloc::boxed::Box;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use log::*;
use spin::Mutex;

use crate::hardware::{HardwareHandle, Stream};

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

        if self.count == 0 {
            return self.amp;
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

#[derive(Debug, Clone)]
struct Tone {
    sweep_time: usize,
    sweep_sub: bool,
    sweep_shift: usize,
    sound_len: usize,
    wave_duty: usize,
    env_init: usize,
    env_inc: bool,
    env_count: usize,
    counter: bool,
    freq: usize,
    nr: [u8; 5],
}

impl Tone {
    fn new() -> Self {
        Self {
            sweep_time: 0,
            sweep_sub: false,
            sweep_shift: 0,
            sound_len: 0,
            wave_duty: 0,
            env_init: 0,
            env_inc: false,
            env_count: 0,
            counter: false,
            freq: 0,
            nr: [0; 5],
        }
    }

    fn on_read(&self, base: u16, addr: u16) -> u8 {
        if addr == base + 3 {
            0xff
        } else {
            self.nr[(addr - base) as usize]
        }
    }

    fn on_write(&mut self, base: u16, addr: u16, value: u8) -> bool {
        if addr == base + 0 {
            self.sweep_time = ((value >> 4) & 0x7) as usize;
            self.sweep_sub = value & 0x08 != 0;
            self.sweep_shift = (value & 0x07) as usize;
        } else if addr == base + 1 {
            self.wave_duty = (value >> 6).into();
            self.sound_len = (value & 0x1f) as usize;
        } else if addr == base + 2 {
            self.env_init = (value >> 4) as usize;
            self.env_inc = value & 0x08 != 0;
            self.env_count = (value & 0x7) as usize;
        } else if addr == base + 3 {
            self.freq = (self.freq & !0xff) | value as usize;
        } else if addr == base + 4 {
            self.counter = value & 0x40 != 0;
            self.freq = (self.freq & !0x700) | (((value & 0x7) as usize) << 8);
            return value & 0x80 != 0;
        } else {
            unreachable!()
        }

        self.nr[(addr - base) as usize] = value;

        false
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
    fn max(&self) -> u16 {
        unreachable!()
    }

    fn next(&mut self, rate: u32) -> u16 {
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
}

#[derive(Debug, Clone)]
struct Wave {
    enable: bool,
    sound_len: usize,
    amp_shift: Arc<AtomicUsize>,
    counter: bool,
    freq: Arc<AtomicUsize>,
    wavebuf: [u8; 16],
    nr: [u8; 5],
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
            nr: [0; 5],
        }
    }

    fn on_read(&self, addr: u16) -> u8 {
        if addr == 0xff1d {
            0xff
        } else {
            self.nr[(addr - 0xff1a) as usize]
        }
    }

    fn on_write(&mut self, addr: u16, value: u8) -> bool {
        if addr >= 0xff1a && addr <= 0xff1e {
            self.nr[(addr - 0xff1a) as usize] = value;
        }

        if addr == 0xff1a {
            debug!("Wave enable: {:02x}", value);
            self.enable = value & 0x80 != 0;
            return true;
        } else if addr == 0xff1b {
            debug!("Wave len: {:02x}", value);
            self.sound_len = value as usize;
        } else if addr == 0xff1c {
            debug!("Wave amp shift: {:02x}", value);
            self.amp_shift.set((value as usize >> 5) & 0x3);
        } else if addr == 0xff1d {
            debug!("Wave freq1: {:02x}", value);
            self.freq.set((self.freq.get() & !0xff) | value as usize);
        } else if addr == 0xff1e {
            debug!("Wave freq2: {:02x}", value);
            self.counter = value & 0x40 != 0;
            self.freq
                .set((self.freq.get() & !0x700) | (((value & 0x7) as usize) << 8));
            return value & 0x80 != 0;
        } else if addr >= 0xff30 && addr <= 0xff3f {
            self.wavebuf[(addr - 0xff30) as usize] = value;
        } else {
            unreachable!()
        }

        false
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
    fn max(&self) -> u16 {
        unreachable!()
    }

    fn next(&mut self, rate: u32) -> u16 {
        if !self.wave.enable {
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

        amp as u16
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

    nr: [u8; 4],
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

            nr: [0; 4],
        }
    }

    fn on_read(&self, addr: u16) -> u8 {
        self.nr[(addr - 0xff20) as usize]
    }

    fn on_write(&mut self, addr: u16, value: u8) -> bool {
        self.nr[(addr - 0xff20) as usize] = value;

        if addr == 0xff20 {
            self.sound_len = (value & 0x1f) as usize;
        } else if addr == 0xff21 {
            self.env_init = (value >> 4) as usize;
            self.env_inc = value & 0x08 != 0;
            self.env_count = (value & 0x7) as usize;
        } else if addr == 0xff22 {
            self.shift_freq = (value >> 4) as usize;
            self.step = value & 0x08 != 0;
            self.div_freq = (value & 0x7) as usize;
        } else if addr == 0xff23 {
            self.counter = value & 0x40 != 0;
            return value & 0x80 != 0;
        } else {
            unreachable!()
        }

        false
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
    fn max(&self) -> u16 {
        unreachable!()
    }

    fn next(&mut self, rate: u32) -> u16 {
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
            amp as u16
        } else {
            0
        }
    }
}

struct Mixer {
    so1_volume: usize,
    so2_volume: usize,
    so_mask: usize,
    enable: bool,
    stream: MixerStream,
    nr: [u8; 3],
}

impl Mixer {
    fn new() -> Self {
        Self {
            so1_volume: 0,
            so2_volume: 0,
            so_mask: 0,
            enable: false,
            stream: MixerStream::new(),
            nr: [0; 3],
        }
    }

    fn setup_stream(&self, hw: &HardwareHandle) {
        hw.get()
            .borrow_mut()
            .sound_play(Box::new(self.stream.clone()))
    }

    fn on_read(&self, addr: u16) -> u8 {
        if addr == 0xff26 {
            let mut v = 0;
            v |= if self.enable { 0x80 } else { 0x00 };
            v |= if self.stream.tone1.on() { 0x08 } else { 0x00 };
            v |= if self.stream.tone2.on() { 0x04 } else { 0x00 };
            v |= if self.stream.wave.on() { 0x02 } else { 0x00 };
            v |= if self.stream.noise.on() { 0x01 } else { 0x00 };
            v
        } else {
            self.nr[(addr - 0xff24) as usize]
        }
    }

    fn on_write(&mut self, addr: u16, value: u8) {
        self.nr[(addr - 0xff24) as usize] = value;

        if addr == 0xff24 {
            self.so1_volume = (value as usize & 0x70) >> 4;
            self.so2_volume = value as usize & 0x07;
            self.update_volume();
        } else if addr == 0xff25 {
            self.so_mask = value as usize;
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
            unreachable!()
        }
    }

    fn restart_tone1(&self, t: Tone) {
        self.stream.tone1.update(Some(ToneStream::new(t, true)));
    }

    fn restart_tone2(&self, t: Tone) {
        self.stream.tone2.update(Some(ToneStream::new(t, false)));
    }

    fn restart_wave(&self, w: Wave) {
        self.stream.wave.update(Some(WaveStream::new(w)));
    }

    fn restart_noise(&self, n: Noise) {
        self.stream.noise.update(Some(NoiseStream::new(n)));
    }

    fn update_volume(&self) {
        self.stream.enable.set(self.enable);
        self.stream.tone1.volume.set(self.get_volume(0));
        self.stream.tone2.volume.set(self.get_volume(1));
        self.stream.wave.volume.set(self.get_volume(2));
        self.stream.noise.volume.set(self.get_volume(3));
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

struct Unit<T> {
    stream: Arc<Mutex<Option<T>>>,
    volume: Arc<AtomicUsize>,
}

impl<T> Clone for Unit<T> {
    fn clone(&self) -> Self {
        Self {
            stream: self.stream.clone(),
            volume: self.volume.clone(),
        }
    }
}

impl<T> Unit<T> {
    fn new() -> Self {
        Self {
            stream: Arc::new(Mutex::new(None)),
            volume: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl<T: Stream> Unit<T> {
    fn on(&self) -> bool {
        self.stream.lock().is_some()
    }

    fn update(&self, s: Option<T>) {
        *self.stream.lock() = s;
    }

    fn next(&self, rate: u32) -> (u16, u16) {
        (
            self.stream
                .lock()
                .as_mut()
                .map(|s| s.next(rate))
                .unwrap_or(0),
            self.volume.get() as u16,
        )
    }
}

#[derive(Clone)]
struct MixerStream {
    tone1: Unit<ToneStream>,
    tone2: Unit<ToneStream>,
    wave: Unit<WaveStream>,
    noise: Unit<NoiseStream>,
    enable: Arc<AtomicBool>,
}

impl MixerStream {
    fn new() -> Self {
        Self {
            tone1: Unit::new(),
            tone2: Unit::new(),
            wave: Unit::new(),
            noise: Unit::new(),
            enable: Arc::new(AtomicBool::new(false)),
        }
    }

    fn volume(&self, amp: u16, vol: u16) -> u16 {
        amp * vol
    }
}

impl Stream for MixerStream {
    fn max(&self) -> u16 {
        // volume max = 7 * 2 = 14
        // amplitude max = 15
        // total volume max = 14 * 15 * 4 = 840
        // * 3 to soften the sound
        840 * 3
    }

    fn next(&mut self, rate: u32) -> u16 {
        if self.enable.get() {
            let mut vol = 0;

            let (t, v) = self.tone1.next(rate);
            vol += self.volume(t, v);
            let (t, v) = self.tone2.next(rate);
            vol += self.volume(t, v);
            let (t, v) = self.wave.next(rate);
            vol += self.volume(t, v);
            let (t, v) = self.noise.next(rate);
            vol += self.volume(t, v) / 2; // Soften the noise

            assert!(vol <= 840, "vol = {}", vol);

            vol
        } else {
            0
        }
    }
}

pub struct Sound {
    tone1: Tone,
    tone2: Tone,
    wave: Wave,
    noise: Noise,
    mixer: Mixer,
}

impl Sound {
    pub fn new(hw: HardwareHandle) -> Self {
        let mixer = Mixer::new();

        mixer.setup_stream(&hw);

        Self {
            tone1: Tone::new(),
            tone2: Tone::new(),
            wave: Wave::new(),
            noise: Noise::new(),
            mixer,
        }
    }

    pub fn on_read(&self, addr: u16) -> u8 {
        match addr {
            0xff10..=0xff14 => self.tone1.on_read(0xff10, addr),
            0xff15..=0xff19 => self.tone2.on_read(0xff15, addr),
            0xff1a..=0xff1e => self.wave.on_read(addr),
            0xff20..=0xff23 => self.noise.on_read(addr),
            0xff24..=0xff26 => self.mixer.on_read(addr),
            _ => unimplemented!(),
        }
    }

    pub fn on_write(&mut self, addr: u16, value: u8) {
        if addr >= 0xff10 && addr <= 0xff14 {
            if self.tone1.on_write(0xff10, addr, value) {
                self.mixer.restart_tone1(self.tone1.clone());
            }
        } else if addr >= 0xff15 && addr <= 0xff19 {
            if self.tone2.on_write(0xff15, addr, value) {
                self.mixer.restart_tone2(self.tone2.clone());
            }
        } else if addr >= 0xff1a && addr <= 0xff1e {
            if self.wave.on_write(addr, value) {
                self.mixer.restart_wave(self.wave.clone());
            }
        } else if addr >= 0xff30 && addr <= 0xff3f {
            let _ = self.wave.on_write(addr, value);
        } else if addr >= 0xff20 && addr <= 0xff23 {
            if self.noise.on_write(addr, value) {
                self.mixer.restart_noise(self.noise.clone());
            }
        } else if addr >= 0xff24 && addr <= 0xff26 {
            self.mixer.on_write(addr, value);
        } else {
            info!("Write sound: {:04x} {:02x}", addr, value);
        }
    }
}
