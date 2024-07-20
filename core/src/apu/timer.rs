#[derive(Debug, Clone)]
pub struct Timer {
    counter: usize,
    interval: usize,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            counter: 0,
            interval: 0,
        }
    }

    pub fn set_interval(&mut self, interval: usize) {
        self.interval = interval;
    }

    pub fn remaining(&self) -> usize {
        self.interval.saturating_sub(self.counter)
    }

    pub fn tick(&mut self) -> bool {
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
}

#[test]
fn test_timer_interval() {
    let mut timer = Timer::new();

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
