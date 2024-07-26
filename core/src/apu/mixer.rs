use super::{noise::Noise, tone::Tone, wave::Wave};
use crate::hardware::Stream;
use alloc::sync::Arc;
use bitfield_struct::bitfield;
use core::sync::atomic::{AtomicIsize, Ordering};
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

        self.sync_volume();
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

        self.sync_volume();
    }

    pub fn sync_tone(&mut self, index: usize, tone: Tone) {
        self.stream.tones[index].sync_channel(tone);
    }

    pub fn sync_wave(&mut self, wave: Wave) {
        self.stream.wave.sync_channel(wave);
    }

    pub fn sync_noise(&mut self, noise: Noise) {
        self.stream.noise.sync_channel(noise);
    }

    fn sync_volume(&mut self) {
        self.stream
            .sync_volume(0, (self.nr50.left_volume() + 1) as isize);
        self.stream
            .sync_volume(1, (self.nr50.right_volume() + 1) as isize);

        self.stream.tones[0].sync_volume(0, self.nr51.ch1_left() as isize);
        self.stream.tones[0].sync_volume(1, self.nr51.ch1_right() as isize);
        self.stream.tones[1].sync_volume(0, self.nr51.ch2_left() as isize);
        self.stream.tones[1].sync_volume(1, self.nr51.ch2_right() as isize);
        self.stream
            .wave
            .sync_volume(0, self.nr51.ch3_left() as isize);
        self.stream
            .wave
            .sync_volume(1, self.nr51.ch3_right() as isize);
        self.stream
            .noise
            .sync_volume(0, self.nr51.ch4_left() as isize);
        self.stream
            .noise
            .sync_volume(1, self.nr51.ch4_right() as isize);
    }

    pub fn step(&mut self, _cycles: usize) {}

    pub fn create_stream(&self) -> MixerStream {
        self.stream.clone()
    }

    pub fn power_on(&mut self) {
        self.power = true;

        self.sync_volume();
    }

    pub fn power_off(&mut self) {
        self.power = false;

        self.nr50 = Nr50::default();
        self.nr51 = Nr51::default();

        self.sync_volume();
    }
}

#[derive(Debug, Clone)]
struct Shared<T> {
    channel: Arc<Mutex<T>>,
    volumes: [Arc<AtomicIsize>; 2],
}

impl<T: VolumeUnit> Shared<T> {
    fn new(channel: T) -> Self {
        Self {
            channel: Arc::new(Mutex::new(channel)),
            volumes: [Arc::new(AtomicIsize::new(0)), Arc::new(AtomicIsize::new(0))],
        }
    }

    fn step(&mut self, rate: u32) {
        self.channel.lock().step(rate as usize);
    }

    fn sync_channel(&self, channel: T) {
        *self.channel.lock() = channel;
    }

    fn sync_volume(&self, index: usize, volume: isize) {
        self.volumes[index].store(volume, Ordering::Relaxed);
    }

    fn volume(&mut self, index: usize) -> isize {
        self.volumes[index].load(Ordering::Relaxed) * self.channel.lock().amp()
    }
}

trait VolumeUnit {
    fn amp(&self) -> isize;

    fn step(&mut self, rate: usize);
}

impl VolumeUnit for Tone {
    fn amp(&self) -> isize {
        self.amp()
    }

    fn step(&mut self, rate: usize) {
        self.step_with_rate(rate);
    }
}

impl VolumeUnit for Wave {
    fn amp(&self) -> isize {
        self.amp()
    }

    fn step(&mut self, rate: usize) {
        self.step_with_rate(rate);
    }
}

impl VolumeUnit for Noise {
    fn amp(&self) -> isize {
        self.amp()
    }

    fn step(&mut self, rate: usize) {
        self.step_with_rate(rate);
    }
}

#[derive(Clone)]
pub struct MixerStream {
    tones: [Shared<Tone>; 2],
    wave: Shared<Wave>,
    noise: Shared<Noise>,
    volumes: [Arc<AtomicIsize>; 2],
}

impl MixerStream {
    fn new() -> Self {
        Self {
            tones: [Shared::new(Tone::new(true)), Shared::new(Tone::new(false))],
            wave: Shared::new(Wave::new()),
            noise: Shared::new(Noise::new()),
            volumes: [Arc::new(AtomicIsize::new(0)), Arc::new(AtomicIsize::new(0))],
        }
    }

    fn sync_volume(&self, index: usize, volume: isize) {
        self.volumes[index].store(volume, Ordering::Relaxed);
    }

    fn volume(&self, index: usize) -> isize {
        self.volumes[index].load(Ordering::Relaxed)
    }
}

impl Stream for MixerStream {
    fn max(&self) -> u16 {
        // Master volume max is 8, we have left and right: 8 * 2
        // Each channel max is 15, 4 channels, left and right: 15 * 4 * 2
        8 * 2 * 15 * 4 * 2
    }

    fn next(&mut self, rate: u32) -> u16 {
        let center = (self.max() / 2) as isize;

        self.tones[0].step(rate);
        self.tones[1].step(rate);
        self.wave.step(rate);
        self.noise.step(rate);

        (0..2)
            .map(|i| {
                let mut vol = 0;
                vol += self.tones[0].volume(i);
                vol += self.tones[1].volume(i);
                vol += self.wave.volume(i);
                vol += self.noise.volume(i);
                vol *= self.volume(i);
                vol
            })
            .fold(0, |total, vol| total + vol + center) as u16
    }

    fn on(&self) -> bool {
        true
    }
}
