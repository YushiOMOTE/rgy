//!
//! `rgy` is no-std cross-platform Rust GameBoy emulator library.
//!
//! The users of this library only needs to implement [`Hardware`][] trait, which abstracts OS-specific function.
//! Once it's implemented, the emulator works.
//!
//! The following code is the example which just implements `Hardware`. The implementation does nothing.
//! You can replace the body of each function with the actual meaningful logic.
//!
//! ```rust,no_run
//! use rgy::{Config, Key, Stream, VRAM_HEIGHT, VRAM_WIDTH};
//!
//! struct Hardware {
//!     dummy_display: Vec<Vec<u32>>,
//! }
//!
//! impl Hardware {
//!     fn new() -> Self {
//!         // Create a frame buffer with the size VRAM_WIDTH * VRAM_HEIGHT.
//!         let dummy_display = vec![vec![0u32; VRAM_HEIGHT]; VRAM_WIDTH];
//!
//!         Self { dummy_display }
//!     }
//! }
//!
//! impl rgy::Hardware for Hardware {
//!     // Called when a horizontal line in the display is updated by the emulator.
//!     fn vram_update(&mut self, line: usize, buffer: &[u32]) {
//!         // `line` corresponds to the y coordinate.
//!         let y = line;
//!
//!         for (x, col) in buffer.iter().enumerate() {
//!             // TODO: Update the pixels in the actual display here.
//!             self.dummy_display[x][y] = *col;
//!         }
//!     }
//!
//!     // Called when the emulator checks if a key is pressed or not.
//!     fn joypad_pressed(&mut self, key: Key) -> bool {
//!         println!("Is {:?} pressed?", key);
//!
//!         // TODO: Read a keyboard device and check if the `key` is pressed or not.
//!
//!         false
//!     }
//!
//!     // Called when the emulator plays a sound.
//!     fn sound_play(&mut self, _stream: Box<dyn Stream>) {
//!         // TODO: Play the wave pattern provided `Stream`.
//!     }
//!
//!     // Provides clock for the emulator.
//!     fn clock(&mut self) -> u64 {
//!         // TODO: Return the epoch in microseconds.
//!         let epoch = std::time::SystemTime::now()
//!             .duration_since(std::time::UNIX_EPOCH)
//!             .expect("Couldn't get epoch");
//!         epoch.as_micros() as u64
//!     }
//!
//!     // Called when the emulator sends a byte to the serial port.
//!     fn send_byte(&mut self, _b: u8) {
//!         // TODO: Send a byte to a serial port.
//!     }
//!
//!     // Called when the emulator peeks a byte from the serial port.
//!     fn recv_byte(&mut self) -> Option<u8> {
//!         // TODO: Check the status of the serial port and read a byte if any.
//!         None
//!     }
//!
//!     // Called every time the emulator executes an instruction.
//!     fn sched(&mut self) -> bool {
//!         // TODO: Do some periodic jobs if any. Return `true` to continue, `false` to stop the emulator.
//!         println!("It's running!");
//!         true
//!     }
//!
//!     // Called when the emulator stores the save data to the battery-backed RAM.
//!     fn load_ram(&mut self, size: usize) -> Vec<u8> {
//!         // TODO: Return save data.
//!         vec![0; size]
//!     }
//!
//!     // Called when the emulator loads the save data from the battery-backed RAM.
//!     fn save_ram(&mut self, _ram: &[u8]) {
//!         // TODO: Store save data.
//!     }
//! }
//!
//! fn main() {
//!     // Create the default config.
//!     let cfg = Config::new();
//!
//!     // Create the hardware instance.
//!     let hw = Hardware::new();
//!
//!     // TODO: The content of a ROM file, which can be downloaded from the Internet.
//!     let rom = vec![0u8; 1024];
//!
//!     // Run the emulator.
//!     rgy::run(cfg, &rom, hw);
//! }
//! ```

#![no_std]
#![cfg_attr(feature = "readme", feature(external_doc))]
#![warn(missing_docs)]

#[cfg_attr(feature = "readme", doc(include = "../../README.md"))]
type _Doctest = ();

extern crate alloc;

mod alu;
mod cgb;
mod dma;
mod fc;
mod gpu;
mod ic;
mod joypad;
mod mbc;
mod serial;
mod sound;
mod system;
mod timer;

/// CPU state.
pub mod cpu;

/// Debugger interface.
pub mod debug;

/// Adaptor to register devices to MMU.
pub mod device;

/// Decoder which evaluates each CPU instructions.
pub mod inst;

/// Handles memory and I/O port access from the CPU.
pub mod mmu;

/// Hardware interface, which abstracts OS-specific functions.
mod hardware;

pub use crate::hardware::{Hardware, Key, Stream, VRAM_HEIGHT, VRAM_WIDTH};
pub use crate::system::{run, run_debug, Config, System};
