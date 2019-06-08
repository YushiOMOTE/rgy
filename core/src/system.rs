use crate::cpu::Cpu;
use crate::debug::{Debugger, Perf};
use crate::device::{Hardware, HardwareHandle};
use crate::gpu::Gpu;
use crate::ic::Ic;
use crate::inst;
use crate::joypad::Joypad;
use crate::mbc::Mbc;
use crate::mmu::Mmu;
use crate::sound::Sound;
use crate::timer::Timer;
use crate::Opt;
use log::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

struct FreqControl {
    last: Instant,
    count: usize,
    barrier: AtomicUsize,
    sample: usize,
    delay: usize,
    delay_unit: usize,
    target: usize,
}

impl FreqControl {
    fn new(target: usize, sample: usize, delay_unit: usize) -> Self {
        Self {
            last: Instant::now(),
            count: 0,
            barrier: AtomicUsize::new(0),
            delay: 0,
            sample,
            delay_unit,
            target,
        }
    }

    fn reset(&mut self) {
        self.last = Instant::now();
    }

    fn adjust(&mut self) {
        self.count += 1;

        for _ in 0..self.delay {
            self.barrier.fetch_add(1, Ordering::Relaxed);
        }

        if self.count % self.sample == 0 {
            let now = Instant::now();
            let df = now - self.last;
            let df = df.as_secs() as usize * 1000000 + df.subsec_micros() as usize;
            let ips = self.sample * 1000000 / df;

            if ips > self.target {
                self.delay += self.delay_unit;
            } else {
                if self.delay > 0 {
                    self.delay -= self.delay_unit;
                }
            }

            self.last = now;
        }
    }
}

pub fn run<T: Hardware + 'static>(opt: Opt, rom: Vec<u8>, hw: T) {
    info!("Initializing...");

    let mut fc = FreqControl::new(opt.freq, opt.sample, opt.delay_unit);

    let hw = HardwareHandle::new(hw);

    let mut dbg = Debugger::new();
    let mut cpu = Cpu::new();
    let mut mmu = Mmu::new();
    let mut sound = Sound::new(hw.clone());
    let ic = Ic::new();
    let gpu = Gpu::new(hw.clone(), ic.irq());
    let joypad = Joypad::new(hw.clone(), ic.irq());
    let timer = Timer::new(hw.clone(), ic.irq());
    let mbc = Mbc::new(rom);

    if opt.debug {
        mmu.add_handler((0x0000, 0xffff), dbg.handler());
    }

    mmu.add_handler((0x0000, 0x7fff), mbc.handler());
    mmu.add_handler((0xff50, 0xff50), mbc.handler());
    mmu.add_handler((0xff10, 0xff26), sound.handler());
    mmu.add_handler((0xff40, 0xff4f), gpu.handler());
    mmu.add_handler((0xff0f, 0xffff), ic.handler());
    mmu.add_handler((0xff00, 0xff00), joypad.handler());
    mmu.add_handler((0xff04, 0xff07), timer.handler());

    if opt.debug {
        dbg.init(&mmu);
    }

    let mut perf = Perf::new();

    info!("Starting...");

    let mut last = Instant::now();
    let mut count = AtomicUsize::new(0);
    let mut delay = 0;
    let sample = 500;
    let unit = 100;

    fc.reset();

    while hw.get().borrow_mut().sched() {
        let (code, arg) = cpu.fetch(&mmu);

        if opt.debug {
            dbg.check_signal();
            dbg.take_cpu_snapshot(cpu.clone());
            dbg.on_decode(&mmu);
        }

        let (time, size) = inst::decode(code, arg, &mut cpu, &mut mmu);
        cpu.set_pc(cpu.get_pc().wrapping_add(size as u16));

        cpu.check_interrupt(&mut mmu, &ic);

        gpu.step(time, &mut mmu);
        timer.step(time);

        perf.count();

        if !opt.native_speed {
            fc.adjust();
        }
    }
}
