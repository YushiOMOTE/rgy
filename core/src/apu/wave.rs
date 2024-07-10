use alloc::sync::Arc;
use core::sync::atomic::AtomicUsize;

use log::*;

use crate::hardware::Stream;

use super::util::{AtomicHelper, Counter, WaveIndex};

#[derive(Debug, Clone)]
pub struct Wave {
    enable: bool,
    amp: u8,
    amp_shift: Arc<AtomicUsize>,
    counter: Counter,
    freq: Arc<AtomicUsize>,
    freq_high: u8,
    wavebuf: [u8; 16],
    dac: bool,
}

impl Wave {
    pub fn new() -> Self {
        Self {
            enable: false,
            amp: 0,
            amp_shift: Arc::new(AtomicUsize::new(0)),
            counter: Counter::type256(),
            freq: Arc::new(AtomicUsize::new(0)),
            freq_high: 0,
            wavebuf: [0; 16],
            dac: false,
        }
    }

    /// Read NR30 register (0xff1a)
    pub fn read_enable(&self) -> u8 {
        if self.enable {
            0xff
        } else {
            0x7f
        }
    }

    /// Write NR30 register (0xff1a)
    pub fn write_enable(&mut self, value: u8) {
        self.enable = value & 0x80 != 0;
        self.dac = value & 0x80 != 0;
        if !self.dac {
            self.counter.deactivate();
        }
    }

    /// Read NR31 register (0xff1b)
    pub fn read_len(&self) -> u8 {
        // Write-only?
        0xff
    }

    /// Write NR31 register (0xff1b)
    pub fn write_len(&mut self, value: u8) {
        self.counter.load(value as usize);
    }

    /// Read NR32 register (0xff1c)
    pub fn read_amp(&self) -> u8 {
        self.amp | 0x9f
    }

    /// Write NR32 register (0xff1c)
    pub fn write_amp(&mut self, value: u8) {
        debug!("Wave amp shift: {:02x}", value);
        self.amp = value;
        self.amp_shift.set((value as usize >> 5) & 0x3);
    }

    /// Read NR33 register (0xff1d)
    pub fn read_freq_low(&self) -> u8 {
        // Write only
        0xff
    }

    /// Write NR33 register (0xff1d)
    pub fn write_freq_low(&mut self, value: u8) {
        self.freq.set((self.freq.get() & !0xff) | value as usize);
    }

    /// Read NR34 register (0xff1e)
    pub fn read_freq_high(&self) -> u8 {
        // Mask write-only bits
        self.freq_high | 0xbf
    }

    /// Write NR34 register (0xff1e)
    pub fn write_freq_high(&mut self, value: u8) -> bool {
        self.freq_high = value;
        self.freq
            .set((self.freq.get() & !0x700) | (((value & 0x7) as usize) << 8));
        let trigger = value & 0x80 != 0;
        let enable = value & 0x40 != 0;
        self.counter.update(trigger, enable);
        trigger
    }

    /// Read wave pattern buffer
    pub fn read_wave_buf(&self, offset: u16) -> u8 {
        self.wavebuf[offset as usize - 0xff30]
    }

    /// Write wave pattern buffer
    pub fn write_wave_buf(&mut self, offset: u16, value: u8) {
        self.wavebuf[offset as usize - 0xff30] = value;
    }

    /// Create stream from the current data
    pub fn create_stream(&self) -> WaveStream {
        WaveStream::new(self.clone())
    }

    pub fn step(&mut self, cycles: usize) {
        self.counter.step(cycles);
    }

    pub fn is_active(&self) -> bool {
        self.counter.is_active() && self.dac
    }

    pub fn clear(&mut self) {
        let mut wave = Wave::new();
        core::mem::swap(&mut wave.wavebuf, &mut self.wavebuf);
        core::mem::swap(self, &mut wave);
    }
}

pub struct WaveStream {
    wave: Wave,
    counter: Counter,
    index: WaveIndex,
}

impl WaveStream {
    fn new(wave: Wave) -> Self {
        let counter = wave.counter.clone();

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

        self.counter.step_with_rate(rate);

        if !self.counter.is_active() {
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
            2 => amp >> 1,
            3 => amp >> 2,
            _ => unreachable!(),
        };

        amp as u16
    }

    fn on(&self) -> bool {
        self.counter.is_active()
    }
}
