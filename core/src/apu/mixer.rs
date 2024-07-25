use super::{
    noise::{Noise, NoiseStream},
    tone::{Tone, ToneStream},
    util::AtomicHelper,
    wave::{Wave, WaveStream},
};
use crate::hardware::Stream;
use alloc::sync::Arc;
use bitfield_struct::bitfield;
use core::sync::atomic::{AtomicBool, AtomicUsize};
use spin::Mutex;

pub struct Mixer {
    power: bool,
    nr50: Nr50,
    nr51: Nr51,
    stream: MixerStream,
}

#[bitfield(u8)]
struct Nr50 {
    #[bits(3)]
    right_volume: usize,
    vin_right_enable: bool,
    #[bits(3)]
    left_volume: usize,
    vin_left_enable: bool,
}

#[bitfield(u8)]
struct Nr51 {
    ch1_right: bool,
    ch2_right: bool,
    ch3_right: bool,
    ch4_right: bool,
    ch1_left: bool,
    ch2_left: bool,
    ch3_left: bool,
    ch4_left: bool,
}

impl Mixer {
    pub fn new() -> Self {
        Self {
            power: false,
            nr50: Nr50::default(),
            nr51: Nr51::default(),
            stream: MixerStream::new(),
        }
    }

    /// Read NR50 register (0xff24)
    pub fn read_ctrl(&self) -> u8 {
        self.nr50.into_bits()
    }

    /// Write NR50 register (0xff24)
    pub fn write_ctrl(&mut self, value: u8) {
        if !self.power {
            return;
        }

        self.nr50 = Nr50::from_bits(value);

        self.update_stream();
    }

    /// Read NR51 register (0xff25)
    pub fn read_so_mask(&self) -> u8 {
        self.nr51.into_bits()
    }

    /// Write NR51 register (0xff25)
    pub fn write_so_mask(&mut self, value: u8) {
        if !self.power {
            return;
        }

        self.nr51 = Nr51::from_bits(value);

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

    // Update streams based on register settings
    fn update_stream(&mut self) {
        self.stream.enable.set(self.power);

        if self.power {
            for (i, tone) in self.stream.tones.iter().enumerate() {
                tone.volume.set(self.get_tone_volume(i))
            }
            self.stream.wave.volume.set(self.get_wave_volume());
            self.stream.noise.volume.set(self.get_noise_volume());
        }
    }

    fn get_tone_volume(&self, tone: usize) -> usize {
        if tone == 0 {
            self.get_volume(self.nr51.ch1_right(), self.nr51.ch1_left())
        } else {
            self.get_volume(self.nr51.ch2_right(), self.nr51.ch2_left())
        }
    }

    fn get_wave_volume(&self) -> usize {
        self.get_volume(self.nr51.ch3_right(), self.nr51.ch3_left())
    }

    fn get_noise_volume(&self) -> usize {
        self.get_volume(self.nr51.ch4_right(), self.nr51.ch4_left())
    }

    fn get_volume(&self, right_enable: bool, left_enable: bool) -> usize {
        let right = if right_enable {
            self.nr50.right_volume()
        } else {
            0
        };

        let left = if left_enable {
            self.nr50.left_volume()
        } else {
            0
        };

        right + left
    }

    pub fn power_on(&mut self) {
        self.power = true;
        self.update_stream();
    }

    pub fn power_off(&mut self) {
        self.power = false;

        self.nr50 = Nr50::default();
        self.nr51 = Nr51::default();

        for tone in &mut self.stream.tones {
            tone.clear();
        }
        self.stream.wave.clear();
        self.stream.noise.clear();
        self.update_stream();
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
