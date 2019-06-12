use crate::device::IoHandler;
use crate::mmu::{MemRead, MemWrite, Mmu};
use log::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Irq {
    request: Rc<RefCell<Ints>>,
}

impl Irq {
    fn new(request: Rc<RefCell<Ints>>) -> Irq {
        Irq { request }
    }

    pub fn vblank(&self, v: bool) {
        self.request.borrow_mut().vblank = v;
    }

    pub fn lcd(&self, v: bool) {
        self.request.borrow_mut().lcd = v;
    }

    pub fn timer(&self, v: bool) {
        self.request.borrow_mut().timer = v;
    }

    pub fn serial(&self, v: bool) {
        self.request.borrow_mut().serial = v;
    }

    pub fn joypad(&self, v: bool) {
        self.request.borrow_mut().joypad = v;
    }
}

#[derive(Debug, Default)]
struct Ints {
    vblank: bool,
    lcd: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
}

impl Ints {
    fn set(&mut self, value: u8) {
        self.vblank = value & 0x01 != 0;
        self.lcd = value & 0x02 != 0;
        self.timer = value & 0x04 != 0;
        self.serial = value & 0x08 != 0;
        self.joypad = value & 0x10 != 0;
    }

    fn get(&self) -> u8 {
        let mut v = 0;
        v |= if self.vblank { 0x01 } else { 0x00 };
        v |= if self.lcd { 0x02 } else { 0x00 };
        v |= if self.timer { 0x04 } else { 0x00 };
        v |= if self.serial { 0x08 } else { 0x00 };
        v |= if self.joypad { 0x10 } else { 0x00 };
        v
    }
}

pub struct Ic {
    enable: Rc<RefCell<Ints>>,
    request: Rc<RefCell<Ints>>,
}

impl Ic {
    pub fn new() -> Ic {
        Ic {
            enable: Rc::new(RefCell::new(Ints::default())),
            request: Rc::new(RefCell::new(Ints::default())),
        }
    }

    pub fn irq(&self) -> Irq {
        Irq::new(self.request.clone())
    }

    pub fn peek(&self) -> Option<u8> {
        self.check(false)
    }

    pub fn poll(&self) -> Option<u8> {
        self.check(true)
    }

    fn check(&self, consume: bool) -> Option<u8> {
        let e = self.enable.borrow();
        let mut r = self.request.borrow_mut();

        if e.vblank && r.vblank {
            r.vblank = !consume;
            Some(0x40)
        } else if e.lcd && r.lcd {
            r.lcd = !consume;
            Some(0x48)
        } else if e.timer && r.timer {
            r.timer = !consume;
            Some(0x50)
        } else if e.serial && r.serial {
            r.serial = !consume;
            Some(0x58)
        } else if e.joypad && r.joypad {
            r.joypad = !consume;
            Some(0x60)
        } else {
            None
        }
    }
}

impl IoHandler for Ic {
    fn on_read(&mut self, _mmu: &Mmu, addr: u16) -> MemRead {
        if addr == 0xffff {
            let v = self.enable.borrow().get();
            info!("Read interrupt enable: {:02x}", v);
            MemRead::Replace(v)
        } else if addr == 0xff0f {
            let v = self.request.borrow().get();
            info!("Read interrupt: {:02x}", v);
            MemRead::Replace(v)
        } else {
            MemRead::PassThrough
        }
    }

    fn on_write(&mut self, _mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        if addr == 0xffff {
            info!("Write interrupt enable: {:02x}", value);
            self.enable.borrow_mut().set(value);
            MemWrite::Block
        } else if addr == 0xff0f {
            info!("Write interrupt: {:02x}", value);
            self.request.borrow_mut().set(value);
            MemWrite::Block
        } else {
            MemWrite::PassThrough
        }
    }
}
