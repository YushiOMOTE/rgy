use crate::cpu::{Cpu, CPU_FREQ_HZ};
use crate::debug::Debugger;
use crate::fc::FreqControl;
use crate::hardware::{Hardware, HardwareHandle};
use crate::mmu::Mmu;
use log::*;

/// Configuration of the emulator.
pub struct Config {
    /// CPU frequency.
    pub(crate) freq: u64,
    /// Interval to refill tokens in the token bucket for CPU rate-limiting in micro-seconds.
    pub(crate) rate_limit_interval: u64,
    /// Don't adjust CPU frequency.
    pub(crate) native_speed: bool,
    /// Emulate Gameboy Color
    pub(crate) color: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    /// Create the default configuration.
    pub fn new() -> Self {
        let freq = CPU_FREQ_HZ as u64;

        Self {
            freq,
            rate_limit_interval: 20_000,
            native_speed: false,
            color: false,
        }
    }

    /// Set the CPU frequency.
    pub fn freq(mut self, freq: u64) -> Self {
        self.freq = freq;
        self
    }

    /// Interval to refill tokens in the token bucket for CPU rate-limiting in micro-seconds.
    pub fn rate_limit_interval(mut self, interval: u64) -> Self {
        self.rate_limit_interval = interval;
        self
    }

    /// Set the flag to run at native speed.
    pub fn native_speed(mut self, native: bool) -> Self {
        self.native_speed = native;
        self
    }

    /// Set the flag to enable Gameboy Color.
    pub fn color(mut self, color: bool) -> Self {
        self.color = color;
        self
    }
}

/// Represents the entire emulator context.
pub struct System<D> {
    cfg: Config,
    hw: HardwareHandle,
    fc: FreqControl,
    cpu: Cpu,
    _dbg: D,
}

impl<D> System<D>
where
    D: Debugger + 'static,
{
    /// Create a new emulator context.
    pub fn new<T>(cfg: Config, rom: &[u8], hw: T, dbg: D) -> Self
    where
        T: Hardware + 'static,
    {
        info!("Initializing...");

        let hw = HardwareHandle::new(hw);

        let mut fc = FreqControl::new(hw.clone(), &cfg);

        let mmu = Mmu::new(hw.clone(), rom.to_vec(), cfg.color);
        let cpu = Cpu::new(mmu);

        info!("Starting...");

        fc.reset();

        Self {
            cfg,
            hw,
            fc,
            cpu,
            _dbg: dbg,
        }
    }

    /// Run a single step of emulation.
    /// This function needs to be called repeatedly until it returns `false`.
    /// Returning `false` indicates the end of emulation, and the functions shouldn't be called again.
    pub fn poll(&mut self) -> bool {
        if !self.hw.get().borrow_mut().sched() {
            return false;
        }

        let time = self.cpu.execute();

        if !self.cfg.native_speed {
            self.fc.adjust(time);
        }

        true
    }
}

/// Run the emulator with the given configuration.
pub fn run<T: Hardware + 'static>(cfg: Config, rom: &[u8], hw: T) {
    run_inner(cfg, rom, hw, <dyn Debugger>::empty())
}

/// Run the emulator with the given configuration and debugger.
pub fn run_debug<T: Hardware + 'static, D: Debugger + 'static>(
    cfg: Config,
    rom: &[u8],
    hw: T,
    dbg: D,
) {
    run_inner(cfg, rom, hw, dbg)
}

fn run_inner<T: Hardware + 'static, D: Debugger + 'static>(cfg: Config, rom: &[u8], hw: T, dbg: D) {
    let mut sys = System::new(cfg, rom, hw, dbg);
    while sys.poll() {}
}
