use log::*;

#[derive(Clone, Debug)]
pub struct LengthCounter {
    enable: bool,
    active: bool,
    length: usize,
    base: usize,
    freeze: bool,
    first_half: bool,
}

impl LengthCounter {
    fn new(base: usize) -> Self {
        Self {
            enable: false,
            active: false,
            length: 0,
            base,
            freeze: false,
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

        // Conditions to clock when enabled.
        let disabled_to_enabled = !self.enable && enable; // TODO: GB only. CGB has a different condition.
        let clock_by_enable = disabled_to_enabled && self.first_half;
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

    pub fn power_on(&mut self) {}

    pub fn power_off(&mut self) {
        self.enable = false;
        self.active = false;
    }

    pub fn load(&mut self, value: usize) {
        self.length = self.base - value;
    }

    pub fn step(&mut self, frame: Option<usize>) {
        match frame {
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
