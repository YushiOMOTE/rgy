mod alu;
mod cpu;
mod debug;
mod device;
mod gpu;
mod ic;
mod inst;
mod joypad;
mod mbc;
mod mmu;
mod sound;
mod system;
mod timer;

use crate::device::HardwareImpl;
use std::fs::File;
use std::io::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// Cpu frequency
    #[structopt(short = "f", long = "freq", default_value = "1000000")]
    freq: usize,
    /// Sampling rate for cpu frequency controller
    #[structopt(short = "s", long = "sample", default_value = "1000")]
    sample: usize,
    /// Delay unit for cpu frequency controller
    #[structopt(short = "u", long = "delayunit", default_value = "10")]
    delay_unit: usize,
    /// Don't adjust cpu frequency
    #[structopt(short = "n", long = "native")]
    native_speed: bool,
    /// Enable debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,
    #[structopt(name = "ROM")]
    rom: String,
}

fn load_rom(name: &str) -> Vec<u8> {
    let mut f = File::open(name).expect("Couldn't open file");
    let mut buf = Vec::new();

    f.read_to_end(&mut buf).expect("Couldn't read file");

    buf
}

fn main() {
    let opt = Opt::from_args();

    env_logger::init();

    let hw = HardwareImpl::new();
    let rom = load_rom(&opt.rom);

    system::run(opt, rom, hw);
}
