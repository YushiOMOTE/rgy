mod alu;
mod cpu;
mod debug;
mod device;
mod gpu;
mod ic;
mod inst;
mod joypad;
mod mmu;
mod sound;
mod system;
mod timer;

use crate::device::HardwareImpl;
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

fn main() {
    let opt = Opt::from_args();

    env_logger::init();

    let hw = HardwareImpl::new();

    system::run(opt, hw);
}
