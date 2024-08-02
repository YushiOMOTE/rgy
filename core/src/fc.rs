use crate::hardware::HardwareHandle;
use crate::system::Config;

pub struct FreqControl {
    hw: HardwareHandle,
    last: u64,
    refill_interval: u64,
    target_freq: u64,
    remaining_cycles: u64,
}

impl FreqControl {
    pub fn new(hw: HardwareHandle, cfg: &Config) -> Self {
        Self {
            hw,
            last: 0,
            refill_interval: cfg.rate_limit_interval,
            target_freq: cfg.freq,
            remaining_cycles: 0,
        }
    }

    pub fn reset(&mut self) {
        self.last = self.hw.get().borrow_mut().clock();
    }

    pub fn adjust(&mut self, time: usize) {
        let consumed_cycles = time as u64;

        self.try_fill();

        while self.remaining_cycles < consumed_cycles {
            self.try_fill();
        }

        self.remaining_cycles -= consumed_cycles;
    }

    fn try_fill(&mut self) {
        let now = self.hw.get().borrow_mut().clock();

        let diff = now.saturating_sub(self.last);

        if diff >= self.refill_interval {
            self.last = now;

            self.remaining_cycles += self.target_freq * diff / 1_000_000;
        }
    }
}
