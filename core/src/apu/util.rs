use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub trait AtomicHelper {
    type Item;

    fn get(&self) -> Self::Item;
    fn set(&self, v: Self::Item);
}

impl AtomicHelper for AtomicUsize {
    type Item = usize;

    fn get(&self) -> Self::Item {
        self.load(Ordering::SeqCst)
    }

    fn set(&self, v: Self::Item) {
        self.store(v, Ordering::SeqCst)
    }
}

impl AtomicHelper for AtomicBool {
    type Item = bool;

    fn get(&self) -> Self::Item {
        self.load(Ordering::SeqCst)
    }

    fn set(&self, v: Self::Item) {
        self.store(v, Ordering::SeqCst)
    }
}

#[derive(Clone, Debug)]
pub struct Counter {
    enable: bool,
    active: bool,
    count: usize,
    base: usize,
    clock: usize,
    expired: bool,
}

impl Counter {
    pub fn new(enable: bool, count: usize, base: usize) -> Self {
        Self {
            enable,
            active: false,
            expired: false,
            count,
            base,
            clock: 0,
        }
    }

    pub fn trigger(&mut self) {
        if self.expired {
            // If expired, set the count back to maximum
            self.count = 0;
            self.clock = 0;
        }
        self.active = true;
        self.expired = false;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn enable(&mut self, enable: bool) {
        self.enable = enable;
    }

    pub fn load(&mut self, count: usize) {
        self.count = count;
        self.clock = 0;
        self.expired = false;
    }

    pub fn proceed(&mut self, rate: usize, count: usize) {
        if !self.enable || self.expired {
            return;
        }

        let deadline = rate * (self.base - self.count) / 256;

        self.clock += count;
        if self.clock >= deadline {
            self.clock = deadline;
            self.expired = true;

            // Timeout de-activates the channel
            self.active = false;
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

pub struct WaveIndex {
    clock: usize,
    index: usize,
}

impl WaveIndex {
    pub fn new() -> Self {
        Self { clock: 0, index: 0 }
    }

    pub fn index(&mut self, rate: usize, freq: usize, max: usize) -> usize {
        self.clock += freq;

        if self.clock >= rate {
            self.clock -= rate;
            self.index = (self.index + 1) % max;
        }

        self.index
    }
}

pub struct Envelop {
    amp: usize,
    count: usize,
    inc: bool,
    clock: usize,
}

impl Envelop {
    pub fn new(amp: usize, count: usize, inc: bool) -> Self {
        Self {
            amp,
            count,
            inc,
            clock: 0,
        }
    }

    pub fn amp(&mut self, rate: usize) -> usize {
        if self.amp == 0 {
            return 0;
        }

        if self.count == 0 {
            return self.amp;
        }

        let interval = rate * self.count / 64;

        self.clock += 1;
        if self.clock >= interval {
            self.clock -= interval;

            self.amp = if self.inc {
                self.amp.saturating_add(1).min(15)
            } else {
                self.amp.saturating_sub(1)
            };
        }

        self.amp
    }
}
