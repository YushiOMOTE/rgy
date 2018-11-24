use std::cell::Cell;
use crate::mmu::Mmu;

pub struct Cpu {
    a: Cell<u8>,
    b: Cell<u8>,
    c: Cell<u8>,
    d: Cell<u8>,
    e: Cell<u8>,
    f: Cell<u8>,
    h: Cell<u8>,
    l: Cell<u8>,
    pc: Cell<u16>,
    sp: Cell<u16>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: Cell::new(0),
            b: Cell::new(0),
            c: Cell::new(0),
            d: Cell::new(0),
            e: Cell::new(0),
            f: Cell::new(0),
            h: Cell::new(0),
            l: Cell::new(0),
            pc: Cell::new(0),
            sp: Cell::new(0),
        }
    }

    pub fn halt(&self) {}

    pub fn disable_interrupt(&self) {}

    pub fn enable_interrupt(&self) {}

    pub fn enable_interrupt_immediate(&self) {}

    pub fn stop(&self) {}

    pub fn get_zf(&self) -> bool {
        self.f.get() & 0x80 == 0x80
    }

    pub fn get_nf(&self) -> bool {
        self.f.get() & 0x40 == 0x40
    }

    pub fn get_hf(&self) -> bool {
        self.f.get() & 0x20 == 0x20
    }

    pub fn get_cf(&self) -> bool {
        self.f.get() & 0x10 == 0x10
    }

    pub fn set_zf(&self, v: bool) {
        if v {
            self.f.set(self.f.get() | 0x80)
        } else {
            self.f.set(self.f.get() & !0x80)
        }
    }

    pub fn set_nf(&self, v: bool) {
        if v {
            self.f.set(self.f.get() | 0x40)
        } else {
            self.f.set(self.f.get() & !0x40)
        }
    }

    pub fn set_hf(&self, v: bool) {
        if v {
            self.f.set(self.f.get() | 0x20)
        } else {
            self.f.set(self.f.get() & !0x20)
        }
    }

    pub fn set_cf(&self, v: bool) {
        if v {
            self.f.set(self.f.get() | 0x10)
        } else {
            self.f.set(self.f.get() & !0x10)
        }
    }

    pub fn set_a(&self, v: u8) {
        self.a.set(v)
    }

    pub fn set_b(&self, v: u8) {
        self.b.set(v)
    }

    pub fn set_c(&self, v: u8) {
        self.c.set(v)
    }

    pub fn set_d(&self, v: u8) {
        self.d.set(v)
    }

    pub fn set_e(&self, v: u8) {
        self.e.set(v)
    }

    pub fn set_f(&self, v: u8) {
        self.f.set(v)
    }

    pub fn set_h(&self, v: u8) {
        self.h.set(v)
    }

    pub fn set_l(&self, v: u8) {
        self.l.set(v)
    }

    pub fn set_af(&self, v: u16) {
        self.a.set((v >> 8) as u8);
        self.f.set(v as u8);
    }

    pub fn set_bc(&self, v: u16) {
        self.b.set((v >> 8) as u8);
        self.c.set(v as u8);
    }

    pub fn set_de(&self, v: u16) {
        self.d.set((v >> 8) as u8);
        self.e.set(v as u8);
    }

    pub fn set_hl(&self, v: u16) {
        self.h.set((v >> 8) as u8);
        self.l.set(v as u8);
    }

    pub fn get_a(&self) -> u8 {
        self.a.get()
    }

    pub fn get_b(&self) -> u8 {
        self.b.get()
    }

    pub fn get_c(&self) -> u8 {
        self.c.get()
    }

    pub fn get_d(&self) -> u8 {
        self.d.get()
    }

    pub fn get_e(&self) -> u8 {
        self.e.get()
    }

    pub fn get_f(&self) -> u8 {
        self.f.get()
    }

    pub fn get_h(&self) -> u8 {
        self.h.get()
    }

    pub fn get_l(&self) -> u8 {
        self.l.get()
    }

    pub fn get_af(&self) -> u16 {
        (self.a.get() as u16) << 8 | self.f.get() as u16
    }

    pub fn get_bc(&self) -> u16 {
        (self.b.get() as u16) << 8 | self.c.get() as u16
    }

    pub fn get_de(&self) -> u16 {
        (self.d.get() as u16) << 8 | self.e.get() as u16
    }

    pub fn get_hl(&self) -> u16 {
        (self.h.get() as u16) << 8 | self.l.get() as u16
    }

    pub fn get_pc(&self) -> u16 {
        self.pc.get()
    }

    pub fn set_pc(&self, v: u16) {
        self.pc.set(v)
    }

    pub fn get_sp(&self) -> u16 {
        self.sp.get()
    }

    pub fn set_sp(&self, v: u16) {
        self.sp.set(v)
    }

    pub fn push(&self, mmu: &Mmu, v: u16) {
        let p = self.get_sp().wrapping_sub(1);
        self.set_sp(self.get_sp().wrapping_sub(2));
        mmu.set16(p, v)
    }

    pub fn pop(&self, mmu: &Mmu) -> u16 {
        let p = self.get_sp();
        self.set_sp(self.get_sp().wrapping_add(2));
        mmu.get16(p)
    }
}
