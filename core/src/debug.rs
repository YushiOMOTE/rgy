use crate::cpu::Cpu;
use crate::device::IoHandler;
use crate::mmu::{MemRead, MemWrite, Mmu};

/// Debugger interface.
///
/// The users of this library can implement this interface to inspect the state of the emulator.
pub trait Debugger: IoHandler {
    /// The function is called on the initialization phase.
    fn init(&mut self, mmu: &Mmu);

    /// The function is called right before the emulator starts executing an instruction. Deprecated.
    fn take_cpu_snapshot(&mut self, cpu: Cpu);

    /// Decode an instruction.
    fn on_decode(&mut self, mmu: &Mmu);

    /// Check if the external signal is triggered. Deprecated.
    fn check_signal(&mut self);
}

impl dyn Debugger {
    /// Create an empty debugger.
    pub fn empty() -> NullDebugger {
        NullDebugger
    }
}

/// Empty debugger which does nothing.
pub struct NullDebugger;

impl Debugger for NullDebugger {
    fn init(&mut self, _: &Mmu) {}

    fn take_cpu_snapshot(&mut self, _: Cpu) {}

    fn on_decode(&mut self, _: &Mmu) {}

    fn check_signal(&mut self) {}
}

impl IoHandler for NullDebugger {
    fn on_read(&mut self, _: &Mmu, _: u16) -> MemRead {
        MemRead::PassThrough
    }

    fn on_write(&mut self, _: &Mmu, _: u16, _: u8) -> MemWrite {
        MemWrite::PassThrough
    }
}
