use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

/// The width of the VRAM.
pub const VRAM_WIDTH: usize = 160;

/// The height of the VRAM.
pub const VRAM_HEIGHT: usize = 144;

/// Represents a key of the joypad.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    /// Cursor right key.
    Right,
    /// Cursor left key.
    Left,
    /// Cursor up key.
    Up,
    /// Cursor down key.
    Down,
    /// A key.
    A,
    /// B key.
    B,
    /// Select key.
    Select,
    /// Start key.
    Start,
}

/// Sound wave stream which generates the wave to be played by the sound device.
pub trait Stream: Send + 'static {
    /// The maximum value of the amplitude returned by this stream.
    fn max(&self) -> u16;

    /// The argument takes the sample rate, and the return value indicates the amplitude,
    /// whose max value is determined by [`Stream::max`][].
    fn next(&mut self, rate: u32) -> u16;

    /// Indicate the stream is active.
    fn on(&self) -> bool;
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

/// The interface to abstracts the OS-specific functions.
///
/// The users of this emulator library need to implement this trait,
/// providing OS-specific functions.
pub trait Hardware {
    /// Called when one horizontal line in the display is updated.
    fn vram_update(&mut self, line: usize, buffer: &[u32]);

    /// Called when the emulator checks if the key is pressed.
    fn joypad_pressed(&mut self, key: Key) -> bool;

    /// Called when the emulator plays a sound.
    /// The stream in the argument is the stream which keeps returning wave patterns.
    fn sound_play(&mut self, stream: Box<dyn Stream>);

    /// Clock source used by the emulator.
    /// The return value needs to be epoch time in microseconds.
    fn clock(&mut self) -> u64;

    /// Send one byte to the serial port.
    fn send_byte(&mut self, b: u8);

    /// Try receiving one byte from the serial port.
    fn recv_byte(&mut self) -> Option<u8>;

    /// Called every time the CPU executes one instruction.
    /// Returning `false` stops the emulator.
    fn sched(&mut self) -> bool {
        true
    }

    /// Called when the CPU attempts to write save data to the cartridge battery-backed RAM.
    fn load_ram(&mut self, size: usize) -> Vec<u8>;

    /// Called when the CPU attempts to read save data from the cartridge battery-backed RAM.
    fn save_ram(&mut self, ram: &[u8]);
}
