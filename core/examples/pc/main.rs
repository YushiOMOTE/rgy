mod debug;
mod hardware;

use crate::{debug::Debugger, hardware::Hardware};

use log::*;
use std::fs::File;
use std::io::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// Cpu frequency
    #[structopt(short = "f", long = "freq", default_value = "4200000")]
    freq: u64,
    /// Sampling rate for cpu frequency controller
    #[structopt(short = "s", long = "sample", default_value = "4200")]
    sample: u64,
    /// Delay unit for cpu frequency controller
    #[structopt(short = "u", long = "delayunit", default_value = "50")]
    delay_unit: u64,
    /// Don't adjust cpu frequency
    #[structopt(short = "n", long = "native")]
    native_speed: bool,
    /// Enable debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,
    /// RAM file name
    #[structopt(short = "r", long = "ram")]
    ram: Option<String>,
    /// ROM file name
    #[structopt(name = "ROM")]
    rom: String,
}

fn load_rom(name: &str) -> Vec<u8> {
    let mut f = File::open(name).expect("Couldn't open file");
    let mut buf = Vec::new();

    f.read_to_end(&mut buf).expect("Couldn't read file");

    buf
}

fn to_cfg(opt: Opt) -> rgy::Config {
    rgy::Config::new()
        .freq(opt.freq)
        .sample(opt.sample)
        .delay_unit(opt.delay_unit)
        .native_speed(opt.native_speed)
}

fn set_affinity() {
    let set = || {
        let core_ids = core_affinity::get_core_ids()?;
        core_affinity::set_for_current(*core_ids.get(0)?);
        Some(())
    };

    match set() {
        None => warn!("Couldn't set CPU affinity"),
        _ => {}
    }
}

fn main() {
    let opt = Opt::from_args();

    env_logger::init();

    let hw = Hardware::new(opt.ram.clone());
    let rom = load_rom(&opt.rom);

    let hw1 = hw.clone();

    std::thread::spawn(move || {
        set_affinity();

        if opt.debug {
            rgy::run_debug(to_cfg(opt), &rom, hw1, Debugger::new());
        } else {
            rgy::run(to_cfg(opt), &rom, hw1);
        }
    });

    hw.run();
}
