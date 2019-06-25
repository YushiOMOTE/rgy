#![no_std]

extern crate alloc;

mod alu;
mod cgb;
pub mod cpu;
pub mod debug;
pub mod device;
mod dma;
mod fc;
mod gpu;
pub mod hardware;
mod ic;
pub mod inst;
mod joypad;
mod mbc;
pub mod mmu;
mod serial;
mod sound;
mod system;
mod timer;

pub use crate::system::{run, run_debug, Config};
