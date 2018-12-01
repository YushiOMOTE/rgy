#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate minifb;
extern crate rustyline;

mod cpu;
mod gpu;
mod mmu;
mod inst;
mod alu;
mod system;
mod debug;
mod lcd;

fn main() {
    env_logger::init();

    system::run();
}
