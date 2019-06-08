use crate::device::{HardwareHandle, Key};
use crate::mmu::{MemHandler, MemRead, MemWrite, Mmu};
use log::*;
use std::cell::RefCell;
use std::rc::Rc;

const BOOT_ROM: &[u8] = include_bytes!("boot.bin");

pub struct Mbc {
    inner: Rc<RefCell<Inner>>,
}

impl Mbc {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(Inner::new(rom))),
        }
    }

    pub fn handler(&self) -> MbcMemHandler {
        MbcMemHandler::new(self.inner.clone())
    }
}

struct Inner {
    rom: Vec<u8>,
    use_boot_rom: bool,
    rom_bank: usize,
}

impl Inner {
    fn new(rom: Vec<u8>) -> Self {
        Self {
            rom,
            use_boot_rom: true,
            rom_bank: 0,
        }
    }

    fn on_read(&mut self, mmu: &Mmu, addr: u16) -> MemRead {
        if self.use_boot_rom && addr < 0x100 {
            MemRead::Replace(BOOT_ROM[addr as usize])
        } else if addr >= 0x0000 && addr <= 0x3fff {
            MemRead::Replace(self.rom[addr as usize])
        } else if addr >= 0x4000 && addr <= 0x7fff {
            let base = self.rom_bank * 0x4000;
            let offset = addr as usize - 0x4000;
            MemRead::Replace(self.rom[base + offset])
        } else {
            MemRead::PassThrough
        }
    }

    fn on_write(&mut self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        if self.use_boot_rom && addr < 0x100 {
            unreachable!("Writing to BOOT rom")
        } else if addr == 0xff50 {
            info!("Disable boot rom");
            self.use_boot_rom = false;
            MemWrite::PassThrough
        } else if addr >= 0x2000 && addr <= 0x3fff {
            self.rom_bank = (value & 0x1f) as usize;
            if self.rom_bank == 0 {
                self.rom_bank = 1;
            }
            info!("Switch ROM bank to {}", self.rom_bank);
            MemWrite::Block
        } else {
            unimplemented!("write to rom {:02x} {:02x}", addr, value)
        }
    }
}

pub struct MbcMemHandler {
    inner: Rc<RefCell<Inner>>,
}

impl MbcMemHandler {
    fn new(inner: Rc<RefCell<Inner>>) -> Self {
        Self { inner }
    }
}

impl MemHandler for MbcMemHandler {
    fn on_read(&self, mmu: &Mmu, addr: u16) -> MemRead {
        self.inner.borrow_mut().on_read(mmu, addr)
    }

    fn on_write(&self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        self.inner.borrow_mut().on_write(mmu, addr, value)
    }
}
