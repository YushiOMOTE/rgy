#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate env_logger;

mod cpu;
mod mmu;
mod inst;
mod alu;
mod system;
mod debug;

fn main() {
    env_logger::init();

    system::run();
}
