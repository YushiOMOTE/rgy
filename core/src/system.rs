use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::gpu::Gpu;
use crate::lcd::Lcd;
use crate::inst;
use crate::debug::{Debugger, Perf, Resource};

use std::rc::Rc;

pub fn run(debug: bool) {
    info!("Initializing...");

    let mut lcd = Lcd::new();

    let screen = Box::new(lcd.handle());

    let hd = std::thread::spawn(move || {
        let mut dbg = Debugger::new();
        let mut cpu = Cpu::new();
        let mut mmu = Mmu::new();
        let gpu = Gpu::new(screen);

        mmu.load();
        mmu.add_rdhooks((0xff40, 0xff4f), gpu.handler());
        mmu.add_wrhooks((0xff40, 0xff4f), gpu.handler());

        if debug {
            dbg.init(&Resource::new(&cpu, &mmu));
        }

        let mut perf = Perf::new();

        info!("Starting...");

        loop {
            let (code, arg) = cpu.fetch(&mmu);

            dbg.on_decode(&Resource::new(&cpu, &mmu));
            let (time, size) = inst::decode(code, arg, &mut cpu, &mut mmu);
            cpu.set_pc(cpu.get_pc().wrapping_add(size as u16));

            gpu.step(time, &mut mmu);

            perf.count();
        }
    });

    lcd.run();

    // hd.join().unwrap();
}
