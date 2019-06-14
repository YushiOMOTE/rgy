use crate::cpu::Cpu;
use crate::debug::{Debugger, Perf};
use crate::device::Device;
use crate::fc::FreqControl;
use crate::gpu::Gpu;
use crate::hardware::{Hardware, HardwareHandle};
use crate::ic::Ic;
use crate::joypad::Joypad;
use crate::mbc::Mbc;
use crate::mmu::Mmu;
use crate::serial::Serial;
use crate::sound::Sound;
use crate::timer::Timer;
use crate::Opt;
use log::*;

pub fn run<T: Hardware + 'static>(opt: Opt, rom: Vec<u8>, hw: T) {
    info!("Initializing...");

    let hw = HardwareHandle::new(hw);

    let mut fc = FreqControl::new(hw.clone(), opt.freq, opt.sample, opt.delay_unit);

    let dbg = if opt.debug {
        Some(Device::new(Debugger::new()))
    } else {
        None
    };
    let mut cpu = Cpu::new();
    let mut mmu = Mmu::new();
    let sound = Device::new(Sound::new(hw.clone()));
    let ic = Device::new(Ic::new());
    let irq = ic.borrow().irq().clone();
    let gpu = Device::new(Gpu::new(hw.clone(), irq.clone()));
    let joypad = Device::new(Joypad::new(hw.clone(), irq.clone()));
    let timer = Device::new(Timer::new(irq.clone()));
    let serial = Device::new(Serial::new(hw.clone(), irq.clone()));
    let mbc = Device::new(Mbc::new(rom));

    if let Some(dbg) = dbg.as_ref() {
        mmu.add_handler((0x0000, 0xffff), dbg.handler());
    }

    mmu.add_handler((0x0000, 0x7fff), mbc.handler());
    mmu.add_handler((0xff50, 0xff50), mbc.handler());
    mmu.add_handler((0xa000, 0xbfff), mbc.handler());
    mmu.add_handler((0xff10, 0xff3f), sound.handler());
    mmu.add_handler((0xff40, 0xff4f), gpu.handler());
    mmu.add_handler((0xff0f, 0xffff), ic.handler());
    mmu.add_handler((0xff00, 0xff00), joypad.handler());
    mmu.add_handler((0xff04, 0xff07), timer.handler());
    mmu.add_handler((0xff01, 0xff02), serial.handler());

    if let Some(dbg) = dbg.as_ref() {
        dbg.borrow_mut().init(&mmu);
    }

    let mut perf = Perf::new();

    info!("Starting...");

    fc.reset();

    while hw.get().borrow_mut().sched() {
        if let Some(dbg) = dbg.as_ref() {
            let mut dbg = dbg.borrow_mut();
            dbg.check_signal();
            dbg.take_cpu_snapshot(cpu.clone());
            dbg.on_decode(&mmu);
        }

        let mut time = cpu.execute(&mut mmu);

        time += cpu.check_interrupt(&mut mmu, &ic);

        gpu.borrow_mut().step(time, &mut mmu);
        timer.borrow_mut().step(time);
        serial.borrow_mut().step(time);
        joypad.borrow_mut().poll();

        perf.count();

        if !opt.native_speed {
            fc.adjust(time);
        }
    }
}
