use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::RefCell;

pub const VRAM_WIDTH: usize = 160;
pub const VRAM_HEIGHT: usize = 144;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

#[derive(Clone, Debug)]
pub enum StreamId {
    Tone1,
    Tone2,
    Wave,
    Noise,
}

pub trait Stream: Send + 'static {
    fn next(&mut self, rate: u32) -> u16;
}

#[derive(Clone)]
pub struct HardwareHandle(Rc<RefCell<dyn Hardware>>);

impl HardwareHandle {
    pub fn new<T: Hardware + 'static>(inner: T) -> Self {
        Self(Rc::new(RefCell::new(inner)))
    }

    pub fn get(&self) -> &Rc<RefCell<dyn Hardware>> {
        &self.0
    }
}

pub trait Hardware {
    fn vram_update(&mut self, line: usize, buffer: &[u32]);

    fn joypad_pressed(&mut self, key: Key) -> bool;

    fn sound_play(&mut self, id: StreamId, stream: Box<dyn Stream>);

    fn sound_stop(&mut self, id: StreamId);

    /// Epoch in microseconds
    fn clock(&mut self) -> u64;

    fn send_byte(&mut self, b: u8);

    fn recv_byte(&mut self) -> Option<u8>;

    fn sched(&mut self) -> bool {
        true
    }
}
