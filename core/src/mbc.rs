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

struct MbcNone {
    rom: Vec<u8>,
}

impl MbcNone {
    fn new(rom: Vec<u8>) -> Self {
        Self { rom }
    }

    fn on_read(&mut self, mmu: &Mmu, addr: u16) -> MemRead {
        if addr <= 0x7fff {
            MemRead::Replace(self.rom[addr as usize])
        } else {
            MemRead::PassThrough
        }
    }

    fn on_write(&mut self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        if addr <= 0x7fff {
            MemWrite::Block
        } else if addr >= 0xa000 && addr <= 0xbfff {
            MemWrite::PassThrough
        } else {
            unreachable!("Write to ROM: {:02x} {:02x}", addr, value);
        }
    }
}

struct Mbc1 {
    rom: Vec<u8>,
    rom_bank: usize,
    ram_bank: usize,
    ram_enable: bool,
    ram_select: bool,
}

impl Mbc1 {
    fn new(rom: Vec<u8>) -> Self {
        Self {
            rom,
            rom_bank: 0,
            ram_bank: 0,
            ram_enable: false,
            ram_select: false,
        }
    }

    fn on_read(&mut self, mmu: &Mmu, addr: u16) -> MemRead {
        if addr <= 0x3fff {
            MemRead::Replace(self.rom[addr as usize])
        } else if addr >= 0x4000 && addr <= 0x7fff {
            let base = self.rom_bank.max(1) * 0x4000;
            let offset = addr as usize - 0x4000;
            MemRead::Replace(self.rom[base + offset])
        } else if addr >= 0xa000 && addr <= 0xbfff {
            if self.ram_enable {
                MemRead::PassThrough
            } else {
                warn!("Read from disabled external RAM: {:04x}", addr);
                MemRead::Replace(0)
            }
        } else {
            MemRead::PassThrough
        }
    }

    fn on_write(&mut self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        if addr <= 0x1fff {
            if value == 0x00 {
                info!("External RAM disabled");
                self.ram_enable = false;
            } else if value == 0x0a {
                info!("External RAM enabled");
                self.ram_enable = true;
            }
            MemWrite::Block
        } else if addr >= 0x2000 && addr <= 0x3fff {
            self.rom_bank = (self.rom_bank & !0x1f) | (value as usize & 0x1f);
            debug!("Switch ROM bank to {}", self.rom_bank);
            MemWrite::Block
        } else if addr >= 0x4000 && addr <= 0x5fff {
            if self.ram_select {
                self.ram_bank = value as usize & 0x3;
            } else {
                self.rom_bank = (self.rom_bank & !0xc0) | ((value as usize & 0x3) << 5);
            }
            MemWrite::Block
        } else if addr >= 0xa000 && addr <= 0xbfff {
            if self.ram_enable {
                MemWrite::PassThrough
            } else {
                warn!("Write to disabled external RAM: {:04x} {:02x}", addr, value);
                MemWrite::Block
            }
        } else {
            unimplemented!("write to rom {:04x} {:02x}", addr, value)
        }
    }
}

struct Mbc2 {
    rom: Vec<u8>,
}

impl Mbc2 {
    fn new(rom: Vec<u8>) -> Self {
        Self { rom }
    }

    fn on_read(&mut self, mmu: &Mmu, addr: u16) -> MemRead {
        unimplemented!()
    }

    fn on_write(&mut self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        unimplemented!()
    }
}

struct Mbc3 {
    rom: Vec<u8>,
}

impl Mbc3 {
    fn new(rom: Vec<u8>) -> Self {
        Self { rom }
    }

    fn on_read(&mut self, mmu: &Mmu, addr: u16) -> MemRead {
        unimplemented!()
    }

    fn on_write(&mut self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        unimplemented!()
    }
}

struct Mbc5 {
    rom: Vec<u8>,
}

impl Mbc5 {
    fn new(rom: Vec<u8>) -> Self {
        Self { rom }
    }

    fn on_read(&mut self, mmu: &Mmu, addr: u16) -> MemRead {
        unimplemented!()
    }

    fn on_write(&mut self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        unimplemented!()
    }
}

struct HuC1 {
    rom: Vec<u8>,
}

impl HuC1 {
    fn new(rom: Vec<u8>) -> Self {
        Self { rom }
    }

    fn on_read(&mut self, mmu: &Mmu, addr: u16) -> MemRead {
        unimplemented!()
    }

    fn on_write(&mut self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        unimplemented!()
    }
}

enum MbcType {
    None(MbcNone),
    Mbc1(Mbc1),
    Mbc2(Mbc2),
    Mbc3(Mbc3),
    Mbc5(Mbc5),
    HuC1(HuC1),
}

impl MbcType {
    fn new(code: u8, rom: Vec<u8>) -> Self {
        match code {
            0x00 => MbcType::None(MbcNone::new(rom)),
            0x01 | 0x02 | 0x03 => MbcType::Mbc1(Mbc1::new(rom)),
            0x05 | 0x06 => MbcType::Mbc2(Mbc2::new(rom)),
            0x08 | 0x09 => unimplemented!("ROM+RAM: {:02x}", code),
            0x0b | 0x0c | 0x0d => unimplemented!("MMM01: {:02x}", code),
            0x0f | 0x10 | 0x11 | 0x12 | 0x13 => MbcType::Mbc3(Mbc3::new(rom)),
            0x15 | 0x16 | 0x17 => unimplemented!("Mbc4: {:02x}", code),
            0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e => MbcType::Mbc5(Mbc5::new(rom)),
            0xfc => unimplemented!("POCKET CAMERA"),
            0xfd => unimplemented!("BANDAI TAMAS"),
            0xfe => unimplemented!("HuC3"),
            0xff => MbcType::HuC1(HuC1::new(rom)),
            _ => unreachable!("Invalid cartridge type: {:02x}", code),
        }
    }

    fn on_read(&mut self, mmu: &Mmu, addr: u16) -> MemRead {
        match self {
            MbcType::None(c) => c.on_read(mmu, addr),
            MbcType::Mbc1(c) => c.on_read(mmu, addr),
            MbcType::Mbc2(c) => c.on_read(mmu, addr),
            MbcType::Mbc3(c) => c.on_read(mmu, addr),
            MbcType::Mbc5(c) => c.on_read(mmu, addr),
            MbcType::HuC1(c) => c.on_read(mmu, addr),
        }
    }

    fn on_write(&mut self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        match self {
            MbcType::None(c) => c.on_write(mmu, addr, value),
            MbcType::Mbc1(c) => c.on_write(mmu, addr, value),
            MbcType::Mbc2(c) => c.on_write(mmu, addr, value),
            MbcType::Mbc3(c) => c.on_write(mmu, addr, value),
            MbcType::Mbc5(c) => c.on_write(mmu, addr, value),
            MbcType::HuC1(c) => c.on_write(mmu, addr, value),
        }
    }
}

impl std::fmt::Display for MbcType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            MbcType::None(_) => "None",
            MbcType::Mbc1(_) => "Mbc1",
            MbcType::Mbc2(_) => "Mbc2",
            MbcType::Mbc3(_) => "Mbc3",
            MbcType::Mbc5(_) => "Mbc5",
            MbcType::HuC1(_) => "HuC1",
        };

        write!(f, "{}", name)
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

struct Cartridge {
    title: String,
    cgb: bool,
    cgb_only: bool,
    license_new: String,
    license_old: u8,
    sgb: bool,
    mbc: MbcType,
    rom_size: u8,
    ram_size: u8,
    dstcode: u8,
    rom_version: u8,
    rom_checksum: u16,
}

fn verify(rom: &[u8], checksum: u16) {
    let mut sum = 0u16;

    for (i, b) in rom.iter().enumerate() {
        if i == 0x14e || i == 0x14f {
            continue;
        }
        sum = sum.wrapping_add(*b as u16);
    }

    if sum == checksum {
        info!("ROM checksum verified: {:04x}", checksum);
    } else {
        warn!(
            "ROM checksum mismatch: expect: {:04x}, actual: {:04x}",
            checksum, sum
        );
    }
}

impl Cartridge {
    fn new(rom: Vec<u8>) -> Self {
        let checksum = (rom[0x14e] as u16) << 8 | (rom[0x14f] as u16);

        verify(&rom, checksum);

        Self {
            title: parse_str(&rom[0x134..0x144]),
            cgb: rom[0x143] & 0x80 != 0,
            cgb_only: rom[0x143] == 0xc0,
            license_new: parse_str(&rom[0x144..0x146]),
            license_old: rom[0x14b],
            sgb: rom[0x146] == 0x03,
            mbc: MbcType::new(rom[0x147], rom.clone()),
            rom_size: rom[0x148],
            ram_size: rom[0x149],
            dstcode: rom[0x14a],
            rom_version: rom[0x14c],
            rom_checksum: checksum,
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

        info!("Mbc: {}", self.mbc);
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
            0x05 => "1MByte (64 banks)  - only 63 banks used by Mbc1",
            0x06 => "2MByte (128 banks) - only 125 banks used by Mbc1",
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

    fn on_read(&mut self, mmu: &Mmu, addr: u16) -> MemRead {
        self.mbc.on_read(mmu, addr)
    }

    fn on_write(&mut self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        self.mbc.on_write(mmu, addr, value)
    }
}

struct Inner {
    cartridge: Cartridge,
    use_boot_rom: bool,
}

impl Inner {
    fn new(rom: Vec<u8>) -> Self {
        let cartridge = Cartridge::new(rom);

        cartridge.show_info();

        Self {
            cartridge,
            use_boot_rom: true,
        }
    }

    fn on_read(&mut self, mmu: &Mmu, addr: u16) -> MemRead {
        if self.use_boot_rom && addr < 0x100 {
            MemRead::Replace(BOOT_ROM[addr as usize])
        } else {
            self.cartridge.on_read(mmu, addr)
        }
    }

    fn on_write(&mut self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        if self.use_boot_rom && addr < 0x100 {
            unreachable!("Writing to boot ROM")
        } else if addr == 0xff50 {
            info!("Disable boot ROM");
            self.use_boot_rom = false;
            MemWrite::Block
        } else {
            self.cartridge.on_write(mmu, addr, value)
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
