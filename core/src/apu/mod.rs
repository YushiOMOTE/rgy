mod mixer;
mod noise;
mod tone;
mod util;
mod wave;

use self::{mixer::Mixer, noise::Noise, tone::Tone, wave::Wave};
use crate::hardware::HardwareHandle;
use alloc::boxed::Box;
use log::*;

pub struct Apu {
    tones: [Tone; 2],
    wave: Wave,
    noise: Noise,
    mixer: Mixer,
    enable: bool,
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
            enable: false,
        }
    }

    /// Read NR10 register (0xff10)
    pub fn read_tone_sweep(&self) -> u8 {
        self.tones[0].read_sweep()
    }

    /// Write NR10 register (0xff10)
    pub fn write_tone_sweep(&mut self, value: u8) {
        if !self.enable {
            return;
        }
        self.tones[0].write_sweep(value)
    }

    /// Read NR11/NR21 register (0xff11/0xff16)
    pub fn read_tone_wave(&self, tone: usize) -> u8 {
        self.tones[tone].read_wave()
    }

    /// Write NR11/NR21 register (0xff11/0xff16)
    pub fn write_tone_wave(&mut self, tone: usize, value: u8) {
        if !self.enable {
            return;
        }
        self.tones[tone].write_wave(value)
    }

    /// Read NR12/NR22 register (0xff12/0xff17)
    pub fn read_tone_envelop(&self, tone: usize) -> u8 {
        self.tones[tone].read_envelop()
    }

    /// Write NR12/NR22 register (0xff12/0xff17)
    pub fn write_tone_envelop(&mut self, tone: usize, value: u8) {
        if !self.enable {
            return;
        }
        self.tones[tone].write_envelop(value)
    }

    /// Read NR13/NR23 register (0xff13/0xff18)
    pub fn read_tone_freq_low(&self, tone: usize) -> u8 {
        self.tones[tone].read_freq_low()
    }

    /// Write NR13/NR23 register (0xff13/0xff18)
    pub fn write_tone_freq_low(&mut self, tone: usize, value: u8) {
        if !self.enable {
            return;
        }
        self.tones[tone].write_freq_low(value)
    }

    /// Read NR14/NR24 register (0xff14/0xff19)
    pub fn read_tone_freq_high(&self, tone: usize) -> u8 {
        self.tones[tone].read_freq_high()
    }

    /// Write NR14/NR24 register (0xff14/0xff19)
    pub fn write_tone_freq_high(&mut self, tone: usize, value: u8) {
        if !self.enable {
            return;
        }
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
        if !self.enable {
            return;
        }
        self.wave.write_enable(value);
        self.mixer.sync_wave(self.wave.clone());
    }

    /// Read NR31 register (0xff1b)
    pub fn read_wave_len(&self) -> u8 {
        self.wave.read_len()
    }

    /// Write NR31 register (0xff1b)
    pub fn write_wave_len(&mut self, value: u8) {
        if !self.enable {
            return;
        }
        self.wave.write_len(value);
    }

    /// Read NR32 register (0xff1c)
    pub fn read_wave_amp(&self) -> u8 {
        self.wave.read_amp()
    }

    /// Write NR32 register (0xff1c)
    pub fn write_wave_amp(&mut self, value: u8) {
        if !self.enable {
            return;
        }
        self.wave.write_amp(value)
    }

    /// Read NR33 register (0xff1d)
    pub fn read_wave_freq_low(&self) -> u8 {
        self.wave.read_freq_low()
    }

    /// Write NR33 register (0xff1d)
    pub fn write_wave_freq_low(&mut self, value: u8) {
        if !self.enable {
            return;
        }
        self.wave.write_freq_low(value)
    }

    /// Read NR34 register (0xff1e)
    pub fn read_wave_freq_high(&self) -> u8 {
        self.wave.read_freq_high()
    }

    /// Write NR34 register (0xff1e)
    pub fn write_wave_freq_high(&mut self, value: u8) {
        if !self.enable {
            return;
        }
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
        if !self.enable {
            return;
        }
        self.wave.write_wave_buf(offset, value)
    }

    /// Read NR41 register (0xff20)
    pub fn read_noise_len(&self) -> u8 {
        self.noise.read_len()
    }

    /// Write NR41 register (0xff20)
    pub fn write_noise_len(&mut self, value: u8) {
        if !self.enable {
            return;
        }
        self.noise.write_len(value)
    }

    /// Read NR42 register (0xff21)
    pub fn read_noise_envelop(&self) -> u8 {
        self.noise.read_envelop()
    }

    /// Write NR42 register (0xff21)
    pub fn write_noise_envelop(&mut self, value: u8) {
        if !self.enable {
            return;
        }
        self.noise.write_envelop(value)
    }

    /// Read NR43 register (0xff22)
    pub fn read_noise_poly_counter(&self) -> u8 {
        self.noise.read_poly_counter()
    }

    /// Write NR43 register (0xff22)
    pub fn write_noise_poly_counter(&mut self, value: u8) {
        if !self.enable {
            return;
        }
        self.noise.write_poly_counter(value)
    }

    /// Read NR44 register (0xff23)
    pub fn read_noise_select(&self) -> u8 {
        self.noise.read_select()
    }

    /// Write NR44 register (0xff23)
    pub fn write_noise_select(&mut self, value: u8) {
        if !self.enable {
            return;
        }
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
        if !self.enable {
            return;
        }
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
        if !self.enable {
            return;
        }
        self.mixer.write_so_mask(value)
    }

    /// Read NR52 register (0xff26)
    pub fn read_enable(&self) -> u8 {
        let mut v = 0x70;
        v |= if self.enable { 0x80 } else { 0x00 };
        v |= if self.tones[0].is_active() {
            0x01
        } else {
            0x00
        };
        v |= if self.tones[1].is_active() {
            0x02
        } else {
            0x00
        };
        v |= if self.wave.is_active() { 0x04 } else { 0x00 };
        v |= if self.noise.is_active() { 0x08 } else { 0x00 };
        debug!("Read NR52: {:02x}", v);
        v
    }

    /// Write NR52 register (0xff26)
    pub fn write_enable(&mut self, value: u8) {
        debug!("Write NR52: {:02x}", value);

        self.enable = value & 0x80 != 0;

        self.mixer.enable(self.enable);

        if self.enable {
            info!("Sound master enabled");
        } else {
            info!("Sound master disabled");
            // If disabled, clear all registers
            for tone in &mut self.tones {
                tone.clear();
            }
            self.wave.clear();
            self.noise.clear();
            self.mixer.clear();
        }
    }

    pub fn step(&mut self, cycles: usize) {
        let rate = 4_194_304;
        for tone in &mut self.tones {
            tone.proceed(rate, cycles);
        }
        self.wave.proceed(rate, cycles);
        self.noise.proceed(rate, cycles);
        self.mixer.proceed(rate, cycles);
    }
}
