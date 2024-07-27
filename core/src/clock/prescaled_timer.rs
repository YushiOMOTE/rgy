use super::{ClockDivider, Timer};

pub struct PrescaledTimer {
    timer: Timer,
    divider: ClockDivider,
}

impl PrescaledTimer {
    fn new(enable: bool, target_clock_rate: usize) -> Self {
        Self {
            timer: Timer::new(enable),
            divider: ClockDivider::new(target_clock_rate),
        }
    }

    /// Timer starts counting.
    pub fn enable(&mut self) {
        self.timer.enable();
    }

    /// Timer stops counting.
    pub fn disable(&mut self) {
        self.timer.disable();
    }

    /// Update the interval (the number of ticks) until the timer expires resetting the current counter.
    pub fn set_interval(&mut self, interval: usize) {
        self.timer.set_interval(interval);
    }

    /// Update the frequency of ticks.
    pub fn set_frequency(&mut self, frequency: usize) {
        self.divider.set_target_clock_rate(frequency);
    }

    /// Update the source clock rate.
    pub fn set_source_clock_rate(&mut self, rate: usize) {
        self.divider.set_source_clock_rate(rate);
    }

    /// Get the remaining ticks until the timer expires next.
    pub fn expires_in(&self) -> usize {
        self.timer.expires_in()
    }

    /// Updates the timer state given `cycles` that counts at source clock rate.
    /// Returns `true` every `interval` ticks ticked at `frequency`.
    pub fn step(&mut self, cycles: usize) -> bool {
        if !self.divider.step_one(cycles) {
            return false;
        }

        self.timer.tick()
    }

    /// Reset the counter without disabling the timer.
    pub fn reset(&mut self) {
        self.timer.reset();
    }
}
