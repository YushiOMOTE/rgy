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

#[derive(Debug)]
enum Type {
    None,
    MBC1,
    MBC2,
    MBC3,
    MBC5,
    HuC1,
}

impl From<u8> for Type {
    fn from(code: u8) -> Self {
        match code {
            0x00 => Type::None,
            0x01 | 0x02 | 0x03 => Type::MBC1,
            0x05 | 0x06 => Type::MBC2,
            0x08 | 0x09 => unimplemented!("ROM+RAM: {:02x}", code),
            0x0b | 0x0c | 0x0d => unimplemented!("MMM01: {:02x}", code),
            0x0f | 0x10 | 0x11 | 0x12 | 0x13 => Type::MBC3,
            0x15 | 0x16 | 0x17 => unimplemented!("MBC4: {:02x}", code),
            0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e => Type::MBC5,
            0xfc => unimplemented!("POCKET CAMERA"),
            0xfd => unimplemented!("BANDAI TAMAS"),
            0xfe => unimplemented!("HuC3"),
            0xff => Type::HuC1,
            _ => unreachable!("Invalid cartridge type: {:02x}", code),
        }
    }
}

fn parse_str(b: &[u8]) -> String {
    let b: Vec<u8> = b
        .iter()
        .take_while(|b| *b & 0x80 == 0)
        .map(|b| if *b == 0x00 { b' ' } else { *b })
        .collect();
    String::from_utf8_lossy(&b).to_string()
}

struct Header {
    title: String,
    cgb: bool,
    cgb_only: bool,
    license_new: String,
    license_old: u8,
    sgb: bool,
    mbc_type: Type,
    rom_size: u8,
    ram_size: u8,
    dstcode: u8,
    rom_version: u8,
    rom_checksum: u16,
}

impl Header {
    fn new(rom: &[u8]) -> Self {
        Self {
            title: parse_str(&rom[0x134..0x144]),
            cgb: rom[0x143] & 0x80 != 0,
            cgb_only: rom[0x143] == 0xc0,
            license_new: parse_str(&rom[0x144..0x146]),
            license_old: rom[0x14b],
            sgb: rom[0x146] == 0x03,
            mbc_type: rom[0x147].into(),
            rom_size: rom[0x148],
            ram_size: rom[0x149],
            dstcode: rom[0x14a],
            rom_version: rom[0x14c],
            rom_checksum: (rom[0x14e] as u16) << 8 | (rom[0x14f] as u16),
        }
    }

    fn verify(&self, rom: &[u8]) {
        let mut sum = 0u16;

        for (i, b) in rom.iter().enumerate() {
            if i == 0x14e || i == 0x14f {
                continue;
            }
            sum = sum.wrapping_add(*b as u16);
        }

        if sum == self.rom_checksum {
            info!("ROM checksum verified: {:04x}", self.rom_checksum);
        } else {
            warn!(
                "ROM checksum mismatch: expect: {:02x}, actual: {:02x}",
                self.rom_checksum, sum
            );
        }
    }

    fn show_info(&self) {
        info!("Title: {}", self.title);
        info!(
            "License: {} ({:02x}), Version: {}",
            self.license_new, self.license_old, self.rom_version,
        );
        let dstcode = match self.dstcode {
            0x00 => "Japanese",
            0x01 => "Non-Japanese",
            _ => "Unknown",
        };
        info!("Destination: {}", dstcode);

        info!("MBC: {:?}", self.mbc_type);
        info!(
            "Color: {} (Compat: {}), Super: {}",
            self.cgb, !self.cgb_only, self.sgb,
        );

        let rom_size = match self.rom_size {
            0x00 => "32KByte (no ROM banking)",
            0x01 => "64KByte (4 banks)",
            0x02 => "128KByte (8 banks)",
            0x03 => "256KByte (16 banks)",
            0x04 => "512KByte (32 banks)",
            0x05 => "1MByte (64 banks)  - only 63 banks used by MBC1",
            0x06 => "2MByte (128 banks) - only 125 banks used by MBC1",
            0x07 => "4MByte (256 banks)",
            0x52 => "1.1MByte (72 banks)",
            0x53 => "1.2MByte (80 banks)",
            0x54 => "1.5MByte (96 banks)",
            _ => "Unknown",
        };
        let ram_size = match self.ram_size {
            0x00 => "None",
            0x01 => "2 KBytes",
            0x02 => "8 Kbytes",
            0x03 => "32 KBytes (4 banks of 8KBytes each)",
            _ => "Unknown",
        };
        info!("ROM size: {}", rom_size);
        info!("RAM size: {}", ram_size);
    }
}

struct Inner {
    hdr: Header,
    rom: Vec<u8>,
    use_boot_rom: bool,
    rom_bank: usize,
}

impl Inner {
    fn new(rom: Vec<u8>) -> Self {
        let hdr = Header::new(&rom);

        hdr.show_info();
        hdr.verify(&rom);

        Self {
            hdr,
            rom,
            use_boot_rom: true,
            rom_bank: 0,
        }
    }

    fn on_read(&mut self, mmu: &Mmu, addr: u16) -> MemRead {
        if self.use_boot_rom && addr < 0x100 {
            MemRead::Replace(BOOT_ROM[addr as usize])
        } else if addr <= 0x3fff {
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
