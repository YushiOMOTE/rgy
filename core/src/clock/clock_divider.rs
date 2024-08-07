use crate::cpu::CPU_FREQ_HZ;

#[derive(Debug, Clone)]
pub struct ClockDivider {
    counter: usize,
    source_clock_rate: usize,
    target_clock_rate: usize,
}

impl ClockDivider {
    pub fn new(target_clock_rate: usize) -> Self {
        Self {
            counter: 0,
            source_clock_rate: CPU_FREQ_HZ,
            target_clock_rate,
        }
    }

    pub fn set_target_clock_rate(&mut self, target_clock_rate: usize) {
        self.target_clock_rate = target_clock_rate;
    }

    pub fn step(&mut self, cycles: usize) -> usize {
        self.counter += cycles * self.target_clock_rate;

        let times = self.counter / self.source_clock_rate;

        self.counter %= self.source_clock_rate;

        times
    }

    pub fn step_one(&mut self, cycles: usize) -> bool {
        let times = self.step(cycles);
        assert!(times <= 1);
        times == 1
    }

    pub fn reset(&mut self) {
        self.counter = 0;
    }
}
