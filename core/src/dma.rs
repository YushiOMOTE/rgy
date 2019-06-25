use crate::device::IoHandler;
use crate::mmu::{MemRead, MemWrite, Mmu};
use log::*;

pub struct Dma {
    on: bool,
    src: u8,
}

impl Dma {
    pub fn new() -> Self {
        Self { on: false, src: 0 }
    }

    pub fn step(&mut self, mmu: &mut Mmu) {
        if self.on {
            assert!(self.src <= 0x80 || self.src >= 0x9f);
            debug!("Perform DMA transfer: {:02x}", self.src);

            let src = (self.src as u16) << 8;
            for i in 0..0xa0 {
                mmu.set8(0xfe00 + i, mmu.get8(src + i));
            }

            self.on = false;
        }
    }
}

impl IoHandler for Dma {
    fn on_write(&mut self, _mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        assert_eq!(addr, 0xff46);
        debug!("Start DMA transfer: {:02x}", self.src);
        self.on = true;
        self.src = value;
        MemWrite::Block
    }

    fn on_read(&mut self, _mmu: &Mmu, _addr: u16) -> MemRead {
        MemRead::Replace(0)
    }
}
