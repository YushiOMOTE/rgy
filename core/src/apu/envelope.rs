use crate::clock::Timer;

#[derive(Debug, Clone)]
pub struct Envelope {
    amp: usize,
    inc: bool,
    timer: Timer,
}

impl Envelope {
    pub fn new() -> Self {
        Self {
            amp: 0,
            inc: false,
            timer: Timer::enabled(),
        }
    }

    pub fn update(&mut self, amp: usize, count: usize, inc: bool) {
        self.amp = amp;
        self.inc = inc;
        self.timer.reset();
        self.timer.set_interval(count);
    }

    pub fn step(&mut self, frame: Option<usize>) {
        match frame {
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

    pub fn amp(&self) -> usize {
        self.amp
    }
}
