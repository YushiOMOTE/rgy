mod debug;
mod hardware;
mod loader;

use crate::{
    debug::Debugger,
    hardware::Hardware,
    loader::{load_rom, Loader},
};

use log::*;
use std::path::PathBuf;
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
    /// Enable state dump file. Effective only in debug mode.
    #[structopt(short = "p", long = "dump_path")]
    dump_path: Option<PathBuf>,
    /// Don't enter debug shell on start.
    #[structopt(short = "D", long = "no_dbg_shell")]
    no_dbg_shell: bool,
    /// RAM file name
    #[structopt(short = "r", long = "ram")]
    ram: Option<String>,
    /// ROM file name or directory
    #[structopt(name = "ROM")]
    rom: PathBuf,
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
            let debugger = if let Some(path) = opt.dump_path.as_ref() {
                Debugger::with_dump_file(!opt.no_dbg_shell, path)
            } else {
                Debugger::new(!opt.no_dbg_shell)
            };
            rgy::run_debug(to_cfg(opt), &rom, hw1, debugger);
        } else {
            rgy::run(to_cfg(opt), &rom, hw1);
        }
    });

    hw.run();
}
