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

pub struct Counter {
    can_expire: bool,
    count: usize,
    base: usize,
    clock: usize,
    expired: bool,
}

impl Counter {
    pub fn expired() -> Self {
        Self {
            can_expire: false,
            count: 0,
            base: 0,
            clock: 0,
            expired: true,
        }
    }

    pub fn new(can_expire: bool, count: usize, base: usize) -> Self {
        Self {
            can_expire,
            expired: false,
            count,
            base,
            clock: 0,
        }
    }

    pub fn proceed(&mut self, rate: usize) {
        self.proceed_by(rate, 1);
    }

    pub fn proceed_by(&mut self, rate: usize, count: usize) {
        if !self.can_expire || self.expired {
            return;
        }

        let deadline = rate * (self.base - self.count) / 256;

        if self.clock >= deadline {
            self.expired = true;
        } else {
            self.clock += count;
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expired
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
