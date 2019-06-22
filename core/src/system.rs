use crate::cgb::Cgb;
use crate::cpu::Cpu;
use crate::debug::Debugger;
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
use alloc::vec::Vec;
use log::*;

pub struct Config {
    /// CPU frequency
    pub(crate) freq: u64,
    /// Cycle sampling count in CPU frequency controller
    pub(crate) sample: u64,
    /// Delay unit in CPU frequency controller
    pub(crate) delay_unit: u64,
    /// Don't adjust CPU frequency
    pub(crate) native_speed: bool,
}

impl Config {
    pub fn new() -> Self {
        let freq = 4194300; // 4.1943 MHz
        Self {
            freq,
            sample: freq / 1000,
            delay_unit: 10,
            native_speed: false,
        }
    }

    pub fn freq(mut self, freq: u64) -> Self {
        self.freq = freq;
        self
    }

    pub fn sample(mut self, sample: u64) -> Self {
        self.sample = sample;
        self
    }

    pub fn delay_unit(mut self, delay: u64) -> Self {
        self.delay_unit = delay;
        self
    }

    pub fn native_speed(mut self, native: bool) -> Self {
        self.native_speed = native;
        self
    }
}

pub fn run<T: Hardware + 'static>(cfg: Config, rom: Vec<u8>, hw: T) {
    run_inner(cfg, rom, hw, Debugger::empty())
}

pub fn run_debug<T: Hardware + 'static, D: Debugger + 'static>(
    cfg: Config,
    rom: Vec<u8>,
    hw: T,
    dbg: D,
) {
    run_inner(cfg, rom, hw, dbg)
}

fn run_inner<T: Hardware + 'static, D: Debugger + 'static>(
    cfg: Config,
    rom: Vec<u8>,
    hw: T,
    dbg: D,
) {
    info!("Initializing...");

    let hw = HardwareHandle::new(hw);

    let mut fc = FreqControl::new(hw.clone(), &cfg);

    let dbg = Device::new(dbg);
    let mut cpu = Cpu::new();
    let mut mmu = Mmu::new();
    let sound = Device::new(Sound::new(hw.clone()));
    let ic = Device::new(Ic::new());
    let irq = ic.borrow().irq().clone();
    let gpu = Device::new(Gpu::new(hw.clone(), irq.clone()));
    let joypad = Device::new(Joypad::new(hw.clone(), irq.clone()));
    let timer = Device::new(Timer::new(irq.clone()));
    let serial = Device::new(Serial::new(hw.clone(), irq.clone()));
    let mbc = Device::new(Mbc::new(hw.clone(), rom));
    let cgb = Device::new(Cgb::new());

    mmu.add_handler((0x0000, 0xffff), dbg.handler());

    mmu.add_handler((0xc000, 0xdfff), cgb.handler());
    mmu.add_handler((0xff4d, 0xff4d), cgb.handler());
    mmu.add_handler((0xff56, 0xff56), cgb.handler());
    mmu.add_handler((0xff70, 0xff70), cgb.handler());

    mmu.add_handler((0x0000, 0x7fff), mbc.handler());
    mmu.add_handler((0xff50, 0xff50), mbc.handler());
    mmu.add_handler((0xa000, 0xbfff), mbc.handler());
    mmu.add_handler((0xff10, 0xff3f), sound.handler());
    mmu.add_handler((0xff40, 0xff4f), gpu.handler());
    mmu.add_handler((0xff0f, 0xffff), ic.handler());
    mmu.add_handler((0xff00, 0xff00), joypad.handler());
    mmu.add_handler((0xff04, 0xff07), timer.handler());
    mmu.add_handler((0xff01, 0xff02), serial.handler());

    dbg.borrow_mut().init(&mmu);

    info!("Starting...");

    fc.reset();

    while hw.get().borrow_mut().sched() {
        {
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

        if !cfg.native_speed {
            fc.adjust(time);
        }
    }
}
