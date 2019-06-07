use crate::device::{HardwareHandle, Key};
use crate::ic::Irq;
use crate::mmu::{MemHandler, MemRead, MemWrite, Mmu};
use log::*;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Keyboard {
    fn pressed(&self, key: char) -> bool;
}

pub struct Joypad {
    inner: Rc<RefCell<Inner>>,
}

impl Joypad {
    pub fn new(hw: HardwareHandle, irq: Irq) -> Self {
        Self {
            inner: Rc::new(RefCell::new(Inner::new(hw, irq))),
        }
    }

    pub fn handler(&self) -> JoypadMemHandler {
        JoypadMemHandler::new(self.inner.clone())
    }
}

struct Inner {
    hw: HardwareHandle,
    irq: Irq,
    select: u8,
}

impl Inner {
    fn new(hw: HardwareHandle, irq: Irq) -> Self {
        Self {
            hw,
            irq,
            select: 0xff,
        }
    }

    fn on_read(&mut self, mmu: &Mmu, addr: u16) -> MemRead {
        if addr == 0xff00 {
            let p = |key| self.hw.get().borrow_mut().joypad_pressed(key);
            info!("Joypad read: dir: {:02x}", self.select);
            let mut value = 0xff & self.select;
            if self.select & 0x10 == 0 {
                value |= if p(Key::Right) { 0x00 } else { 0x01 };
                value |= if p(Key::Left) { 0x00 } else { 0x02 };
                value |= if p(Key::Up) { 0x00 } else { 0x04 };
                value |= if p(Key::Down) { 0x00 } else { 0x08 };
            }
            if self.select & 0x20 == 0 {
                value |= if p(Key::A) { 0x00 } else { 0x01 };
                value |= if p(Key::B) { 0x00 } else { 0x02 };
                value |= if p(Key::Select) { 0x00 } else { 0x04 };
                value |= if p(Key::Start) { 0x0 } else { 0x08 };
            }
            MemRead::Replace(value)
        } else {
            MemRead::PassThrough
        }
    }

    fn on_write(&mut self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        if addr == 0xff00 {
            self.select = value;
        }
        MemWrite::PassThrough
    }
}

pub struct JoypadMemHandler {
    inner: Rc<RefCell<Inner>>,
}

impl JoypadMemHandler {
    fn new(inner: Rc<RefCell<Inner>>) -> Self {
        Self { inner }
    }
}

impl MemHandler for JoypadMemHandler {
    fn on_read(&self, mmu: &Mmu, addr: u16) -> MemRead {
        self.inner.borrow_mut().on_read(mmu, addr)
    }

    fn on_write(&self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        self.inner.borrow_mut().on_write(mmu, addr, value)
    }
}
