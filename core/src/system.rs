use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::inst;
use crate::debug::{Debugger, Perf, Resource};

pub fn run() {
    let mut dbg = Debugger::new();
    let cpu = Cpu::new();
    let mmu = Mmu::new();

    mmu.load();

    dbg.init(&Resource::new(&cpu, &mmu));

    let mut perf = Perf::new();

    loop {
        let (code, arg) = cpu.fetch(&mmu);

        dbg.on_decode(&Resource::new(&cpu, &mmu));
        let (time, size) = inst::decode(code, arg, &cpu, &mmu);

        cpu.set_pc(cpu.get_pc().wrapping_add(size as u16));

        perf.count();
    }
}
