use crate::{cpu::CPU_FREQ_HZ, Stream};

use super::buffer::{open_buffer, Consumer, Producer};

pub struct SoundSink {
    left: Producer,
    right: Producer,
}

const BUF_SIZE: usize = 16384;
const MAX_FRAME_SIZE: usize = 1024;

pub fn open_stream() -> (SoundSink, SoundStream) {
    let left = open_buffer(BUF_SIZE, MAX_FRAME_SIZE);
    let right = open_buffer(BUF_SIZE, MAX_FRAME_SIZE);

    (
        SoundSink::new(left.0, right.0),
        SoundStream::new(left.1, right.1),
    )
}

impl SoundSink {
    fn new(left: Producer, right: Producer) -> Self {
        Self { left, right }
    }

    pub fn send(&mut self, cycles: usize, amp: (isize, isize)) {
        self.left.add_sample(cycles, amp.0);
        self.right.add_sample(cycles, amp.1);
    }
}

pub struct SoundStream {
    left: Consumer,
    right: Consumer,
    upscaler: UpScaler,
}

impl SoundStream {
    fn new(left: Consumer, right: Consumer) -> Self {
        Self {
            left,
            right,
            upscaler: UpScaler::new(CPU_FREQ_HZ),
        }
    }
}

#[derive(Clone)]
struct UpScaler {
    target_rate: usize,
    count: usize,
}

impl UpScaler {
    fn new(target_rate: usize) -> Self {
        Self {
            target_rate,
            count: 0,
        }
    }

    fn compute_cycles(&mut self, rate: usize) -> usize {
        let mut cycles = 0;

        while self.count < self.target_rate {
            self.count += rate;
            cycles += 1;
        }
        self.count -= self.target_rate;

        cycles
    }
}

impl Stream for SoundStream {
    fn max(&self) -> u16 {
        // Master volume max is 8, we have left and right: 8 * 2
        // Each channel max is 15, 4 channels, left and right: 15 * 4 * 2
        8 * 2 * 15 * 4 * 2
    }

    fn next(&mut self, rate: u32) -> u16 {
        let (left, right) = self.next_dual(rate);
        (left + right) / 2
    }

    fn next_dual(&mut self, rate: u32) -> (u16, u16) {
        let cycles = self.upscaler.compute_cycles(rate as usize);

        let left = self.left.get_sample(cycles);

        let right = self.right.get_sample(cycles);

        let center = (self.max() / 2) as isize;

        ((left + center) as u16 * 2, (right + center) as u16 * 2)
    }

    fn on(&self) -> bool {
        true
    }
}
