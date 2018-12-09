use log::*;
use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::gpu::Gpu;
use crate::device::{Lcd, Pcm};
use crate::sound::Sound;
use crate::ic::Ic;
use crate::inst;
use crate::debug::{Debugger, Perf};
use std::time::Instant;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::Opt;

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

pub fn run(opt: Opt) {
    info!("Initializing...");

    let mut fc = FreqControl::new(opt.freq, opt.sample, opt.delay_unit);

    let mut lcd = Lcd::new();
    let pcm = Pcm::new();

    let screen = Box::new(lcd.handle());
    let speaker = Box::new(pcm.handle());

    let _core_thread = std::thread::spawn(move || {
        let mut dbg = Debugger::new();
        let mut cpu = Cpu::new();
        let mut mmu = Mmu::new();
        let mut sound = Sound::new(speaker);
        let ic = Ic::new();
        let gpu = Gpu::new(screen, ic.irq());

        mmu.setup(&opt.rom);

        if opt.debug {
            mmu.add_handler((0x0000, 0xffff), dbg.handler());
        }

        mmu.add_handler((0xff10, 0xff26), sound.handler());
        mmu.add_handler((0xff40, 0xff4f), gpu.handler());
        mmu.add_handler((0xff0f, 0xffff), ic.handler());

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

        loop {
            let (code, arg) = cpu.fetch(&mmu);

            dbg.take_cpu_snapshot(cpu.clone());
            dbg.on_decode(&mmu);

            let (time, size) = inst::decode(code, arg, &mut cpu, &mut mmu);
            cpu.set_pc(cpu.get_pc().wrapping_add(size as u16));

            cpu.check_interrupt(&mut mmu, &ic);

            gpu.step(time, &mut mmu);

            perf.count();

            if !opt.native_speed {
                fc.adjust();
            }
        }
    });

    let _sound_thread = std::thread::spawn(move || {
        pcm.run();
    });

    lcd.run();
}
