use log::*;

#[derive(Clone, Debug)]
pub struct LengthCounter {
    enable: bool,
    active: bool,
    length: usize,
    base: usize,
    first_half: bool,
}

impl LengthCounter {
    fn new(base: usize) -> Self {
        Self {
            enable: false,
            active: false,
            length: 0,
            base,
            first_half: false,
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
            if !self.enable && self.first_half {
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
                if self.enable && self.first_half {
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
        self.first_half = false;
    }

    pub fn power_off(&mut self) {
        self.enable = false;
        self.active = false;
    }

    pub fn load(&mut self, value: usize) {
        self.length = self.base - value;
    }

    pub fn step(&mut self, step: Option<usize>) {
        match step {
            Some(0) | Some(2) | Some(4) | Some(6) => {
                self.first_half = true;
            }
            Some(1) | Some(3) | Some(5) | Some(7) => {
                self.first_half = false;
                return;
            }
            _ => return,
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
