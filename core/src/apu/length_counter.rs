use log::*;

use super::frame_sequencer::Frame;

#[derive(Clone, Debug)]
pub struct LengthCounter {
    enable: bool,
    active: bool,
    length: usize,
    base: usize,
    extra_clock: bool,
}

impl LengthCounter {
    fn new(base: usize) -> Self {
        Self {
            enable: false,
            active: false,
            length: 0,
            base,
            extra_clock: false,
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

        if enable {
            // Disabled -> enabled in the first half
            // should clock once on enable
            if !self.enable && self.extra_clock {
                // Clock unless length reaches zero
                if self.length != 0 {
                    self.clock();

                    // If counter reaches zero, should deactivate.
                    // Mark this special case as "freeze"
                    if self.length == 0 {
                        self.active = false;
                    }
                }
            }
        }

        self.enable = enable;

        if trigger {
            // Trigger on zero length loads max
            if self.length == 0 {
                self.length = self.base;

                // Reloading 0 -> max on trigger & enable in the first half
                // should clock once on enable.
                if self.enable && self.extra_clock {
                    self.clock();
                }
            }
            self.active = true;
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn power_on(&mut self) {
        self.extra_clock = false;
    }

    pub fn power_off(&mut self) {
        self.enable = false;
        self.active = false;
    }

    pub fn load(&mut self, value: usize) {
        self.length = self.base - value;
    }

    pub fn step(&mut self, frame: Frame) {
        // Extra clock happens if next frame isn't active.
        self.extra_clock = !can_clock(frame.next);

        if !can_clock(frame.switched()) {
            return;
        }

        if self.enable {
            // Disabling length should stop length clocking
            self.clock();

            if self.length == 0 {
                // Timeout de-activates the channel
                self.active = false;
            }
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    fn clock(&mut self) {
        self.length = self.length.saturating_sub(1);
    }
}

fn can_clock(frame_index: Option<usize>) -> bool {
    match frame_index {
        Some(0) | Some(2) | Some(4) | Some(6) => true,
        _ => false,
    }
}
