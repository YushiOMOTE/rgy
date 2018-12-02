use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::gpu::Gpu;
use crate::device::{Lcd, Pcm};
use crate::sound::Sound;
use crate::inst;
use crate::debug::{Debugger, Perf, Resource};

pub fn run(debug: bool) {
    info!("Initializing...");

    let mut lcd = Lcd::new();
    let mut pcm = Pcm::new();

    let screen = Box::new(lcd.handle());
    let speaker = Box::new(pcm.handle());

    let _core_thread = std::thread::spawn(move || {
        let mut dbg = Debugger::new();
        let mut cpu = Cpu::new();
        let mut mmu = Mmu::new();
        let mut sound = Sound::new(speaker);
        let gpu = Gpu::new(screen);

        mmu.load();

        if debug {
            mmu.add_handler((0x0000, 0xffff), dbg.handler());
        }

        mmu.add_handler((0xff10, 0xff26), sound.handler());
        mmu.add_handler((0xff40, 0xff4f), gpu.handler());

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

    let _sound_thread = std::thread::spawn(move || {
        pcm.run();
    });

    lcd.run();
}
