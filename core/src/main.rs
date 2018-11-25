#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rustyline;

mod cpu;
mod gpu;
mod mmu;
mod inst;
mod alu;
mod system;
mod debug;

fn main() {
    env_logger::init();

    system::run();
}
