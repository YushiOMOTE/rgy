#[derive(Debug, Clone)]
pub struct ClockDivider {
    counter: usize,
    source_clock_rate: usize,
    target_clock_rate: usize,
}

impl ClockDivider {
    pub fn new(source_clock_rate: usize, target_clock_rate: usize) -> Self {
        Self {
            counter: 0,
            source_clock_rate,
            target_clock_rate,
        }
    }

    pub fn set_source_clock_rate(&mut self, source_clock_rate: usize) {
        self.source_clock_rate = source_clock_rate;
    }

    pub fn step(&mut self, cycles: usize) -> usize {
        self.counter += cycles * self.target_clock_rate;

        let times = self.counter / self.source_clock_rate;

        self.counter = self.counter % self.source_clock_rate;

        times
    }

    pub fn reset(&mut self) {
        self.counter = 0;
    }
}

#[test]
fn test_clock_divider() {
    // 2 Hz -> 1 Hz
    let mut divider = ClockDivider::new(2, 1);

    // 4 ticks -> 2 ticks
    assert_eq!(divider.step(1), 0);
    assert_eq!(divider.step(1), 1);
    assert_eq!(divider.step(1), 0);
    assert_eq!(divider.step(1), 1);

    divider.set_source_clock_rate(4);

    // 8 ticks -> 2 ticks
    assert_eq!(divider.step(1), 0);
    assert_eq!(divider.step(1), 0);
    assert_eq!(divider.step(1), 0);
    assert_eq!(divider.step(1), 1);
    assert_eq!(divider.step(1), 0);
    assert_eq!(divider.step(1), 0);
    assert_eq!(divider.step(1), 0);
    assert_eq!(divider.step(1), 1);
}
