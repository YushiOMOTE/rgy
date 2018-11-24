use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::inst;

use std::time::Instant;

struct Perf {
    counter: u64,
    last: Instant,
}

impl Perf {
    fn new() -> Perf {
        Perf {
            counter: 0,
            last: Instant::now(),
        }
    }

    fn count(&mut self) {
        let sample = 10000000;

        self.counter += 1;

        if self.counter % sample == 0 {
            let now = Instant::now();
            let df = now - self.last;
            let df = df.as_secs() * 1000000 + df.subsec_micros() as u64;

            println!("{} ips", sample * 1000000 / df);

            self.last = now;
        }
    }
}

pub fn run() {
    let cpu = Cpu::new();
    let mmu = Mmu::new();

    mmu.load();

    let mut perf = Perf::new();

    loop {
        let pc = cpu.get_pc();
        let fb = mmu.get8(pc);

        let (time, size) = if fb == 0xcb {
            let sb = mmu.get8(pc);
            inst::decode((sb as u16) << 8 | fb as u16, &cpu, &mmu)
        } else {
            inst::decode(fb as u16, &cpu, &mmu)
        };

        cpu.set_pc(cpu.get_pc().wrapping_add(size as u16));

        perf.count();
    }
}
