use super::{frame_sequencer::FrameSequencer, timer::Timer};
use log::*;

#[derive(Clone, Debug)]
pub struct Sweep {
    enable: bool,
    disabling_channel: bool,
    frame_sequencer: FrameSequencer,
    freq: usize,
    timer: Timer,
    subtract: bool,
    subtracted: bool,
    period: usize,
    shift: usize,
}

impl Sweep {
    pub fn new() -> Self {
        Self {
            enable: false,
            frame_sequencer: FrameSequencer::new(4_194_304),
            freq: 0,
            timer: Timer::new(),
            subtract: false,
            period: 0,
            shift: 0,
            subtracted: false,
            disabling_channel: false,
        }
    }

    pub fn disabling_channel(&self) -> bool {
        self.disabling_channel
    }

    pub fn freq(&self) -> usize {
        self.freq
    }

    pub fn trigger(&mut self, freq: usize, period: usize, subtract: bool, shift: usize) {
        self.freq = freq;
        self.enable = period > 0 || shift > 0;
        self.disabling_channel = false;
        self.period = period;
        self.shift = shift;
        self.subtract = subtract;
        self.subtracted = false;

        self.reload_timer();

        debug!("trigger: {:x?}", self);

        if self.shift > 0 {
            // If shift is non-zero, calucation and overflow checks again on trigger
            // discarding the new frequency
            // self.subtracted = self.subtract;
            self.calculate();
        }
    }

    pub fn update_params(&mut self, period: usize, subtract: bool, shift: usize) {
        debug!("update period/shift {}/{}, {:?}", period, shift, self);

        // Ending subtraction mode after calculation with subtraction disables the channel.
        if self.subtracted && self.subtract && !subtract {
            self.disable();
        }

        self.period = period;
        self.shift = shift;
        self.subtract = subtract;
    }

    pub fn step_with_rate(&mut self, rate: usize) {
        self.frame_sequencer.set_source_clock_rate(rate);
        self.step(1);
    }

    pub fn step(&mut self, cycles: usize) -> Option<usize> {
        match self.frame_sequencer.step(cycles) {
            Some(2) | Some(6) => {}
            _ => return None,
        }

        if !self.enable {
            return None;
        }

        if !self.timer.tick() {
            return None;
        }
        self.reload_timer();

        // Calculation happens only when period > 0
        if self.period == 0 {
            return None;
        }

        let new_freq = self.calculate();

        // Frequency update happens only when shift > 0
        if self.shift > 0 {
            self.freq = new_freq;

            // Calculation and overflow check actually happens AGAIN
            // but discarding the new frequency
            self.calculate();
        }

        Some(self.freq)
    }

    fn calculate(&mut self) -> usize {
        let new_freq = if self.subtract {
            // This it to detect subtract mode ends after subtraction
            // to disable channel.
            self.subtracted = true;

            self.freq.wrapping_sub(self.freq >> self.shift)
        } else {
            self.freq.wrapping_add(self.freq >> self.shift)
        };

        if new_freq >= 2048 {
            self.disable();
            self.freq
        } else {
            new_freq
        }
    }

    fn reload_timer(&mut self) {
        self.timer
            .set_interval(if self.period == 0 { 8 } else { self.period });
    }

    fn disable(&mut self) {
        self.enable = false;
        self.disabling_channel = true;
    }
}
