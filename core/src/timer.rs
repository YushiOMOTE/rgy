use crate::device::IoHandler;
use crate::ic::Irq;
use crate::mmu::{MemRead, MemWrite, Mmu};
use log::*;

pub struct Timer {
    irq: Irq,
    div: u8,
    div_clocks: usize,
    tim: u8,
    tim_clocks: usize,
    tim_load: u8,
    ctrl: u8,
}

impl Timer {
    pub fn new(irq: Irq) -> Self {
        Self {
            irq,
            div: 0,
            div_clocks: 0,
            tim: 0,
            tim_clocks: 0,
            tim_load: 0,
            ctrl: 0,
        }
    }

    fn tim_clock_reset(&mut self) {
        self.tim_clocks = match self.ctrl & 0x3 {
            0x0 => 1024, // 4096Hz = 1024 cpu clocks
            0x1 => 16,   // 262144Hz = 16 cpu clocks
            0x2 => 64,   // 65536Hz = 64 cpu clocks
            0x3 => 256,  // 16384Hz = 256 cpu clocks
            _ => unreachable!(),
        };
    }

    fn div_clock_reset(&mut self) {
        self.div_clocks = 256; // 16384Hz = 256 cpu clocks
    }

    pub fn step(&mut self, time: usize) {
        if self.div_clocks < time {
            self.div = self.div.wrapping_add(1);
            self.div_clock_reset();
            self.div_clocks -= time - self.div_clocks;
        } else {
            self.div_clocks -= time;
        }

        if self.ctrl & 0x04 == 0 {
            return;
        }

        if self.tim_clocks < time {
            let (tim, of) = self.tim.overflowing_add(1);
            self.tim = tim;
            if of {
                self.tim = self.tim_load;
                self.irq.timer(true);
            }
            self.tim_clock_reset();
            self.tim_clocks -= time - self.tim_clocks;
        } else {
            self.tim_clocks -= time;
        }
    }
}

impl IoHandler for Timer {
    fn on_read(&mut self, _mmu: &Mmu, addr: u16) -> MemRead {
        info!("Timer read: {:04x}", addr);
        match addr {
            0xff04 => MemRead::Replace(self.div),
            0xff05 => MemRead::Replace(self.tim),
            0xff06 => MemRead::Replace(self.tim_load),
            0xff07 => MemRead::Replace(self.ctrl),
            _ => MemRead::PassThrough,
        }
    }

    fn on_write(&mut self, _mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        info!("Timer write: {:04x} {:02x}", addr, value);
        match addr {
            0xff04 => self.div = 0,
            0xff05 => self.tim = value,
            0xff06 => self.tim_load = value,
            0xff07 => self.ctrl = value,
            _ => {}
        }
        MemWrite::PassThrough
    }
}
