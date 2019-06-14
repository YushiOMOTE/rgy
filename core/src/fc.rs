use crate::hardware::HardwareHandle;
use log::*;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct FreqControl {
    hw: HardwareHandle,
    last: u64,
    cycles: usize,
    barrier: AtomicUsize,
    sample: usize,
    delay: usize,
    delay_unit: usize,
    target_freq: usize,
}

impl FreqControl {
    pub fn new(hw: HardwareHandle, target_freq: usize, sample: usize, delay_unit: usize) -> Self {
        Self {
            hw,
            last: 0,
            cycles: 0,
            barrier: AtomicUsize::new(0),
            delay: 0,
            sample,
            delay_unit,
            target_freq,
        }
    }

    pub fn reset(&mut self) {
        self.last = self.hw.get().borrow_mut().clock();
    }

    pub fn adjust(&mut self, time: usize) {
        self.cycles += time;

        for _ in 0..self.delay {
            self.barrier.fetch_add(1, Ordering::Relaxed);
        }

        if self.cycles > self.sample {
            self.cycles -= self.sample;

            let now = self.hw.get().borrow_mut().clock();
            let (diff, of) = now.overflowing_sub(self.last);
            if of || diff == 0 {
                warn!("Overflow: {} - {}", self.last, now);
                self.last = now;
                return;
            }
            // get cycles per second
            let freq = self.sample * 1000_000 / diff as usize;

            debug!("Frequency: {}", freq);

            if freq > self.target_freq {
                self.delay += self.delay_unit;
            } else {
                if self.delay > 0 {
                    self.delay -= self.delay_unit;
                }
            }

            self.last = now;
        }
    }
}
