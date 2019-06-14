use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

pub struct FreqControl {
    last: Instant,
    count: usize,
    barrier: AtomicUsize,
    sample: usize,
    delay: usize,
    delay_unit: usize,
    target: usize,
}

impl FreqControl {
    pub fn new(target: usize, sample: usize, delay_unit: usize) -> Self {
        Self {
            last: Instant::now(),
            count: 0,
            barrier: AtomicUsize::new(0),
            delay: 0,
            sample,
            delay_unit,
            target,
        }
    }

    pub fn reset(&mut self) {
        self.last = Instant::now();
    }

    pub fn adjust(&mut self, time: usize) {
        self.count += time;

        for _ in 0..self.delay {
            self.barrier.fetch_add(1, Ordering::Relaxed);
        }

        if self.count > self.sample {
            self.count = self.count - self.sample;

            let now = Instant::now();
            let df = now - self.last;
            let df = df.as_secs() as usize * 1000000 + df.subsec_micros() as usize;
            let ips = self.sample * 1000000 / df;

            if ips > self.target {
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
