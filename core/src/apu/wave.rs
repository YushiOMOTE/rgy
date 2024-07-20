use alloc::sync::Arc;
use core::sync::atomic::AtomicUsize;

use log::*;

use crate::hardware::Stream;

use super::{
    clock_divider::ClockDivider,
    length_counter::LengthCounter,
    timer::Timer,
    util::AtomicHelper,
    wave_buf::{WaveIndex, WaveRam},
};

#[derive(Debug, Clone)]
pub struct Wave {
    enable: bool,
    amp: u8,
    amp_shift: Arc<AtomicUsize>,
    counter: LengthCounter,
    freq: Arc<AtomicUsize>,
    freq_high: u8,
    wave_ram: WaveRam,
    next_index: usize,
    dac: bool,
    divider: ClockDivider,
    timer: Timer,
    fetching: bool,
}

impl Wave {
    pub fn new() -> Self {
        Self {
            enable: false,
            amp: 0,
            amp_shift: Arc::new(AtomicUsize::new(0)),
            counter: LengthCounter::type256(),
            freq: Arc::new(AtomicUsize::new(0)),
            freq_high: 0,
            wave_ram: WaveRam::new(),
            next_index: 0,
            dac: false,
            divider: ClockDivider::new(4_194_304, 2_097_152),
            timer: Timer::new(),
            fetching: false,
        }
    }

    /// Read NR30 register (0xff1a)
    pub fn read_enable(&self) -> u8 {
        if self.dac {
            0xff
        } else {
            0x7f
        }
    }

    /// Write NR30 register (0xff1a)
    pub fn write_enable(&mut self, value: u8) {
        self.dac = value & 0x80 != 0;
        if !self.dac {
            self.enable = false;
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
        info!("freq low: {:02x}", value);
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
        if self.dac && trigger {
            self.enable = true;

            self.reload_timer(1);

            info!("freq: {:04x}/{:?}", self.freq.get(), self.timer);
            self.fetching = true;
            self.next_index = 0;
            // self.index = 0;
        }
        trigger
    }

    /// Read wave pattern buffer
    pub fn read_wave_buf(&self, offset: u16) -> u8 {
        info!(
            "read wave buf: {} ({})",
            self.index(),
            self.timer.remaining()
        );
        let ram_index = offset - 0xff30;
        let fetching_ram_index = (self.index() / 2) as u16;
        let final_index = if self.is_active() {
            if self.fetching {
                fetching_ram_index
            } else {
                return 0xff;
            }
        } else {
            ram_index
        };
        self.wave_ram.read_byte(final_index)
    }

    /// Write wave pattern buffer
    pub fn write_wave_buf(&mut self, offset: u16, value: u8) {
        self.wave_ram.write_byte(offset - 0xff30, value);
    }

    /// Create stream from the current data
    pub fn create_stream(&self) -> WaveStream {
        WaveStream::new(self.clone())
    }

    pub fn step(&mut self, cycles: usize) {
        self.counter.step(cycles);

        let times = self.divider.step(cycles);

        for _ in 0..times {
            self.update();
        }
    }

    fn update(&mut self) {
        if !self.enable {
            return;
        }

        if !self.timer.tick() {
            return;
        }

        self.reload_timer(0);

        self.next_index = (self.next_index + 1) % self.wave_ram.waveform_length();

        let amp = self.wave_ram.read_waveform(self.index());

        let _amp = match self.amp_shift.get() {
            0 => 0,
            1 => amp,
            2 => amp >> 1,
            3 => amp >> 2,
            _ => unreachable!(),
        };
    }

    fn index(&self) -> usize {
        (self.next_index.wrapping_sub(1)) % self.wave_ram.waveform_length()
    }

    fn reload_timer(&mut self, delay: usize) {
        self.timer.set_interval(2048 - self.freq.get() + delay);
    }

    pub fn is_active(&self) -> bool {
        self.counter.is_active() && self.dac
    }

    pub fn clear(&mut self) {
        let mut wave = Wave::new();
        core::mem::swap(&mut wave.wave_ram, &mut self.wave_ram);
        core::mem::swap(self, &mut wave);
    }
}

pub struct WaveStream {
    wave: Wave,
    counter: LengthCounter,
    index: WaveIndex,
}

impl WaveStream {
    fn new(wave: Wave) -> Self {
        let counter = wave.counter.clone();

        let wave_length = wave.wave_ram.waveform_length();

        Self {
            wave,
            counter,
            index: WaveIndex::new(4_194_304, wave_length),
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

        // let rate = rate as usize;

        // self.counter.step_with_rate(rate);

        // if !self.counter.is_active() {
        //     return 0;
        // }

        // let samples = self.wave.wave_ram.waveform_length();
        // let freq = 65536 / (2048 - self.wave.freq.get());
        // let index_freq = freq * samples;

        // self.index.set_source_clock_rate(rate);
        // self.index.update_index(index_freq);

        // let amp = self.wave.wave_ram.read_waveform(self.index.index());

        // let amp = match self.wave.amp_shift.get() {
        //     0 => 0,
        //     1 => amp,
        //     2 => amp >> 1,
        //     3 => amp >> 2,
        //     _ => unreachable!(),
        // };

        // amp as u16

        0
    }

    fn on(&self) -> bool {
        self.counter.is_active()
    }
}
