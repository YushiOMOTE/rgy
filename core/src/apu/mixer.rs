use super::{
    frame_sequencer::{Frame, FrameSequencer},
    noise::Noise,
    tone::Tone,
    wave::Wave,
};
use crate::{cpu::CPU_FREQ_HZ, divider::Divider, hardware::Stream};
use alloc::sync::Arc;
use bitfield_struct::bitfield;
use core::sync::atomic::{AtomicIsize, Ordering};
use spin::Mutex;

pub struct Mixer {
    power: bool,
    nr50: Nr50,
    nr51: Nr51,
    state: SharedState,
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
            state: SharedState::new(),
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
        self.state.tones[index].sync_channel(tone);
    }

    pub fn sync_wave(&mut self, wave: Wave) {
        self.state.wave.sync_channel(wave);
    }

    pub fn sync_noise(&mut self, noise: Noise) {
        self.state.noise.sync_channel(noise);
    }

    fn sync_volume(&mut self) {
        self.state
            .sync_volume(0, (self.nr50.left_volume() + 1) as isize);
        self.state
            .sync_volume(1, (self.nr50.right_volume() + 1) as isize);

        self.state.tones[0].sync_volume(0, self.nr51.ch1_left() as isize);
        self.state.tones[0].sync_volume(1, self.nr51.ch1_right() as isize);
        self.state.tones[1].sync_volume(0, self.nr51.ch2_left() as isize);
        self.state.tones[1].sync_volume(1, self.nr51.ch2_right() as isize);
        self.state
            .wave
            .sync_volume(0, self.nr51.ch3_left() as isize);
        self.state
            .wave
            .sync_volume(1, self.nr51.ch3_right() as isize);
        self.state
            .noise
            .sync_volume(0, self.nr51.ch4_left() as isize);
        self.state
            .noise
            .sync_volume(1, self.nr51.ch4_right() as isize);
    }

    pub fn create_stream(&self) -> MixerStream {
        MixerStream::new(self.state.clone())
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

    fn step(&mut self, cycles: usize, frame: Frame) {
        self.channel.lock().step(cycles, frame);
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

    fn step(&mut self, rate: usize, frame: Frame);
}

impl VolumeUnit for Tone {
    fn amp(&self) -> isize {
        self.amp()
    }

    fn step(&mut self, cycles: usize, frame: Frame) {
        self.step(cycles, frame);
    }
}

impl VolumeUnit for Wave {
    fn amp(&self) -> isize {
        self.amp()
    }

    fn step(&mut self, cycles: usize, frame: Frame) {
        self.step(cycles, frame);
    }
}

impl VolumeUnit for Noise {
    fn amp(&self) -> isize {
        self.amp()
    }

    fn step(&mut self, cycles: usize, frame: Frame) {
        self.step(cycles, frame);
    }
}

#[derive(Clone)]
pub struct SharedState {
    tones: [Shared<Tone>; 2],
    wave: Shared<Wave>,
    noise: Shared<Noise>,
    volumes: [Arc<AtomicIsize>; 2],
}

impl SharedState {
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

pub struct MixerStream {
    state: SharedState,
    upscaler: UpScaler,
    divider: Divider,
    frame_sequencer: FrameSequencer,
}

impl MixerStream {
    fn new(state: SharedState) -> Self {
        Self {
            state,
            upscaler: UpScaler::new(CPU_FREQ_HZ),
            divider: Divider::new(),
            frame_sequencer: FrameSequencer::new(),
        }
    }

    fn step(&mut self, rate: usize) {
        let mut cycles = self.upscaler.compute_cycles(rate);

        while cycles > 0 {
            let sub_cycles = cycles.max(4);

            let div = self.divider.step(sub_cycles);
            let step = self.frame_sequencer.step(cycles, div);

            self.state.tones[0].step(sub_cycles, step);
            self.state.tones[1].step(sub_cycles, step);
            self.state.wave.step(sub_cycles, step);
            self.state.noise.step(sub_cycles, step);

            cycles -= sub_cycles;
        }
    }
}

#[derive(Clone)]
struct UpScaler {
    target_rate: usize,
    count: usize,
}

impl UpScaler {
    fn new(target_rate: usize) -> Self {
        Self {
            target_rate,
            count: 0,
        }
    }

    fn compute_cycles(&mut self, rate: usize) -> usize {
        let mut cycles = 0;

        while self.count < self.target_rate {
            self.count += rate;
            cycles += 1;
        }
        self.count -= self.target_rate;

        cycles
    }
}

impl Stream for MixerStream {
    fn max(&self) -> u16 {
        // Master volume max is 8, we have left and right: 8 * 2
        // Each channel max is 15, 4 channels, left and right: 15 * 4 * 2
        8 * 2 * 15 * 4 * 2
    }

    fn next(&mut self, rate: u32) -> u16 {
        let (left, right) = self.next_dual(rate);
        (left + right) / 2
    }

    fn next_dual(&mut self, rate: u32) -> (u16, u16) {
        let center = (self.max() / 2) as isize;

        self.step(rate as usize);

        let mut values = (0..2).map(|i| {
            let mut vol = 0;
            vol += self.state.tones[0].volume(i);
            vol += self.state.tones[1].volume(i);
            vol += self.state.wave.volume(i);
            vol += self.state.noise.volume(i);
            vol *= self.state.volume(i);
            (vol + center) as u16 * 2
        });

        (values.next().unwrap(), values.next().unwrap())
    }

    fn on(&self) -> bool {
        true
    }
}
