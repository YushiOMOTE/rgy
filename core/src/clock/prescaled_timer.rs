use super::{ClockDivider, Timer};

pub struct PrescaledTimer {
    timer: Timer,
    divider: ClockDivider,
}

impl PrescaledTimer {
    pub fn builder() -> PrescaledTimerBuilder {
        PrescaledTimerBuilder { timer: Self::new() }
    }

    pub fn new() -> Self {
        Self {
            timer: Timer::enabled(),
            divider: ClockDivider::new(1),
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

    /// Get the current counter.
    pub fn counter(&self) -> usize {
        self.timer.counter()
    }

    pub fn set_counter(&mut self, counter: usize) {
        self.timer.set_counter(counter);
    }

    /// Get the remaining ticks until the timer expires next.
    pub fn remaining(&self) -> usize {
        self.timer.remaining()
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

pub struct PrescaledTimerBuilder {
    timer: PrescaledTimer,
}

impl PrescaledTimerBuilder {
    pub fn enable(mut self) -> Self {
        self.timer.enable();
        self
    }

    pub fn disable(mut self) -> Self {
        self.timer.enable();
        self
    }

    pub fn frequency(mut self, frequency: usize) -> Self {
        self.timer.set_frequency(frequency);
        self
    }

    pub fn interval(mut self, interval: usize) -> Self {
        self.timer.set_interval(interval);
        self
    }

    pub fn source_clock_rate(mut self, rate: usize) -> Self {
        self.timer.set_source_clock_rate(rate);
        self
    }

    pub fn build(self) -> PrescaledTimer {
        self.timer
    }
}
