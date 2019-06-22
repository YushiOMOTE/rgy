use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec::Vec;
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

pub trait Stream: Send + 'static {
    fn max(&self) -> u16;

    // Return value is in range 0 - 16
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

    fn sound_play(&mut self, stream: Box<dyn Stream>);

    /// Epoch in microseconds
    fn clock(&mut self) -> u64;

    fn send_byte(&mut self, b: u8);

    fn recv_byte(&mut self) -> Option<u8>;

    fn sched(&mut self) -> bool {
        true
    }

    fn load_ram(&mut self, size: usize) -> Vec<u8>;

    fn save_ram(&mut self, ram: &[u8]);
}
