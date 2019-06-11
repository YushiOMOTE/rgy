use crate::device::IoHandler;
use crate::hardware::{HardwareHandle, Key};
use crate::ic::Irq;
use crate::mmu::{MemRead, MemWrite, Mmu};
use log::*;

pub struct Joypad {
    hw: HardwareHandle,
    irq: Irq,
    select: u8,
}

impl Joypad {
    pub fn new(hw: HardwareHandle, irq: Irq) -> Self {
        Self {
            hw,
            irq,
            select: 0xff,
        }
    }
}

impl IoHandler for Joypad {
    fn on_read(&mut self, _mmu: &Mmu, addr: u16) -> MemRead {
        if addr == 0xff00 {
            let p = |key| self.hw.get().borrow_mut().joypad_pressed(key);

            debug!("Joypad read: dir: {:02x}", self.select);

            let mut value = 0;

            if self.select & 0x10 == 0 {
                value |= if p(Key::Right) { 0x00 } else { 0x01 };
                value |= if p(Key::Left) { 0x00 } else { 0x02 };
                value |= if p(Key::Up) { 0x00 } else { 0x04 };
                value |= if p(Key::Down) { 0x00 } else { 0x08 };
            } else if self.select & 0x20 == 0 {
                value |= if p(Key::A) { 0x00 } else { 0x01 };
                value |= if p(Key::B) { 0x00 } else { 0x02 };
                value |= if p(Key::Select) { 0x00 } else { 0x04 };
                value |= if p(Key::Start) { 0x0 } else { 0x08 };
            } else {
                value = 0x0f;
            }

            MemRead::Replace(value)
        } else {
            MemRead::PassThrough
        }
    }

    fn on_write(&mut self, _mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        if addr == 0xff00 {
            self.select = value & 0xf0;
        }
        MemWrite::PassThrough
    }
}
