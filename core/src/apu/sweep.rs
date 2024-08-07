use crate::clock::Timer;

use log::*;

use super::frame_sequencer::Frame;

#[derive(Clone, Debug)]
pub struct Sweep {
    disabling_channel: bool,
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
            freq: 0,
            timer: Timer::disabled(),
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

    pub fn trigger(&mut self, freq: usize, period: usize, subtract: bool, shift: usize) {
        self.freq = freq;
        self.disabling_channel = false;
        self.period = period;
        self.shift = shift;
        self.subtract = subtract;
        self.subtracted = false;

        if period > 0 || shift > 0 {
            self.timer.enable();
        } else {
            self.timer.disable();
        }

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

    pub fn step(&mut self, frame: Frame) -> Option<usize> {
        match frame.switched() {
            Some(2) | Some(6) => {}
            _ => return None,
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
        self.timer.reset();
        self.timer
            .set_interval(if self.period == 0 { 8 } else { self.period });
    }

    fn disable(&mut self) {
        self.timer.disable();
        self.disabling_channel = true;
    }

    pub fn power_on(&mut self) {}

    pub fn power_off(&mut self) {
        self.freq = 0;
        self.timer.disable();
        self.timer.reset();
        self.subtract = false;
        self.period = 0;
        self.shift = 0;
        self.subtracted = false;
        self.disabling_channel = false;
    }
}
