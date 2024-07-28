use alloc::boxed::Box;

use log::*;

use crate::hardware::HardwareHandle;

use self::{frame_sequencer::FrameSequencer, mixer::Mixer, noise::Noise, tone::Tone, wave::Wave};

use bitfield_struct::bitfield;

mod dac;
mod envelope;
mod frame_sequencer;
mod length_counter;
mod mixer;
mod noise;
mod sweep;
mod tone;
mod wave;

pub struct Apu {
    tones: [Tone; 2],
    wave: Wave,
    noise: Noise,
    mixer: Mixer,
    nr52: Nr52,
    frame_sequencer: FrameSequencer,
}

#[bitfield(u8)]
struct Nr52 {
    ch1_on: bool,
    ch2_on: bool,
    ch3_on: bool,
    ch4_on: bool,
    #[bits(3)]
    _unused: u8,
    power_on: bool,
}

impl Apu {
    pub fn new(hw: HardwareHandle) -> Self {
        let mixer = Mixer::new();

        hw.get()
            .borrow_mut()
            .sound_play(Box::new(mixer.create_stream()));

        Self {
            tones: [Tone::new(true), Tone::new(false)],
            wave: Wave::new(),
            noise: Noise::new(),
            mixer,
            nr52: Nr52::default(),
            frame_sequencer: FrameSequencer::new(),
        }
    }

    /// Read NR10 register (0xff10)
    pub fn read_tone_sweep(&self) -> u8 {
        self.tones[0].read_sweep()
    }

    /// Write NR10 register (0xff10)
    pub fn write_tone_sweep(&mut self, value: u8) {
        self.tones[0].write_sweep(value)
    }

    /// Read NR11/NR21 register (0xff11/0xff16)
    pub fn read_tone_wave(&self, tone: usize) -> u8 {
        self.tones[tone].read_wave()
    }

    /// Write NR11/NR21 register (0xff11/0xff16)
    pub fn write_tone_wave(&mut self, tone: usize, value: u8) {
        self.tones[tone].write_wave(value)
    }

    /// Read NR12/NR22 register (0xff12/0xff17)
    pub fn read_tone_envelop(&self, tone: usize) -> u8 {
        self.tones[tone].read_envelop()
    }

    /// Write NR12/NR22 register (0xff12/0xff17)
    pub fn write_tone_envelop(&mut self, tone: usize, value: u8) {
        self.tones[tone].write_envelop(value)
    }

    /// Read NR13/NR23 register (0xff13/0xff18)
    pub fn read_tone_freq_low(&self, tone: usize) -> u8 {
        self.tones[tone].read_freq_low()
    }

    /// Write NR13/NR23 register (0xff13/0xff18)
    pub fn write_tone_freq_low(&mut self, tone: usize, value: u8) {
        self.tones[tone].write_freq_low(value)
    }

    /// Read NR14/NR24 register (0xff14/0xff19)
    pub fn read_tone_freq_high(&self, tone: usize) -> u8 {
        self.tones[tone].read_freq_high()
    }

    /// Write NR14/NR24 register (0xff14/0xff19)
    pub fn write_tone_freq_high(&mut self, tone: usize, value: u8) {
        if self.tones[tone].write_freq_high(value) {
            self.mixer.sync_tone(tone, self.tones[tone].clone());
        }
    }

    /// Read NR30 register (0xff1a)
    pub fn read_wave_enable(&self) -> u8 {
        self.wave.read_enable()
    }

    /// Write NR30 register (0xff1a)
    pub fn write_wave_enable(&mut self, value: u8) {
        self.wave.write_enable(value);
        self.mixer.sync_wave(self.wave.clone());
    }

    /// Read NR31 register (0xff1b)
    pub fn read_wave_len(&self) -> u8 {
        self.wave.read_len()
    }

    /// Write NR31 register (0xff1b)
    pub fn write_wave_len(&mut self, value: u8) {
        self.wave.write_len(value);
    }

    /// Read NR32 register (0xff1c)
    pub fn read_wave_amp(&self) -> u8 {
        self.wave.read_amp()
    }

    /// Write NR32 register (0xff1c)
    pub fn write_wave_amp(&mut self, value: u8) {
        self.wave.write_amp(value)
    }

    /// Read NR33 register (0xff1d)
    pub fn read_wave_freq_low(&self) -> u8 {
        self.wave.read_freq_low()
    }

    /// Write NR33 register (0xff1d)
    pub fn write_wave_freq_low(&mut self, value: u8) {
        self.wave.write_freq_low(value)
    }

    /// Read NR34 register (0xff1e)
    pub fn read_wave_freq_high(&self) -> u8 {
        self.wave.read_freq_high()
    }

    /// Write NR34 register (0xff1e)
    pub fn write_wave_freq_high(&mut self, value: u8) {
        if self.wave.write_freq_high(value) {
            self.mixer.sync_wave(self.wave.clone());
        }
    }

    /// Read wave pattern buffer
    pub fn read_wave_buf(&self, offset: u16) -> u8 {
        self.wave.read_wave_buf(offset)
    }

    /// Write wave pattern buffer
    pub fn write_wave_buf(&mut self, offset: u16, value: u8) {
        self.wave.write_wave_buf(offset, value)
    }

    /// Read NR41 register (0xff20)
    pub fn read_noise_len(&self) -> u8 {
        self.noise.read_len()
    }

    /// Write NR41 register (0xff20)
    pub fn write_noise_len(&mut self, value: u8) {
        self.noise.write_len(value)
    }

    /// Read NR42 register (0xff21)
    pub fn read_noise_envelop(&self) -> u8 {
        self.noise.read_envelop()
    }

    /// Write NR42 register (0xff21)
    pub fn write_noise_envelop(&mut self, value: u8) {
        self.noise.write_envelop(value)
    }

    /// Read NR43 register (0xff22)
    pub fn read_noise_poly_counter(&self) -> u8 {
        self.noise.read_poly_counter()
    }

    /// Write NR43 register (0xff22)
    pub fn write_noise_poly_counter(&mut self, value: u8) {
        self.noise.write_poly_counter(value)
    }

    /// Read NR44 register (0xff23)
    pub fn read_noise_select(&self) -> u8 {
        self.noise.read_select()
    }

    /// Write NR44 register (0xff23)
    pub fn write_noise_select(&mut self, value: u8) {
        if self.noise.write_select(value) {
            self.mixer.sync_noise(self.noise.clone());
        }
    }

    /// Read NR50 register (0xff24)
    pub fn read_ctrl(&self) -> u8 {
        let ctrl = self.mixer.read_ctrl();
        debug!("Read NR50: {:02x}", ctrl);
        ctrl
    }

    /// Write NR50 register (0xff24)
    pub fn write_ctrl(&mut self, value: u8) {
        self.mixer.write_ctrl(value)
    }

    /// Read NR51 register (0xff25)
    pub fn read_so_mask(&self) -> u8 {
        let mask = self.mixer.read_so_mask();
        debug!("Read NR51: {:02x}", mask);
        mask
    }

    /// Write NR51 register (0xff25)
    pub fn write_so_mask(&mut self, value: u8) {
        self.mixer.write_so_mask(value)
    }

    /// Read NR52 register (0xff26)
    pub fn read_enable(&self) -> u8 {
        self.nr52
            .with_ch1_on(self.tones[0].is_active())
            .with_ch2_on(self.tones[1].is_active())
            .with_ch3_on(self.wave.is_active())
            .with_ch4_on(self.noise.is_active())
            .into_bits()
            | 0x70
    }

    /// Write NR52 register (0xff26)
    pub fn write_enable(&mut self, value: u8) {
        let before = self.nr52.power_on();

        self.nr52 = Nr52::from_bits(value);

        let after = self.nr52.power_on();

        if !before && after {
            info!("Sound master enabled");

            self.frame_sequencer.reset_step();

            for tone in &mut self.tones {
                tone.power_on();
            }

            self.wave.power_on();
            self.noise.power_on();
            self.mixer.power_on();

            self.sync_all();
        } else if before && !after {
            info!("Sound master disabled");

            for tone in &mut self.tones {
                tone.power_off();
            }
            self.wave.power_off();
            self.noise.power_off();
            self.mixer.power_off();

            self.sync_all();
        }
    }

    fn sync_all(&mut self) {
        for (i, tone) in self.tones.iter().enumerate() {
            self.mixer.sync_tone(i, tone.clone());
        }
        self.mixer.sync_wave(self.wave.clone());
        self.mixer.sync_noise(self.noise.clone());
    }

    pub fn step(&mut self, cycles: usize, div_apu: bool) {
        let frame = self.frame_sequencer.step(cycles, div_apu);

        for tone in &mut self.tones {
            tone.step(cycles, frame);
        }
        self.wave.step(cycles, frame);
        self.noise.step(cycles, frame);
    }
}
