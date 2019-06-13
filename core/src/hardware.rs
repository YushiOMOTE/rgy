use std::cell::RefCell;
use std::rc::Rc;

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
pub enum SoundId {
    Tone1,
    Tone2,
    Wave,
    Noise,
}

#[derive(Clone)]
pub struct HardwareHandle(Rc<RefCell<dyn Hardware>>);

pub type Stream = dyn FnMut(u32) -> Option<u16> + Send + 'static;

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

    fn sound_play(&mut self, id: SoundId, stream: Box<Stream>);

    fn sound_stop(&mut self, id: SoundId);

    fn clock(&mut self) -> u64;

    fn sched(&mut self) -> bool {
        true
    }
}
