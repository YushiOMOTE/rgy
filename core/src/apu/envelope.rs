use crate::clock::Timer;

use super::frame_sequencer::Frame;

#[derive(Debug, Clone)]
pub struct Envelope {
    amp: usize,
    increase: bool,
    timer: Timer,
    interval: usize,
    force_tick: bool,
    active: bool,
}

impl Envelope {
    pub fn new() -> Self {
        Self {
            amp: 0,
            increase: false,
            timer: Timer::enabled(),
            interval: 0,
            force_tick: false,
            active: false,
        }
    }

    pub fn trigger(&mut self, amp: usize, interval: usize, increase: bool) {
        self.amp = amp;
        self.increase = increase;
        self.interval = interval;

        self.reload_timer();

        self.active = true;
    }

    pub fn reload(&mut self, _amp: usize, interval: usize, increase: bool) {
        if !self.active {
            return;
        }

        // Definition of "enabled" here is non-zero interval.
        let old_enabled = self.interval != 0;
        let new_enabled = interval != 0;

        // The envelope speed can be changed
        // while it's active, and the change takes effect after
        // the next time it ticks.
        self.interval = interval;

        if !old_enabled && new_enabled {
            // Enabling the envelope takes effect instantly.
            self.reload_timer();

            // For simulating the APU bug (see below)
            self.force_tick = true;
        } else if old_enabled && !new_enabled {
            // Disabling the envelope takes effect instantly.
            self.reload_timer();

            self.force_tick = false;
        }

        // Zombie mode effect (1)
        // If the old envelope period was zero and the envelope is still doing automatic updates, volume is incremented by 1,
        // otherwise if the envelope was in subtract mode, volume is incremented by 2.
        if !old_enabled && self.amp < 15 && self.increase {
            self.increase_amp();
        }

        // Zombie mode effect (2)
        // If the mode was changed (add to subtract or subtract to add), volume is set to 16-volume.
        if self.increase != increase {
            self.amp = 15;
            self.increase = increase;
        }
    }

    pub fn step(&mut self, frame: Frame) {
        match frame.switched() {
            Some(7) => {}
            Some(1) | Some(3) | Some(5) if self.force_tick => {
                // Enabling the envelope trigger an APU bug - in the next
                // *even* DIV-APU tick, the APU will tick the volume
                // envelope of that apropriate channel, even if it would
                // not tick volume envelope at that tick otherwise
                self.tick();

                return;
            }
            _ => return,
        }

        if !self.timer.tick() {
            return;
        }

        self.tick();
    }

    pub fn amp(&self) -> usize {
        self.amp
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    fn tick(&mut self) {
        self.force_tick = false;

        self.reload_timer();

        self.update_amp();
    }

    fn update_amp(&mut self) {
        if self.increase {
            self.increase_amp();
        } else {
            self.decrease_amp();
        }
    }

    fn increase_amp(&mut self) {
        self.amp = self.amp.saturating_add(1).min(15);
    }

    fn decrease_amp(&mut self) {
        self.amp = self.amp.saturating_sub(1);
    }

    fn reload_timer(&mut self) {
        self.timer.reset();
        self.timer.set_interval(self.interval);
    }

    pub fn power_off(&mut self) {
        self.active = false;
    }
}
