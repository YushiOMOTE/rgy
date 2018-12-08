#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate cpal;
extern crate env_logger;
extern crate minifb;
extern crate rustyline;
extern crate structopt;

mod cpu;
mod gpu;
mod mmu;
mod inst;
mod alu;
mod system;
mod debug;
mod device;
mod sound;
mod ic;

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

    system::run(opt);
}
