use crate::cpu::Cpu;

/// Debugger interface.
///
/// The users of this library can implement this interface to inspect the state of the emulator.
pub trait Debugger {
    /// The function is called on the initialization phase.
    fn init(&mut self, cpu: &Cpu);

    /// The function is called right before the emulator starts executing an instruction. Deprecated.
    fn take_cpu_snapshot(&mut self, cpu: Cpu);

    /// Decode an instruction.
    fn on_decode(&mut self, cpu: &Cpu);

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
    fn init(&mut self, _: &Cpu) {}

    fn take_cpu_snapshot(&mut self, _: Cpu) {}

    fn on_decode(&mut self, _: &Cpu) {}

    fn check_signal(&mut self) {}
}
