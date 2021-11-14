use crate::mmu::Mmu;
use alloc::fmt;
use log::*;

/// Interface for CPU to interact with memory/devices
pub trait Sys {
    /// Get the interrupt vector address clearing the interrupt flag state
    fn pop_int_vec(&self) -> Option<u8>;

    /// Get the interrupt vector address without clearing the interrupt flag state
    fn peek_int_vec(&self) -> Option<u8>;

    /// Read a byte from the address.
    fn get8(&self, addr: u16) -> u8;

    /// Write a byte to the address.
    fn set8(&mut self, addr: u16, v: u8);

    /// Reads two bytes from the given addresss in the memory.
    fn get16(&self, addr: u16) -> u16 {
        let l = self.get8(addr);
        let h = self.get8(addr + 1);
        (h as u16) << 8 | l as u16
    }

    /// Writes two bytes at the given address in the memory.
    fn set16(&mut self, addr: u16, v: u16) {
        self.set8(addr, v as u8);
        self.set8(addr + 1, (v >> 8) as u8);
    }

    /// Proceed the system state by the given CPU cycles.
    fn step(&mut self, cycles: usize);
}

/// Represents CPU state.
pub struct Cpu<T = Mmu> {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
    ime: bool,
    ei_delay: usize,
    di_delay: usize,
    halt: bool,
    halt_bug: bool,
    cycles: usize,
    sys: T,
}

impl<T: Sys> fmt::Display for Cpu<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "a:  [{:02x}],  b:  [{:02x}]\n\
             c:  [{:02x}],  d:  [{:02x}]\n\
             e:  [{:02x}],  f:  [{:02x}]\n\
             h:  [{:02x}],  l:  [{:02x}]\n\
             pc: [{:04x}]\n\
             sp: [{:04x}]\n\
             flgs: [{}{}{}{}]\
             ",
            self.a,
            self.b,
            self.c,
            self.d,
            self.e,
            self.f,
            self.h,
            self.l,
            self.pc,
            self.sp,
            if self.get_zf() { "z" } else { "_" },
            if self.get_nf() { "n" } else { "_" },
            if self.get_hf() { "h" } else { "_" },
            if self.get_cf() { "c" } else { "_" },
        )
    }
}

impl<T: Sys> Cpu<T> {
    /// Create a new CPU state.
    pub fn new(sys: T) -> Cpu<T> {
        Cpu {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
            ime: true,
            ei_delay: 0,
            di_delay: 0,
            halt: false,
            halt_bug: false,
            cycles: 0,
            sys,
        }
    }

    /// Switch the CPU state to halting.
    pub fn halt(&mut self) {
        debug!("Halt");
        if !self.ime && self.sys.peek_int_vec().is_some() {
            self.halt_bug = true;
        } else {
            self.halt = true;
        }
    }

    /// Execute a single instruction.
    ///
    /// The function fetches an instruction code from the memory,
    /// decodes it, and updates the CPU/memory state accordingly.
    /// The return value is the number of clock cycles consumed by the instruction.
    /// If the CPU is in the halt state, the function does nothing but returns a fixed clock cycle.
    pub fn execute(&mut self) -> usize {
        if self.halt {
            self.step(4);
        } else {
            let code = self.fetch_opcode();
            let time = self.decode(code);
            assert_eq!(self.cycles, time, "cycle mismatch op={:04x}", code);
        }

        self.update_ime();
        self.check_interrupt();

        // Get the cycles consumed and reset
        let cycles = self.cycles;
        self.cycles = 0;
        cycles
    }

    /// Step forward
    pub fn step(&mut self, cycles: usize) {
        self.cycles = self.cycles.wrapping_add(cycles);
        self.sys.step(cycles);
    }

    /// Handles DI
    pub fn di(&mut self) {
        self.di_delay = 2;
    }

    /// Handles EI
    pub fn ei(&mut self) {
        self.ei_delay = 2;
    }

    /// Update IME
    fn update_ime(&mut self) {
        if self.di_delay > 0 {
            if self.di_delay == 1 {
                self.ime = false;
            }
            self.di_delay -= 1;
        }

        if self.ei_delay > 0 {
            if self.ei_delay == 1 {
                self.ime = true;
            }
            self.ei_delay -= 1;
        }
    }

    /// Disable interrupts to this CPU.
    pub fn disable_interrupt(&mut self) {
        debug!("Disable interrupt");
        self.ime = false;
    }

    /// Enable interrupts to this CPU.
    pub fn enable_interrupt(&mut self) {
        debug!("Enable interrupt");
        self.ime = true;
    }

    /// Check if pending interrupts in the interrupt controller,
    /// and process them if any.
    fn check_interrupt(&mut self) {
        if !self.ime {
            if self.halt && self.sys.peek_int_vec().is_some() {
                // If HALT is executed while interrupt is disabled,
                // the interrupt wakes up CPU without being consumed.
                self.step(4);
                self.halt = false;
            }
            return;
        }

        let value = match self.sys.pop_int_vec() {
            Some(value) => value,
            None => return,
        };

        info!("Interrupted: {:02x}", value);

        self.interrupted(value);

        if self.halt {
            self.step(24);
        } else {
            self.step(20);
        }

        self.halt = false;
    }

    fn interrupted(&mut self, vector_addr: u8) {
        self.disable_interrupt();

        self.push(self.get_pc());
        self.set_pc(vector_addr as u16);
    }

    /// Stop the CPU.
    pub fn stop(&self) {
        todo!("stop {:04x}", self.get_pc());
    }

    /// Gets the value of `z` flag in the flag register.
    pub fn get_zf(&self) -> bool {
        self.f & 0x80 == 0x80
    }

    /// Gets the value of `n` flag in the flag register.
    pub fn get_nf(&self) -> bool {
        self.f & 0x40 == 0x40
    }

    /// Gets the value of `h` flag in the flag register.
    pub fn get_hf(&self) -> bool {
        self.f & 0x20 == 0x20
    }

    /// Gets the value of `c` flag in the flag register.
    pub fn get_cf(&self) -> bool {
        self.f & 0x10 == 0x10
    }

    /// Updates the value of `z` flag in the flag register.
    pub fn set_zf(&mut self, v: bool) {
        if v {
            self.f = self.f | 0x80
        } else {
            self.f = self.f & !0x80
        }
    }

    /// Updates the value of `n` flag in the flag register.
    pub fn set_nf(&mut self, v: bool) {
        if v {
            self.f = self.f | 0x40
        } else {
            self.f = self.f & !0x40
        }
    }

    /// Updates the value of `h` flag in the flag register.
    pub fn set_hf(&mut self, v: bool) {
        if v {
            self.f = self.f | 0x20
        } else {
            self.f = self.f & !0x20
        }
    }

    /// Updates the value of `c` flag in the flag register.
    pub fn set_cf(&mut self, v: bool) {
        if v {
            self.f = self.f | 0x10
        } else {
            self.f = self.f & !0x10
        }
    }

    /// Updates the value of `a` register.
    pub fn set_a(&mut self, v: u8) {
        self.a = v
    }

    /// Updates the value of `b` register.
    pub fn set_b(&mut self, v: u8) {
        self.b = v
    }

    /// Updates the value of `c` register.
    pub fn set_c(&mut self, v: u8) {
        self.c = v
    }

    /// Updates the value of `d` register.
    pub fn set_d(&mut self, v: u8) {
        self.d = v
    }

    /// Updates the value of `e` register.
    pub fn set_e(&mut self, v: u8) {
        self.e = v
    }

    /// Updates the value of `h` register.
    pub fn set_h(&mut self, v: u8) {
        self.h = v
    }

    /// Updates the value of `l` register.
    pub fn set_l(&mut self, v: u8) {
        self.l = v
    }

    /// Updates the value of `a` and `f` register as a single 16-bit register.
    pub fn set_af(&mut self, v: u16) {
        self.a = (v >> 8) as u8;
        self.f = (v & 0xf0) as u8;
    }

    /// Updates the value of `b` and `c` register as a single 16-bit register.
    pub fn set_bc(&mut self, v: u16) {
        self.b = (v >> 8) as u8;
        self.c = v as u8;
    }

    /// Updates the value of `d` and `e` register as a single 16-bit register
    pub fn set_de(&mut self, v: u16) {
        self.d = (v >> 8) as u8;
        self.e = v as u8;
    }

    /// Updates the value of `h` and `l` register as a single 16-bit register.
    pub fn set_hl(&mut self, v: u16) {
        self.h = (v >> 8) as u8;
        self.l = v as u8;
    }

    /// Gets the value of `a` register.
    pub fn get_a(&self) -> u8 {
        self.a
    }

    /// Gets the value of `b` register.
    pub fn get_b(&self) -> u8 {
        self.b
    }

    /// Gets the value of `c` register.
    pub fn get_c(&self) -> u8 {
        self.c
    }

    /// Gets the value of `d` register.
    pub fn get_d(&self) -> u8 {
        self.d
    }

    /// Gets the value of `e` register.
    pub fn get_e(&self) -> u8 {
        self.e
    }

    /// Gets the value of `h` register.
    pub fn get_h(&self) -> u8 {
        self.h
    }

    /// Gets the value of `l` register.
    pub fn get_l(&self) -> u8 {
        self.l
    }

    /// Gets the value of `a` and `f` register as a single 16-bit register.
    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }

    /// Gets the value of `b` and `c` register as a single 16-bit register.
    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    /// Gets the value of `d` and `e` register as a single 16-bit register.
    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    /// Gets the value of `h` and `l` register as a single 16-bit register.
    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    /// Gets the value of the program counter.
    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    /// Updates the value of the program counter.
    pub fn set_pc(&mut self, v: u16) {
        self.pc = v
    }

    /// Gets the value of the stack pointer register.
    pub fn get_sp(&self) -> u16 {
        self.sp
    }

    /// Updates the value of the stack pointer register.
    pub fn set_sp(&mut self, v: u16) {
        self.sp = v
    }

    /// Jump
    pub fn jump(&mut self, a: u16) {
        self.step(4);
        self.set_pc(a);
    }

    /// Read a byte from memory
    pub fn get8(&mut self, a: u16) -> u8 {
        self.step(4);
        self.sys.get8(a)
    }

    /// Read a word from memory
    pub fn get16(&mut self, a: u16) -> u16 {
        self.step(8);
        self.sys.get16(a)
    }

    /// Write a byte to memory
    pub fn set8(&mut self, a: u16, v: u8) {
        self.step(4);
        self.sys.set8(a, v);
    }

    /// Write a word to memory
    pub fn set16(&mut self, a: u16, v: u16) {
        self.step(8);
        self.sys.set16(a, v)
    }

    /// Pushes a 16-bit value to the stack, updating the stack pointer register.
    pub fn push(&mut self, v: u16) {
        let p = self.get_sp().wrapping_sub(2);
        self.set_sp(self.get_sp().wrapping_sub(2));
        self.set16(p, v)
    }

    /// Pops a 16-bit value from the stack, updating the stack pointer register.
    pub fn pop(&mut self) -> u16 {
        let p = self.get_sp();
        self.set_sp(self.get_sp().wrapping_add(2));
        self.get16(p)
    }

    /// Fetch a byte consuming cycles
    pub fn fetch8(&mut self) -> u8 {
        let b = self.get8(self.get_pc());
        self.inc_pc();
        b
    }

    /// Fetch a word consuming cycles
    pub fn fetch16(&mut self) -> u16 {
        let l = self.fetch8();
        let h = self.fetch8();
        (h as u16) << 8 | l as u16
    }

    /// Fetch next instruction without consuming cycles
    fn prefetch(&mut self) -> u8 {
        let b = self.sys.get8(self.get_pc());
        self.inc_pc();
        b
    }

    /// Add 1 to pc unless HALT bug is triggerred
    fn inc_pc(&mut self) {
        if self.halt_bug {
            info!("Halt bug");
            self.halt_bug = false;
        } else {
            self.set_pc(self.get_pc().wrapping_add(1));
        }
    }

    /// Fetches an opcode from the memory and returns it with its length.
    pub fn fetch_opcode(&mut self) -> u16 {
        match self.prefetch() {
            0xcb => 0xcb00 | self.fetch8() as u16,
            b => b as u16,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mmu::Ram;

    fn exec(cpu: &mut Cpu<Ram>) {
        let code = cpu.fetch_opcode();
        cpu.decode(code);
    }

    #[test]
    fn op_00af() {
        // xor a
        let mut cpu = Cpu::new(Ram::new());

        cpu.set_a(0x32);

        cpu.sys.write(&[0xaf]);
        exec(&mut cpu);

        assert_eq!(cpu.get_a(), 0x00);
    }

    #[test]
    fn op_00f1() {
        // pop af
        let mut cpu = Cpu::new(Ram::new());

        cpu.set_bc(0x1301);
        cpu.sys
            .write(&[0xc5, 0xf1, 0xf5, 0xd1, 0x79, 0xe6, 0xf0, 0xbb]);
        exec(&mut cpu); // push bc
        assert_eq!(cpu.get_bc(), 0x1301);
        exec(&mut cpu); // pop af
        assert_eq!(cpu.get_af(), 0x1300); // because the lower 4 bits of `f` are always zero
        exec(&mut cpu); // push af
        exec(&mut cpu); // pop de
        assert_eq!(cpu.get_de(), 0x1300);
        assert_eq!(cpu.get_c(), 0x01);
        exec(&mut cpu); // ld a,c
        assert_eq!(cpu.get_a(), 0x01);
        assert_eq!(cpu.get_c(), 0x01);
        exec(&mut cpu); // and 0xf0
        assert_eq!(cpu.get_a(), 0x00);
        assert_eq!(cpu.get_e(), 0x00);
        exec(&mut cpu); // cp e
        assert_eq!(cpu.get_zf(), true);
    }
}
