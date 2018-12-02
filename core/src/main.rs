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

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// Enable debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,
}

fn main() {
    let opt = Opt::from_args();

    env_logger::init();

    system::run(opt.debug);
}
