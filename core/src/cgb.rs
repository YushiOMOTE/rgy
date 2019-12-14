use crate::{
    device::IoHandler,
    mmu::{MemRead, MemWrite, Mmu},
};
use alloc::{vec, vec::Vec};
use log::*;

pub struct Cgb {
    double_speed: bool,
    speed_switch: bool,
    wram_select: usize,
    wram_bank: Vec<Vec<u8>>,
}

#[allow(unused)]
impl Cgb {
    pub fn new() -> Self {
        Self {
            double_speed: false,
            speed_switch: false,
            wram_select: 1,
            wram_bank: (0..8).map(|_| vec![0; 0x1000]).collect(),
        }
    }

    pub fn try_switch_speed(&mut self) {
        if self.speed_switch {
            self.double_speed = !self.double_speed;
            self.speed_switch = false;
        }
    }

    pub fn double_speed(&self) -> bool {
        self.double_speed
    }
}

impl IoHandler for Cgb {
    fn on_read(&mut self, _mmu: &Mmu, addr: u16) -> MemRead {
        if addr >= 0xc000 && addr <= 0xcfff {
            let off = addr as usize - 0xc000;
            MemRead::Replace(self.wram_bank[0][off])
        } else if addr >= 0xd000 && addr <= 0xdfff {
            let off = addr as usize - 0xd000;
            MemRead::Replace(self.wram_bank[self.wram_select][off])
        } else if addr == 0xff4d {
            let mut v = 0;
            v |= if self.double_speed { 0x80 } else { 0x00 };
            v |= if self.speed_switch { 0x01 } else { 0x00 };
            MemRead::Replace(v)
        } else if addr == 0xff56 {
            warn!("Infrared read");
            MemRead::PassThrough
        } else if addr == 0xff70 {
            MemRead::Replace(self.wram_select as u8)
        } else {
            MemRead::PassThrough
        }
    }

    fn on_write(&mut self, _mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        if addr >= 0xc000 && addr <= 0xcfff {
            let off = addr as usize - 0xc000;
            self.wram_bank[0][off] = value;
        } else if addr >= 0xd000 && addr <= 0xdfff {
            let off = addr as usize - 0xd000;
            self.wram_bank[self.wram_select][off] = value;
        } else if addr == 0xff4d {
            self.speed_switch = value & 0x01 != 0;
        } else if addr == 0xff56 {
            warn!("Infrared read");
        } else if addr == 0xff70 {
            self.wram_select = (value as usize & 0xf).max(1);
        }

        MemWrite::PassThrough
    }
}
