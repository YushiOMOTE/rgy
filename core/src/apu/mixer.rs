use super::{
    noise::{Noise, NoiseStream},
    tone::{Tone, ToneStream},
    util::AtomicHelper,
    wave::{Wave, WaveStream},
};
use crate::hardware::Stream;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicBool, AtomicUsize};
use spin::Mutex;

pub struct Mixer {
    power: bool,
    ctrl: u8,
    so1_volume: usize,
    so2_volume: usize,
    so_mask: usize,
    enable: bool,
    stream: MixerStream,
}

impl Mixer {
    pub fn new() -> Self {
        Self {
            power: false,
            ctrl: 0,
            so1_volume: 0,
            so2_volume: 0,
            so_mask: 0,
            enable: false,
            stream: MixerStream::new(),
        }
    }

    /// Read NR50 register (0xff24)
    pub fn read_ctrl(&self) -> u8 {
        self.ctrl
    }

    /// Write NR50 register (0xff24)
    pub fn write_ctrl(&mut self, value: u8) {
        if !self.power {
            return;
        }

        self.ctrl = value;
        self.so1_volume = (value as usize & 0x70) >> 4;
        self.so2_volume = value as usize & 0x07;
        self.update_stream();
    }

    /// Read NR51 register (0xff25)
    pub fn read_so_mask(&self) -> u8 {
        self.so_mask as u8
    }

    /// Write NR51 register (0xff25)
    pub fn write_so_mask(&mut self, value: u8) {
        if !self.power {
            return;
        }

        self.so_mask = value as usize;
        self.update_stream();
    }

    pub fn sync_tone(&mut self, index: usize, tone: Tone) {
        self.stream.tones[index].update(Some(tone.create_stream()));
    }

    pub fn sync_wave(&mut self, wave: Wave) {
        self.stream.wave.update(Some(wave.create_stream()));
    }

    pub fn sync_noise(&mut self, noise: Noise) {
        self.stream.noise.update(Some(noise.create_stream()));
    }

    pub fn step(&mut self, _cycles: usize) {}

    pub fn create_stream(&self) -> MixerStream {
        self.stream.clone()
    }

    pub fn enable(&mut self, enable: bool) {
        self.enable = enable;
        self.update_stream();
    }

    // Update streams based on register settings
    fn update_stream(&mut self) {
        self.stream.enable.set(self.enable);

        if self.enable {
            for (i, tone) in self.stream.tones.iter().enumerate() {
                tone.volume.set(self.get_volume(i as u8))
            }
            self.stream.wave.volume.set(self.get_volume(2));
            self.stream.noise.volume.set(self.get_volume(3));
        }
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

    pub fn power_on(&mut self) {
        self.power = true;
    }

    pub fn power_off(&mut self) {
        self.power = false;
        self.ctrl = 0;
        self.so1_volume = 0;
        self.so2_volume = 0;
        self.so_mask = 0;
        for tone in &mut self.stream.tones {
            tone.clear();
        }
        self.stream.wave.clear();
        self.stream.noise.clear();
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
    fn update(&self, s: Option<T>) {
        *self.stream.lock() = s;
    }

    fn clear(&self) {
        self.update(None);
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
pub struct MixerStream {
    tones: [Unit<ToneStream>; 2],
    wave: Unit<WaveStream>,
    noise: Unit<NoiseStream>,
    enable: Arc<AtomicBool>,
}

impl MixerStream {
    fn new() -> Self {
        Self {
            tones: [Unit::new(), Unit::new()],
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

            let (t, v) = self.tones[0].next(rate);
            vol += self.volume(t, v);
            let (t, v) = self.tones[1].next(rate);
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

    fn on(&self) -> bool {
        self.enable.get()
    }
}
