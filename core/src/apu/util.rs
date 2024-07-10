use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use log::*;

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
    length: usize,
    rate: usize,
    base: usize,
    count: usize,
    freeze: bool,
}

impl Counter {
    fn new(base: usize) -> Self {
        Self {
            enable: false,
            active: false,
            length: 0,
            rate: 4_194_304,
            base,
            count: 0,
            freeze: false,
        }
    }

    pub fn type64() -> Self {
        Self::new(64)
    }

    pub fn type256() -> Self {
        Self::new(256)
    }

    // trigger, enable, freeze

    pub fn update(&mut self, trigger: bool, enable: bool) {
        debug!(
            "trigger={}, enable={}: {:p}: {:?}",
            trigger, enable, self, self
        );

        // Conditions to clock when enabled.
        let in_first_half = self.count <= self.length_period() / 2; // First half
        let disabled_to_enabled = !self.enable && enable; // TODO: GB only. CGB has a different condition.
        let clock_by_enable = disabled_to_enabled && in_first_half;
        let freeze_by_enable = if clock_by_enable {
            self.length == 1
        } else {
            false
        };

        if clock_by_enable {
            debug!("clock by enable");
            self.clock();
        }

        if freeze_by_enable {
            debug!("freeze by enable");
            self.freeze = true;
        } else if !trigger && enable {
            debug!("unfreeze by enable");
            self.freeze = false;
        }

        if trigger && self.length == 0 {
            self.length = self.base;
        }

        if trigger {
            if self.freeze && enable {
                debug!("clock by trigger");
                self.clock();
            }
            self.freeze = false;
        }

        // Trigger activates counting.
        if trigger {
            self.active = true;
        }

        self.enable = enable;

        if self.length == 0 {
            // If the clock makes length zero, should deactivate
            self.active = false;
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn load(&mut self, value: usize) {
        self.length = self.base - value;
    }

    /// Called in the OS thread with sampling rate
    pub fn step_with_rate(&mut self, rate: usize) {
        self.rate = rate;
        self.step(1);
    }

    pub fn step(&mut self, count: usize) {
        self.count += count;

        let period = self.length_period();
        if self.count >= period {
            self.count -= period;

            if self.enable {
                // Disabling length should stop length clocking
                self.clock();

                if self.length == 0 {
                    // Timeout de-activates the channel
                    self.active = false;
                }
            }
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    fn clock(&mut self) {
        self.length = self.length.saturating_sub(1);
    }

    fn length_period(&self) -> usize {
        self.rate / 256
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
