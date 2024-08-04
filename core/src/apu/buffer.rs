use alloc::{sync::Arc, vec, vec::Vec};
use log::*;
use spin::Mutex;

struct RingBuffer<T> {
    events: Vec<T>,
    reader: usize,
    writer: usize,
}

impl<T: Copy + Default> RingBuffer<T> {
    fn new(size: usize) -> Self {
        Self {
            events: vec![T::default(); size],
            reader: 0,
            writer: 0,
        }
    }

    fn push(&mut self, event: T) -> bool {
        self.events[self.writer] = event;
        let next_writer = (self.writer + 1) % self.events.len();
        if next_writer == self.reader {
            return false;
        }
        self.writer = next_writer;

        true
    }

    fn pop(&mut self) -> Option<T> {
        if self.reader == self.writer {
            None
        } else {
            let event = self.events[self.reader];
            self.reader = (self.reader + 1) % self.events.len();
            Some(event)
        }
    }
}

#[test]
fn test_ring_buffer() {
    let mut b = RingBuffer::<Frame>::new(10);

    assert!(b.pop().is_none());
    assert!(b.push(Frame::new(1, 0)));
    assert_eq!(b.pop().unwrap().cycles, 1);
    assert!(b.pop().is_none());
    assert!(b.push(Frame::new(2, 0)));
    assert!(b.push(Frame::new(3, 0)));
    assert_eq!(b.pop().unwrap().cycles, 2);
    assert_eq!(b.pop().unwrap().cycles, 3);
    assert!(b.pop().is_none());
    assert!(b.push(Frame::new(4, 0)));
    assert_eq!(b.pop().unwrap().cycles, 4);
    assert!(b.pop().is_none());
}

#[test]
fn test_ring_buffer_overflow() {
    let mut b = RingBuffer::<Frame>::new(10);

    for i in 0..9 {
        assert!(b.push(Frame::new(i, 0)));
    }

    assert!(!b.push(Frame::new(9, 0)));

    for i in 0..9 {
        assert_eq!(b.pop().unwrap().cycles, i);
    }

    assert!(b.pop().is_none());
}

#[derive(Clone)]
pub struct SharedRingBuffer<T> {
    inner: Arc<Mutex<RingBuffer<T>>>,
}

impl<T: Copy + Default> SharedRingBuffer<T> {
    fn new(size: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(RingBuffer::new(size))),
        }
    }

    fn push(&self, event: T) -> bool {
        self.inner.lock().push(event)
    }

    fn pop(&self) -> Option<T> {
        self.inner.lock().pop()
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Frame {
    cycles: usize,
    diff: isize,
}

impl Frame {
    fn new(cycles: usize, diff: isize) -> Self {
        Self { cycles, diff }
    }
}

pub struct Producer {
    cycles: usize,
    amp: isize,
    buf: SharedRingBuffer<Frame>,
    max_frame_size: usize,
}

impl Producer {
    fn new(buf: SharedRingBuffer<Frame>, max_frame_size: usize) -> Self {
        Self {
            amp: 0,
            cycles: 0,
            buf,
            max_frame_size,
        }
    }

    pub fn add_sample(&mut self, cycles: usize, amp: isize) {
        assert!(cycles > 0);

        self.cycles = self.cycles.saturating_add(cycles);

        if self.amp != amp || self.cycles >= self.max_frame_size {
            if self.buf.push(Frame::new(self.cycles, amp - self.amp)) {
                self.cycles = 0;
                self.amp = amp;
            } else {
                warn!("Ring buffer overflow. Skip a sample");
            }
        }
    }
}

#[derive(Clone)]
pub struct Consumer {
    amp: isize,
    cycles: usize,
    next: Option<Frame>,
    buf: SharedRingBuffer<Frame>,
}

impl Consumer {
    fn new(buf: SharedRingBuffer<Frame>) -> Self {
        Self {
            amp: 0,
            cycles: 0,
            next: None,
            buf,
        }
    }

    pub fn get_sample(&mut self, cycles: usize) -> isize {
        assert!(cycles > 0);

        if self.next.is_none() {
            self.next = self.buf.pop();

            if self.next.is_none() {
                // No new frame. Wait for a new frame arrival.
                // Keep the last amp until the new frame arrives.
                return self.amp;
            }
        }

        self.cycles += cycles;

        while self.next.is_some() {
            match self.next.as_mut() {
                Some(e) if self.cycles >= e.cycles => {
                    self.cycles -= e.cycles;
                    self.amp += e.diff;
                    self.next = self.buf.pop();
                }
                Some(e) => {
                    e.cycles -= self.cycles;
                    self.cycles = 0;
                    break;
                }
                None => break,
            }
        }

        self.amp
    }
}

pub fn open_buffer(buf_size: usize, max_frame_size: usize) -> (Producer, Consumer) {
    let buf = SharedRingBuffer::new(buf_size);

    (
        Producer::new(buf.clone(), max_frame_size),
        Consumer::new(buf),
    )
}

#[test]
fn test_stream() {
    let (mut p, mut c) = open_buffer(10, 10);

    p.add_sample(1, 5);
    p.add_sample(1, 6);
    p.add_sample(1, 7);
    p.add_sample(1, 8);
    p.add_sample(1, 7);
    p.add_sample(1, 6);

    assert_eq!(c.get_sample(1), 5);
    assert_eq!(c.get_sample(1), 6);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 8);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 6);
}

#[test]
fn test_stream_consume_by_2() {
    let (mut p, mut c) = open_buffer(10, 10);

    p.add_sample(1, 5);
    p.add_sample(1, 6);
    p.add_sample(1, 7);
    p.add_sample(1, 8);
    p.add_sample(1, 7);
    p.add_sample(1, 6);

    assert_eq!(c.get_sample(2), 6);
    assert_eq!(c.get_sample(2), 8);
    assert_eq!(c.get_sample(2), 6);
}

#[test]
fn test_stream_consume_by_3() {
    let (mut p, mut c) = open_buffer(10, 10);

    p.add_sample(1, 5);
    p.add_sample(1, 6);
    p.add_sample(1, 7);
    p.add_sample(1, 8);
    p.add_sample(1, 7);
    p.add_sample(1, 6);

    assert_eq!(c.get_sample(3), 7);
    assert_eq!(c.get_sample(3), 6);
}

#[test]
fn test_stream_produce_4_consume_1() {
    let (mut p, mut c) = open_buffer(50, 50);

    p.add_sample(4, 5);
    p.add_sample(4, 6);
    p.add_sample(4, 7);
    p.add_sample(4, 8);
    p.add_sample(4, 7);
    p.add_sample(4, 6);

    assert_eq!(c.get_sample(1), 0);
    assert_eq!(c.get_sample(1), 0);
    assert_eq!(c.get_sample(1), 0);
    assert_eq!(c.get_sample(1), 5);
    assert_eq!(c.get_sample(1), 5);
    assert_eq!(c.get_sample(1), 5);
    assert_eq!(c.get_sample(1), 5);
    assert_eq!(c.get_sample(1), 6);
    assert_eq!(c.get_sample(1), 6);
    assert_eq!(c.get_sample(1), 6);
    assert_eq!(c.get_sample(1), 6);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 8);
    assert_eq!(c.get_sample(1), 8);
    assert_eq!(c.get_sample(1), 8);
    assert_eq!(c.get_sample(1), 8);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 6);
    assert_eq!(c.get_sample(1), 6);
    assert_eq!(c.get_sample(1), 6);
    assert_eq!(c.get_sample(1), 6);
}

#[test]
fn test_stream_produce_4_consume_2() {
    let (mut p, mut c) = open_buffer(50, 50);

    p.add_sample(4, 5);
    p.add_sample(4, 6);
    p.add_sample(4, 7);
    p.add_sample(4, 8);
    p.add_sample(4, 7);
    p.add_sample(4, 6);

    assert_eq!(c.get_sample(2), 0);
    assert_eq!(c.get_sample(2), 5);
    assert_eq!(c.get_sample(2), 5);
    assert_eq!(c.get_sample(2), 6);
    assert_eq!(c.get_sample(2), 6);
    assert_eq!(c.get_sample(2), 7);
    assert_eq!(c.get_sample(2), 7);
    assert_eq!(c.get_sample(2), 8);
    assert_eq!(c.get_sample(2), 8);
    assert_eq!(c.get_sample(2), 7);
    assert_eq!(c.get_sample(2), 7);
    assert_eq!(c.get_sample(2), 6);
    assert_eq!(c.get_sample(2), 6);
}

#[test]
fn test_stream_underrun() {
    let (mut p, mut c) = open_buffer(10, 10);

    assert_eq!(c.get_sample(1), 0);
    assert_eq!(c.get_sample(1), 0);
    assert_eq!(c.get_sample(1), 0);

    p.add_sample(1, 5);
    p.add_sample(1, 6);
    p.add_sample(1, 7);
    p.add_sample(1, 8);
    p.add_sample(1, 7);
    p.add_sample(1, 6);

    assert_eq!(c.get_sample(1), 5);
    assert_eq!(c.get_sample(1), 6);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 8);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 6);
}

#[test]
fn test_stream_underrun_in_middle() {
    let (mut p, mut c) = open_buffer(10, 10);

    p.add_sample(1, 5);
    p.add_sample(1, 6);
    p.add_sample(1, 7);

    assert_eq!(c.get_sample(1), 5);
    assert_eq!(c.get_sample(1), 6);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 7);

    p.add_sample(1, 8);
    p.add_sample(1, 7);
    p.add_sample(1, 6);

    assert_eq!(c.get_sample(1), 8);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 6);
}

#[test]
fn test_stream_empty() {
    let (_, mut c) = open_buffer(10, 10);

    assert_eq!(c.get_sample(1), 0);
    assert_eq!(c.get_sample(1), 0);
    assert_eq!(c.get_sample(1), 0);
    assert_eq!(c.get_sample(1), 0);
    assert_eq!(c.get_sample(1), 0);
    assert_eq!(c.get_sample(1), 0);
}

#[test]
fn test_stream_same_sound() {
    let (mut p, mut c) = open_buffer(10, 10);

    p.add_sample(1, 5);
    p.add_sample(1, 5);
    p.add_sample(1, 6);
    p.add_sample(1, 6);
    p.add_sample(1, 6);
    p.add_sample(1, 6);
    p.add_sample(1, 7);
    p.add_sample(1, 7);
    p.add_sample(1, 7);
    p.add_sample(1, 7);
    p.add_sample(1, 7);
    p.add_sample(1, 7);
    p.add_sample(1, 8);
    p.add_sample(1, 7);
    p.add_sample(1, 6);
    p.add_sample(1, 6);

    assert_eq!(c.get_sample(1), 5);
    assert_eq!(c.get_sample(1), 5);
    assert_eq!(c.get_sample(1), 6);
    assert_eq!(c.get_sample(1), 6);
    assert_eq!(c.get_sample(1), 6);
    assert_eq!(c.get_sample(1), 6);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 8);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 6);
    assert_eq!(c.get_sample(1), 6);
}

#[test]
fn test_stream_same_sound_small_max_frame_size() {
    let (mut p, mut c) = open_buffer(10, 2);

    p.add_sample(1, 5);
    p.add_sample(1, 5);
    p.add_sample(1, 6);
    p.add_sample(1, 6);
    p.add_sample(1, 6);
    p.add_sample(1, 6);
    p.add_sample(1, 7);
    p.add_sample(1, 7);
    p.add_sample(1, 7);
    p.add_sample(1, 7);
    p.add_sample(1, 7);
    p.add_sample(1, 7);
    p.add_sample(1, 8);
    p.add_sample(1, 7);
    p.add_sample(1, 6);
    p.add_sample(1, 6);

    assert_eq!(c.get_sample(1), 5);
    assert_eq!(c.get_sample(1), 5);
    assert_eq!(c.get_sample(1), 6);
    assert_eq!(c.get_sample(1), 6);
    assert_eq!(c.get_sample(1), 6);
    assert_eq!(c.get_sample(1), 6);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 8);
    assert_eq!(c.get_sample(1), 7);
    assert_eq!(c.get_sample(1), 6);
    assert_eq!(c.get_sample(1), 6);
}
