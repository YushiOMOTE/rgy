use crate::clock::Timer;

use super::frame_sequencer::FrameSequencer;

#[derive(Debug, Clone)]
pub struct Envelope {
    amp: usize,
    inc: bool,
    frame_sequencer: FrameSequencer,
    timer: Timer,
}

impl Envelope {
    pub fn new() -> Self {
        Self {
            amp: 0,
            inc: false,
            frame_sequencer: FrameSequencer::new(),
            timer: Timer::enabled(),
        }
    }

    pub fn update(&mut self, amp: usize, count: usize, inc: bool) {
        self.amp = amp;
        self.inc = inc;
        self.timer.reset();
        self.timer.set_interval(count);
    }

    pub fn step(&mut self, cycles: usize) {
        match self.frame_sequencer.step(cycles) {
            Some(7) => {}
            _ => return,
        }

        if !self.timer.tick() {
            return;
        }

        self.amp = if self.inc {
            self.amp.saturating_add(1).min(15)
        } else {
            self.amp.saturating_sub(1)
        };
    }

    pub fn step_with_rate(&mut self, rate: usize) {
        self.frame_sequencer.set_source_clock_rate(rate);
        self.step(1);
    }

    pub fn amp(&self) -> usize {
        self.amp
    }
}
