use crate::mmu::{Mmu, ReadHandler, WriteHandler};

use std::cell::Cell;

pub struct Gpu {
    clocks: Cell<usize>,
    clocksbase: Cell<usize>,
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            clocks: Cell::new(0),
            clocksbase: Cell::new(0),
        }
    }

    pub fn step(&self, time: usize) {
        self.clocks.set(self.clocks.get().wrapping_add(time));
    }
}

impl ReadHandler for Gpu {
    fn on_read(&self, mmu: &Mmu, addr: u16) -> Option<u8> {
        trace!("Read GPU register: {:04x}", addr);

        if addr == 0xff44 {
            let clks = self.clocks.get();
            let clksbase = self.clocksbase.get();

            let ly = ((clks.wrapping_sub(clksbase) % 70224) / 456) as u8;

            Some(ly)
        } else {
            None
        }
    }
}

impl WriteHandler for Gpu {
    fn on_write(&self, mmu: &Mmu, addr: u16, value: u8) -> Option<u8> {
        trace!("Write GPU register: {:04x} {:02x}", addr, value);

        None
    }
}
