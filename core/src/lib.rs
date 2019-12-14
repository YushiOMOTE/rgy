//!
//! `rgy` is no-std cross-platform Rust GameBoy emulator library.
//!

#![no_std]
#![cfg_attr(feature = "readme", feature(external_doc))]

#[cfg_attr(feature = "readme", doc(include = "../../README.md"))]
type _Doctest = ();

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

pub use crate::{
    hardware::{Hardware, Key, Stream, VRAM_HEIGHT, VRAM_WIDTH},
    system::{run, run_debug, Config, System},
};
