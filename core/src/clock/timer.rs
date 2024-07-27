/// Timer that triggers an event every given interval.
#[derive(Debug, Clone)]
pub struct Timer {
    enable: bool,
    counter: usize,
    interval: usize,
}

impl Timer {
    pub fn new(enable: bool) -> Self {
        Self {
            enable,
            counter: 0,
            interval: 0,
        }
    }

    /// Create a enabled timer.
    pub fn enabled() -> Self {
        Self::new(true)
    }

    /// Create a disabled timer.
    pub fn disabled() -> Self {
        Self::new(false)
    }

    /// Timer starts counting.
    pub fn enable(&mut self) {
        self.enable = true;
    }

    /// Timer stops counting.
    pub fn disable(&mut self) {
        self.enable = false;
    }

    /// Update the interval of the timer resetting the current counter.
    pub fn set_interval(&mut self, interval: usize) {
        self.interval = interval;
        self.counter = 0;
    }

    /// Get the remaining ticks until the timer expires next.
    pub fn expires_in(&self) -> usize {
        self.interval.saturating_sub(self.counter)
    }

    /// Advance the counter.
    /// Returns `true` every `interval` ticks.
    pub fn tick(&mut self) -> bool {
        if !self.enable {
            return false;
        }

        if self.interval == 0 {
            return false;
        }

        self.counter += 1;

        if self.counter >= self.interval {
            self.counter = 0;
            true
        } else {
            false
        }
    }

    /// Reset the counter without disabling the timer.
    pub fn reset(&mut self) {
        self.counter = 0;
        self.interval = 0;
    }
}

#[test]
fn test_timer_interval() {
    let mut timer = Timer::enabled();

    timer.set_interval(3);

    assert!(!timer.tick());
    assert!(!timer.tick());
    assert!(timer.tick());
    assert!(!timer.tick());
    assert!(!timer.tick());
    assert!(timer.tick());
    assert!(!timer.tick());
    assert!(!timer.tick());
    assert!(timer.tick());

    timer.set_interval(2);

    assert!(!timer.tick());
    assert!(timer.tick());
    assert!(!timer.tick());
    assert!(timer.tick());
    assert!(!timer.tick());
    assert!(timer.tick());

    timer.set_interval(1);

    assert!(timer.tick());
    assert!(timer.tick());
    assert!(timer.tick());
    assert!(timer.tick());
    assert!(timer.tick());
    assert!(timer.tick());
}

#[test]
fn test_timer_enable_disable() {
    let mut timer = Timer::enabled();

    timer.set_interval(2);

    timer.disable();

    for _ in 0..10 {
        assert!(!timer.tick());
    }

    timer.enable();

    assert!(!timer.tick());
    assert!(timer.tick());
    assert!(!timer.tick());

    timer.disable();

    for _ in 0..10 {
        assert!(!timer.tick());
    }

    timer.enable();

    assert!(timer.tick());
    assert!(!timer.tick());
    assert!(timer.tick());

    timer.disable();

    for _ in 0..10 {
        assert!(!timer.tick());
    }

    timer.enable();

    assert!(!timer.tick());
    assert!(timer.tick());
    assert!(!timer.tick());
    assert!(timer.tick());

    timer = Timer::disabled();

    for _ in 0..10 {
        assert!(!timer.tick());
    }
}

#[test]
fn test_timer_expires_in() {
    let mut timer = Timer::enabled();

    timer.set_interval(3);

    assert_eq!(timer.expires_in(), 3);
    assert!(!timer.tick());
    assert_eq!(timer.expires_in(), 2);
    assert!(!timer.tick());
    assert_eq!(timer.expires_in(), 1);
    assert!(timer.tick());
    assert_eq!(timer.expires_in(), 3);
    assert!(!timer.tick());
    assert_eq!(timer.expires_in(), 2);
    assert!(!timer.tick());
    assert_eq!(timer.expires_in(), 1);
    assert!(timer.tick());
}

#[test]
fn test_timer_zero_interval() {
    let mut timer = Timer::enabled();

    for _ in 0..10 {
        assert!(!timer.tick());
    }

    timer.set_interval(0);

    for _ in 0..10 {
        assert!(!timer.tick());
    }
}

#[test]
fn test_timer_reset() {
    let mut timer = Timer::enabled();

    timer.set_interval(3);

    assert_eq!(timer.expires_in(), 3);
    assert!(!timer.tick());

    timer.reset();

    assert_eq!(timer.expires_in(), 0);

    for _ in 0..10 {
        assert!(!timer.tick());
    }

    assert_eq!(timer.expires_in(), 0);

    timer.set_interval(2);

    assert!(!timer.tick());
    assert!(timer.tick());
    assert!(!timer.tick());
    assert!(timer.tick());
    assert!(!timer.tick());
    assert!(timer.tick());
}
