use crate::cpu::Cpu;
use crate::device::IoHandler;
use crate::mmu::{MemRead, MemWrite, Mmu};

pub trait Debugger: IoHandler {
    fn init(&mut self, mmu: &Mmu);
    fn take_cpu_snapshot(&mut self, cpu: Cpu);
    fn on_decode(&mut self, mmu: &Mmu);
    fn check_signal(&mut self);
}

impl dyn Debugger {
    pub fn empty() -> NullDebugger {
        NullDebugger
    }
}

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
