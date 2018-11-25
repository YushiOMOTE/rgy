use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::gpu::Gpu;
use crate::inst;
use crate::debug::{Debugger, Perf, Resource};

use std::rc::Rc;

pub fn run() {
    let mut dbg = Debugger::new();
    let cpu = Cpu::new();
    let mut mmu = Mmu::new();
    let gpu = Rc::new(Gpu::new());

    mmu.load();
    mmu.add_rdhooks((0xff40, 0xff4f), gpu.clone());
    mmu.add_wrhooks((0xff40, 0xff4f), gpu.clone());

    dbg.init(&Resource::new(&cpu, &mmu));

    let mut perf = Perf::new();

    loop {
        let (code, arg) = cpu.fetch(&mmu);

        dbg.on_decode(&Resource::new(&cpu, &mmu));
        let (time, size) = inst::decode(code, arg, &cpu, &mmu);
        cpu.set_pc(cpu.get_pc().wrapping_add(size as u16));

        gpu.step(time);

        perf.count();
    }
}
