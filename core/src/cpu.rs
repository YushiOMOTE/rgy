use crate::device::Device;
use crate::ic::Ic;
use crate::inst::decode;
use crate::mmu::Mmu;
use log::*;

use std::fmt;

#[derive(Clone)]
pub struct Cpu {
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
    halt: bool,
}

impl fmt::Display for Cpu {
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

impl Cpu {
    pub fn new() -> Cpu {
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
            halt: false,
        }
    }

    pub fn halt(&mut self) {
        debug!("Halted");
        // TODO: self.halt = true;
    }

    pub fn execute(&mut self, mmu: &mut Mmu) -> usize {
        if self.halt {
            4
        } else {
            let (code, arg) = self.fetch(mmu);
            let (time, size) = decode(code, arg, self, mmu);
            self.set_pc(self.get_pc().wrapping_add(size as u16));
            time
        }
    }

    pub fn disable_interrupt(&mut self) {
        debug!("Disable interrupt");
        self.ime = false;
    }

    pub fn enable_interrupt(&mut self) {
        debug!("Enable interrupt");
        self.ime = true;
    }

    pub fn check_interrupt(&mut self, mmu: &mut Mmu, ic: &Device<Ic>) -> usize {
        if !self.ime {
            if self.halt {
                // If HALT is executed while interrupt is disabled,
                // the interrupt wakes up CPU without being consumed.
                if let Some(value) = ic.borrow_mut().peek() {
                    debug!("Interrupted on halt + ime=0: {:02x}", value);
                    self.halt = false;
                }
            }

            0
        } else if let Some(value) = ic.borrow_mut().poll() {
            debug!("Interrupted: {:02x}", value);

            self.interrupted(mmu, value);

            self.halt = false;

            16
        } else {
            0
        }
    }

    fn interrupted(&mut self, mmu: &mut Mmu, value: u8) {
        self.disable_interrupt();

        self.push(mmu, self.get_pc());
        self.set_pc(value as u16);
    }

    pub fn stop(&self) {}

    pub fn get_zf(&self) -> bool {
        self.f & 0x80 == 0x80
    }

    pub fn get_nf(&self) -> bool {
        self.f & 0x40 == 0x40
    }

    pub fn get_hf(&self) -> bool {
        self.f & 0x20 == 0x20
    }

    pub fn get_cf(&self) -> bool {
        self.f & 0x10 == 0x10
    }

    pub fn set_zf(&mut self, v: bool) {
        if v {
            self.f = self.f | 0x80
        } else {
            self.f = self.f & !0x80
        }
    }

    pub fn set_nf(&mut self, v: bool) {
        if v {
            self.f = self.f | 0x40
        } else {
            self.f = self.f & !0x40
        }
    }

    pub fn set_hf(&mut self, v: bool) {
        if v {
            self.f = self.f | 0x20
        } else {
            self.f = self.f & !0x20
        }
    }

    pub fn set_cf(&mut self, v: bool) {
        if v {
            self.f = self.f | 0x10
        } else {
            self.f = self.f & !0x10
        }
    }

    pub fn set_a(&mut self, v: u8) {
        self.a = v
    }

    pub fn set_b(&mut self, v: u8) {
        self.b = v
    }

    pub fn set_c(&mut self, v: u8) {
        self.c = v
    }

    pub fn set_d(&mut self, v: u8) {
        self.d = v
    }

    pub fn set_e(&mut self, v: u8) {
        self.e = v
    }

    pub fn set_h(&mut self, v: u8) {
        self.h = v
    }

    pub fn set_l(&mut self, v: u8) {
        self.l = v
    }

    pub fn set_af(&mut self, v: u16) {
        self.a = (v >> 8) as u8;
        self.f = (v & 0xf0) as u8;
    }

    pub fn set_bc(&mut self, v: u16) {
        self.b = (v >> 8) as u8;
        self.c = v as u8;
    }

    pub fn set_de(&mut self, v: u16) {
        self.d = (v >> 8) as u8;
        self.e = v as u8;
    }

    pub fn set_hl(&mut self, v: u16) {
        self.h = (v >> 8) as u8;
        self.l = v as u8;
    }

    pub fn get_a(&self) -> u8 {
        self.a
    }

    pub fn get_b(&self) -> u8 {
        self.b
    }

    pub fn get_c(&self) -> u8 {
        self.c
    }

    pub fn get_d(&self) -> u8 {
        self.d
    }

    pub fn get_e(&self) -> u8 {
        self.e
    }

    pub fn get_h(&self) -> u8 {
        self.h
    }

    pub fn get_l(&self) -> u8 {
        self.l
    }

    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn set_pc(&mut self, v: u16) {
        self.pc = v
    }

    pub fn get_sp(&self) -> u16 {
        self.sp
    }

    pub fn set_sp(&mut self, v: u16) {
        self.sp = v
    }

    pub fn push(&mut self, mmu: &mut Mmu, v: u16) {
        let p = self.get_sp().wrapping_sub(2);
        self.set_sp(self.get_sp().wrapping_sub(2));
        mmu.set16(p, v)
    }

    pub fn pop(&mut self, mmu: &mut Mmu) -> u16 {
        let p = self.get_sp();
        self.set_sp(self.get_sp().wrapping_add(2));
        mmu.get16(p)
    }

    pub fn fetch(&self, mmu: &Mmu) -> (u16, u16) {
        let pc = self.get_pc();

        let fb = mmu.get8(pc);

        if fb == 0xcb {
            let sb = mmu.get8(pc + 1);
            (0xcb00 | sb as u16, 2)
        } else {
            (fb as u16, 1)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::inst::decode;

    fn write(mmu: &mut Mmu, m: Vec<u8>) {
        for i in 0..m.len() {
            mmu.set8(i as u16, m[i]);
        }
    }

    fn exec(cpu: &mut Cpu, mmu: &mut Mmu) {
        let (code, arg) = cpu.fetch(&mmu);

        let (_, size) = decode(code, arg, cpu, mmu);

        cpu.set_pc(cpu.get_pc().wrapping_add(size as u16));
    }

    #[test]
    fn op_00af() {
        // xor a
        let mut mmu = Mmu::new();
        let mut cpu = Cpu::new();

        cpu.set_a(0x32);

        write(&mut mmu, vec![0xaf]);
        exec(&mut cpu, &mut mmu);

        assert_eq!(cpu.get_a(), 0x00);
    }

    #[test]
    fn op_00f1() {
        // pop af
        let mut mmu = Mmu::new();
        let mut cpu = Cpu::new();

        cpu.set_bc(0x1301);
        write(
            &mut mmu,
            vec![0xc5, 0xf1, 0xf5, 0xd1, 0x79, 0xe6, 0xf0, 0xbb],
        );
        exec(&mut cpu, &mut mmu); // push bc
        assert_eq!(cpu.get_bc(), 0x1301);
        exec(&mut cpu, &mut mmu); // pop af
        assert_eq!(cpu.get_af(), 0x1300); // because the lower 4 bits of `f` are always zero
        exec(&mut cpu, &mut mmu); // push af
        exec(&mut cpu, &mut mmu); // pop de
        assert_eq!(cpu.get_de(), 0x1300);
        assert_eq!(cpu.get_c(), 0x01);
        exec(&mut cpu, &mut mmu); // ld a,c
        assert_eq!(cpu.get_a(), 0x01);
        assert_eq!(cpu.get_c(), 0x01);
        exec(&mut cpu, &mut mmu); // and 0xf0
        assert_eq!(cpu.get_a(), 0x00);
        assert_eq!(cpu.get_e(), 0x00);
        exec(&mut cpu, &mut mmu); // cp e
        assert_eq!(cpu.get_zf(), true);
    }
}
