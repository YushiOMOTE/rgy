// mod debug;
mod hardware;
mod loader;

use crate::{
    // debug::Debugger,
    hardware::Hardware,
    loader::{load_rom, Loader},
};

use log::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// Emulate Gameboy Color
    #[structopt(short = "c", long = "color")]
    color: bool,
    /// Cpu frequency
    #[structopt(short = "f", long = "freq", default_value = "4200000")]
    freq: u64,
    /// Interval to refill the token bucket for CPU rate-limiting.
    #[structopt(short = "i", long = "interval", default_value = "20000")]
    interval: u64,
    /// Don't adjust cpu frequency
    #[structopt(short = "n", long = "native")]
    native_speed: bool,
    /// Enable debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,
    /// RAM file name
    #[structopt(short = "r", long = "ram")]
    ram: Option<String>,
    /// ROM file name or directory
    #[structopt(name = "ROM")]
    rom: PathBuf,
}

fn to_cfg(opt: Opt) -> rgy::Config {
    rgy::Config::new()
        .color(opt.color)
        .freq(opt.freq)
        .rate_limit_interval(opt.interval)
        .native_speed(opt.native_speed)
}

fn set_affinity() {
    let set = || {
        let core_ids = core_affinity::get_core_ids()?;
        core_affinity::set_for_current(*core_ids.first()?);
        Some(())
    };

    if set().is_none() {
        warn!("Couldn't set CPU affinity")
    }
}

fn main() {
    let opt = Opt::from_args();

    use std::io::Write;

    let mut builder = env_logger::Builder::from_default_env();

    builder
        .format(|buf, record| {
            let ts = buf.timestamp_millis();

            writeln!(buf, "{}: {}: {}", ts, record.level(), record.args())
        })
        .init();
    // env_logger::init();

    let hw = Hardware::new(opt.ram.clone(), opt.color);
    let hw1 = hw.clone();

    std::thread::spawn(move || {
        let (rom, hw1) = if opt.rom.is_dir() {
            let mut ldr = Loader::new(&opt.rom);

            utils::select(&mut ldr, hw1)
        } else {
            (load_rom(&opt.rom), hw1)
        };

        set_affinity();

        if opt.debug {
            // rgy::run_debug(to_cfg(opt), &rom, hw1, Debugger::new());
        } else {
            rgy::run(to_cfg(opt), &rom, hw1);
        }
    });

    hw.run();
}
