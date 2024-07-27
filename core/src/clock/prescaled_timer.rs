use super::{ClockDivider, Timer};

#[derive(Debug, Clone)]
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

    /// Get the current counter.
    pub fn counter(&self) -> usize {
        self.timer.counter()
    }

    /// Update the counter.
    pub fn set_counter(&mut self, counter: usize) {
        self.timer.set_counter(counter);
    }

    /// Updates the timer state given `cycles` that counts at source clock rate.
    /// Returns `true` every `interval` ticks ticked at `frequency`.
    pub fn step(&mut self, cycles: usize) -> bool {
        if !self.divider.step_one(cycles) {
            return false;
        }

        self.timer.tick()
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
        self.timer.disable();
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

    pub fn build(self) -> PrescaledTimer {
        self.timer
    }
}
