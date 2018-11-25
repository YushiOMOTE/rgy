use crate::mmu::{Mmu, ReadHandler, WriteHandler};

use std::cell::{Cell, RefCell};

pub struct Gpu {
    clocks: Cell<usize>,
    clocksbase: Cell<usize>,
    enable: Cell<bool>,
    winbase: Cell<u16>,
    winenable: Cell<bool>,
    bgwinbase: Cell<u16>,
    bgbase: Cell<u16>,
    spsize: Cell<u16>,
    spenable: Cell<bool>,
    bgenable: Cell<bool>,
    vram: RefCell<Vec<u32>>,
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            clocks: Cell::new(0),
            clocksbase: Cell::new(0),
            enable: Cell::new(false),
            winbase: Cell::new(0x9800),
            winenable: Cell::new(false),
            bgwinbase: Cell::new(0x8800),
            bgbase: Cell::new(0x9800),
            spsize: Cell::new(8),
            spenable: Cell::new(false),
            bgenable: Cell::new(false),
            vram: RefCell::new(vec![0; 256 * 256]),
        }
    }

    pub fn step(&self, time: usize) {
        self.clocks.set(self.clocks.get().wrapping_add(time));
    }

    pub fn fetch(&self, mmu: &Mmu) {}

    fn on_write_ctrl(&self, value: u8) {
        let old_enable = self.enable.get();

        self.enable.set(value & 0x80 != 0);
        self.winbase
            .set(if value & 0x40 != 0 { 0x9c00 } else { 0x9800 });
        self.winenable.set(value & 0x20 != 0);
        self.bgwinbase
            .set(if value & 0x10 != 0 { 0x8000 } else { 0x8800 });
        self.bgbase
            .set(if value & 0x08 != 0 { 0x9c00 } else { 0x9800 });
        self.spsize.set(if value & 0x04 != 0 { 8 } else { 16 });
        self.spenable.set(value & 0x02 != 0);
        self.bgenable.set(value & 0x01 != 0);

        if !old_enable && self.enable.get() {
            info!("LCD enabled");
            self.clocksbase.set(self.clocksbase.get());
        } else if old_enable && !self.enable.get() {
            info!("LCD disabled");
        }
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

        if addr == 0xff40 {
            self.on_write_ctrl(value);
        }

        None
    }
}
