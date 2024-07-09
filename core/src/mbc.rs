use crate::hardware::HardwareHandle;
use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use log::*;

const BOOT_ROM: &[u8] = include_bytes!("dmg.bin");
const BOOT_ROM_COLOR: &[u8] = include_bytes!("cgb.bin");

struct MbcNone {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl MbcNone {
    fn new(rom: Vec<u8>) -> Self {
        Self {
            rom,
            ram: vec![0; 0x2000],
        }
    }

    fn on_read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7fff => self.rom[addr as usize],
            0xa000..=0xbfff => self.ram[addr as usize - 0xa000],
            _ => unreachable!("read attempt to mbc0 addr={:04x}", addr),
        }
    }

    fn on_write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7fff => self.rom[addr as usize] = value,
            0xa000..=0xbfff => self.ram[addr as usize - 0xa000] = value,
            _ => unreachable!("write attempt to mbc0 addr={:04x}, v={:02x}", addr, value),
        }
    }
}

struct Mbc1 {
    hw: HardwareHandle,
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: usize,
    ram_bank: usize,
    ram_enable: bool,
    ram_select: bool,
}

impl Mbc1 {
    fn new(hw: HardwareHandle, rom: Vec<u8>) -> Self {
        let ram = hw.get().borrow_mut().load_ram(0x8000);

        Self {
            hw,
            rom,
            ram,
            rom_bank: 0,
            ram_bank: 0,
            ram_enable: false,
            ram_select: false,
        }
    }

    fn on_read(&self, addr: u16) -> u8 {
        if addr <= 0x3fff {
            self.rom[addr as usize]
        } else if (0x4000..=0x7fff).contains(&addr) {
            let rom_bank = self.rom_bank.max(1);

            // ROM bank 0x20, 0x40, 0x60 are somehow not available
            let rom_bank = if rom_bank == 0x20 || rom_bank == 0x40 || rom_bank == 0x60 {
                warn!("Odd ROM bank selection: {:02x}", rom_bank);
                rom_bank + 1
            } else {
                rom_bank
            };

            let base = rom_bank * 0x4000;
            let offset = addr as usize - 0x4000;
            let addr = (base + offset) & (self.rom.len() - 1);
            self.rom[addr]
        } else if (0xa000..=0xbfff).contains(&addr) {
            if self.ram_enable {
                let base = self.ram_bank * 0x2000;
                let offset = addr as usize - 0xa000;
                let addr = (base + offset) & (self.rom.len() - 1);
                self.ram[addr]
            } else {
                warn!("Read from disabled external RAM: {:04x}", addr);
                0
            }
        } else {
            unimplemented!()
        }
    }

    fn on_write(&mut self, addr: u16, value: u8) {
        if addr <= 0x1fff {
            if value & 0xf == 0x0a {
                info!("External RAM enabled");
                self.ram_enable = true;
            } else {
                info!("External RAM disabled");
                self.ram_enable = false;
                self.hw.get().borrow_mut().save_ram(&self.ram);
            }
        } else if (0x2000..=0x3fff).contains(&addr) {
            self.rom_bank = (self.rom_bank & !0x1f) | (value as usize & 0x1f);
            debug!("Switch ROM bank to {:02x}", self.rom_bank);
        } else if (0x4000..=0x5fff).contains(&addr) {
            if self.ram_select {
                self.ram_bank = value as usize & 0x3;
            } else {
                self.rom_bank = (self.rom_bank & !0x60) | ((value as usize & 0x3) << 5);
            }
        } else if (0x6000..=0x7fff).contains(&addr) {
            if value == 0x00 {
                self.ram_select = false;
            } else if value == 0x01 {
                self.ram_select = true;
            } else {
                unimplemented!("Invalid ROM/RAM select mode");
            }
        } else if (0xa000..=0xbfff).contains(&addr) {
            if self.ram_enable {
                let base = self.ram_bank * 0x2000;
                let offset = addr as usize - 0xa000;
                self.ram[base + offset] = value;
            } else {
                warn!("Write to disabled external RAM: {:04x} {:02x}", addr, value);
            }
        } else {
            unimplemented!("write to rom {:04x} {:02x}", addr, value)
        }
    }
}

struct Mbc2 {
    hw: HardwareHandle,
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: usize,
    ram_enable: bool,
}

impl Mbc2 {
    fn new(hw: HardwareHandle, rom: Vec<u8>) -> Self {
        let ram = hw.get().borrow_mut().load_ram(0x200);

        Self {
            hw,
            rom,
            ram,
            rom_bank: 1,
            ram_enable: false,
        }
    }

    fn on_read(&self, addr: u16) -> u8 {
        if addr <= 0x3fff {
            self.rom[addr as usize]
        } else if (0x4000..=0x7fff).contains(&addr) {
            let base = self.rom_bank.max(1) * 0x4000;
            let offset = addr as usize - 0x4000;
            self.rom[base + offset]
        } else if (0xa000..=0xa1ff).contains(&addr) {
            if self.ram_enable {
                self.ram[addr as usize - 0xa000] & 0xf
            } else {
                warn!("Read from disabled cart RAM: {:04x}", addr);
                0
            }
        } else {
            unimplemented!()
        }
    }

    fn on_write(&mut self, addr: u16, value: u8) {
        if addr <= 0x1fff {
            if addr & 0x100 == 0 {
                self.ram_enable = (value & 0x0f) == 0x0a;
                info!(
                    "Cart RAM {} {:02x}",
                    if self.ram_enable {
                        "enabled"
                    } else {
                        "disabled"
                    },
                    value
                );
                if !self.ram_enable {
                    self.hw.get().borrow_mut().save_ram(&self.ram);
                }
            }
        } else if (0x2000..=0x3fff).contains(&addr) {
            if addr & 0x100 != 0 {
                self.rom_bank = (value as usize & 0xf).max(1);
                debug!("Switch ROM bank to {:02x}", self.rom_bank);
            }
        } else if (0x4000..=0x7fff).contains(&addr) {
            warn!("Writing to read-only range: {:04x} {:02x}", addr, value);
        } else if (0xa000..=0xa1ff).contains(&addr) {
            if self.ram_enable {
                self.ram[addr as usize - 0xa000] = value & 0xf;
            } else {
                warn!("Write to disabled cart RAM: {:04x} {:02x}", addr, value);
            }
        } else {
            warn!("write to rom {:04x} {:02x}", addr, value);
        }
    }
}

struct Mbc3 {
    hw: HardwareHandle,
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: usize,
    enable: bool,
    select: u8,
    rtc_secs: u8,
    rtc_mins: u8,
    rtc_hours: u8,
    rtc_day_low: u8,
    rtc_day_high: u8,
    epoch: u64,
    prelatch: bool,
}

impl Drop for Mbc3 {
    fn drop(&mut self) {
        self.save();
    }
}

impl Mbc3 {
    fn new(hw: HardwareHandle, rom: Vec<u8>) -> Self {
        let ram = hw.get().borrow_mut().load_ram(0x8000);

        let mut s = Self {
            hw,
            rom,
            ram,
            rom_bank: 0,
            enable: false,
            select: 0,
            rtc_secs: 0,
            rtc_mins: 0,
            rtc_hours: 0,
            rtc_day_low: 0,
            rtc_day_high: 0,
            epoch: 0,
            prelatch: false,
        };
        s.update_epoch();
        s
    }

    fn save(&mut self) {
        self.hw.get().borrow_mut().save_ram(&self.ram);
    }

    fn epoch(&self) -> u64 {
        self.hw.get().borrow_mut().clock() / 1_000_000
    }

    fn on_read(&self, addr: u16) -> u8 {
        if addr <= 0x3fff {
            self.rom[addr as usize]
        } else if (0x4000..=0x7fff).contains(&addr) {
            let rom_bank = self.rom_bank.max(1);
            let base = rom_bank * 0x4000;
            let offset = addr as usize - 0x4000;
            self.rom[base + offset]
        } else if (0xa000..=0xbfff).contains(&addr) {
            match self.select {
                x if x == 0x00 || x == 0x01 || x == 0x02 || x == 0x03 => {
                    let base = x as usize * 0x2000;
                    let offset = addr as usize - 0xa000;
                    self.ram[base + offset]
                }
                0x08 => self.rtc_secs,
                0x09 => self.rtc_mins,
                0x0a => self.rtc_hours,
                0x0b => self.rtc_day_low,
                0x0c => self.rtc_day_high,
                s => unimplemented!("Unknown selector: {:02x}", s),
            }
        } else {
            unreachable!("Invalid read from ROM: {:02x}", addr);
        }
    }

    fn on_write(&mut self, addr: u16, value: u8) {
        if addr <= 0x1fff {
            if value == 0x00 {
                info!("External RAM/RTC disabled");
                self.enable = false;
            } else if value == 0x0a {
                info!("External RAM/RTC enabled");
                self.enable = true;
            }
        } else if (0x2000..=0x3fff).contains(&addr) {
            self.rom_bank = value as usize & 0x7f;
            trace!("Switch ROM bank to {}", self.rom_bank);
        } else if (0x4000..=0x5fff).contains(&addr) {
            self.select = value;
            self.save();
            debug!("Select RAM bank/RTC: {:02x}", self.select);
        } else if (0x6000..=0x7fff).contains(&addr) {
            if self.prelatch {
                if value == 0x01 {
                    self.latch();
                }
                self.prelatch = false;
            } else if value == 0x00 {
                self.prelatch = true;
            }
        } else if (0xa000..=0xbfff).contains(&addr) {
            match self.select {
                x if x == 0x00 || x == 0x01 || x == 0x02 || x == 0x03 => {
                    let base = x as usize * 0x2000;
                    let offset = addr as usize - 0xa000;
                    self.ram[base + offset] = value;
                }
                0x08 => {
                    self.rtc_secs = value;
                    self.update_epoch();
                }
                0x09 => {
                    self.rtc_mins = value;
                    self.update_epoch();
                }
                0x0a => {
                    self.rtc_hours = value;
                    self.update_epoch();
                }
                0x0b => {
                    self.rtc_day_low = value;
                    self.update_epoch();
                }
                0x0c => {
                    self.rtc_day_high = value;
                    self.update_epoch();
                }
                s => unimplemented!("Unknown selector: {:02x}", s),
            }
        } else {
            unimplemented!("write to rom {:04x} {:02x}", addr, value)
        }
    }

    fn update_epoch(&mut self) {
        self.epoch = self.epoch();
    }

    fn day(&self) -> u64 {
        ((self.rtc_day_high as u64 & 1) << 8) & self.rtc_day_low as u64
    }

    fn dhms_to_secs(&self) -> u64 {
        let d = self.day();
        let s = self.rtc_secs as u64;
        let m = self.rtc_mins as u64;
        let h = self.rtc_hours as u64;
        (d * 24 + h) * 3600 + m * 60 + s
    }

    fn secs_to_dhms(&mut self, secs: u64) {
        let s = secs % 60;
        let m = (secs / 60) % 60;
        let h = (secs / 3600) % 24;
        let d = secs / (3600 * 24);
        self.rtc_secs = s as u8;
        self.rtc_mins = m as u8;
        self.rtc_hours = h as u8;
        self.rtc_day_low = d as u8;
        self.rtc_day_high = (self.rtc_day_high & !1) | ((d >> 8) & 1) as u8;
    }

    fn latch(&mut self) {
        let new_epoch = if self.rtc_day_high & 0x40 == 0 {
            self.epoch()
        } else {
            // Halt
            self.epoch
        };
        let elapsed = new_epoch - self.epoch;

        let last_day = self.day();
        let last_secs = self.dhms_to_secs();
        self.secs_to_dhms(last_secs + elapsed);
        let new_day = self.day();

        // Overflow
        if new_day < last_day {
            self.rtc_day_high |= 0x80;
        }

        debug!(
            "Latching RTC: {:04}/{:02}:{:02}:{:02}",
            self.day(),
            self.rtc_hours,
            self.rtc_mins,
            self.rtc_secs
        );

        self.epoch = new_epoch;
    }
}

struct Mbc5 {
    hw: HardwareHandle,
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: usize,
    ram_bank: usize,
    ram_enable: bool,
}

impl Mbc5 {
    fn new(hw: HardwareHandle, rom: Vec<u8>) -> Self {
        let ram = hw.get().borrow_mut().load_ram(0x20000);

        Self {
            hw,
            rom,
            ram,
            rom_bank: 0,
            ram_bank: 0,
            ram_enable: false,
        }
    }

    fn on_read(&self, addr: u16) -> u8 {
        if addr <= 0x3fff {
            self.rom[addr as usize]
        } else if (0x4000..=0x7fff).contains(&addr) {
            let base = self.rom_bank * 0x4000;
            let offset = addr as usize - 0x4000;
            self.rom[base + offset]
        } else if (0xa000..=0xbfff).contains(&addr) {
            if self.ram_enable {
                let base = self.ram_bank * 0x2000;
                let offset = addr as usize - 0xa000;
                self.ram[base + offset]
            } else {
                warn!("Read from disabled external RAM: {:04x}", addr);
                0
            }
        } else {
            unimplemented!()
        }
    }

    fn on_write(&mut self, addr: u16, value: u8) {
        if addr <= 0x1fff {
            if value & 0xf == 0x0a {
                info!("External RAM enabled");
                self.ram_enable = true;
            } else {
                info!("External RAM disabled");
                self.ram_enable = false;
                self.hw.get().borrow_mut().save_ram(&self.ram);
            }
        } else if (0x2000..=0x2fff).contains(&addr) {
            self.rom_bank = (self.rom_bank & !0xff) | value as usize;
            debug!("Switch ROM bank to {:02x}", self.rom_bank);
        } else if (0x3000..=0x3fff).contains(&addr) {
            self.rom_bank = (self.rom_bank & !0x100) | (value as usize & 1) << 8;
            debug!("Switch ROM bank to {:02x}", self.rom_bank);
        } else if (0x4000..=0x5fff).contains(&addr) {
            self.ram_bank = value as usize & 0xf;
        } else if (0xa000..=0xbfff).contains(&addr) {
            if self.ram_enable {
                let base = self.ram_bank * 0x2000;
                let offset = addr as usize - 0xa000;
                self.ram[base + offset] = value;
            } else {
                warn!("Write to disabled external RAM: {:04x} {:02x}", addr, value);
            }
        } else {
            unimplemented!("write to rom {:04x} {:02x}", addr, value)
        }
    }
}

#[allow(unused)]
struct HuC1 {
    rom: Vec<u8>,
}

impl HuC1 {
    fn new(rom: Vec<u8>) -> Self {
        Self { rom }
    }

    fn on_read(&self, _addr: u16) -> u8 {
        unimplemented!()
    }

    fn on_write(&mut self, _addr: u16, _value: u8) {
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
    fn new(hw: HardwareHandle, code: u8, rom: Vec<u8>) -> Self {
        match code {
            0x00 => MbcType::None(MbcNone::new(rom)),
            0x01..=0x03 => MbcType::Mbc1(Mbc1::new(hw, rom)),
            0x05 | 0x06 => MbcType::Mbc2(Mbc2::new(hw, rom)),
            0x08 | 0x09 => unimplemented!("ROM+RAM: {:02x}", code),
            0x0b..=0x0d => unimplemented!("MMM01: {:02x}", code),
            0x0f..=0x13 => MbcType::Mbc3(Mbc3::new(hw, rom)),
            0x15..=0x17 => unimplemented!("Mbc4: {:02x}", code),
            0x19..=0x1e => MbcType::Mbc5(Mbc5::new(hw, rom)),
            0xfc => unimplemented!("POCKET CAMERA"),
            0xfd => unimplemented!("BANDAI TAMAS"),
            0xfe => unimplemented!("HuC3"),
            0xff => MbcType::HuC1(HuC1::new(rom)),
            _ => unreachable!("Invalid cartridge type: {:02x}", code),
        }
    }

    fn on_read(&self, addr: u16) -> u8 {
        match self {
            MbcType::None(c) => c.on_read(addr),
            MbcType::Mbc1(c) => c.on_read(addr),
            MbcType::Mbc2(c) => c.on_read(addr),
            MbcType::Mbc3(c) => c.on_read(addr),
            MbcType::Mbc5(c) => c.on_read(addr),
            MbcType::HuC1(c) => c.on_read(addr),
        }
    }

    fn on_write(&mut self, addr: u16, value: u8) {
        match self {
            MbcType::None(c) => c.on_write(addr, value),
            MbcType::Mbc1(c) => c.on_write(addr, value),
            MbcType::Mbc2(c) => c.on_write(addr, value),
            MbcType::Mbc3(c) => c.on_write(addr, value),
            MbcType::Mbc5(c) => c.on_write(addr, value),
            MbcType::HuC1(c) => c.on_write(addr, value),
        }
    }
}

impl alloc::fmt::Display for MbcType {
    fn fmt(&self, f: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
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
    fn new(hw: HardwareHandle, rom: Vec<u8>) -> Self {
        let checksum = (rom[0x14e] as u16) << 8 | (rom[0x14f] as u16);

        verify(&rom, checksum);

        Self {
            title: parse_str(&rom[0x134..0x144]),
            cgb: rom[0x143] & 0x80 != 0,
            cgb_only: rom[0x143] == 0xc0,
            license_new: parse_str(&rom[0x144..0x146]),
            license_old: rom[0x14b],
            sgb: rom[0x146] == 0x03,
            mbc: MbcType::new(hw, rom[0x147], rom.clone()),
            rom_size: rom[0x148],
            ram_size: rom[0x149],
            dstcode: rom[0x14a],
            rom_version: rom[0x14c],
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

    fn on_read(&self, addr: u16) -> u8 {
        self.mbc.on_read(addr)
    }

    fn on_write(&mut self, addr: u16, value: u8) {
        self.mbc.on_write(addr, value)
    }
}

pub struct Mbc {
    color: bool,
    cartridge: Cartridge,
    use_boot_rom: bool,
}

impl Mbc {
    pub fn new(hw: HardwareHandle, rom: Vec<u8>, color: bool) -> Self {
        let cartridge = Cartridge::new(hw, rom);

        cartridge.show_info();

        Self {
            color,
            cartridge,
            use_boot_rom: true,
        }
    }

    fn in_boot_rom(&self, addr: u16) -> bool {
        if self.color {
            assert_eq!(0x900, BOOT_ROM_COLOR.len());

            addr < 0x100 || (0x200..0x900).contains(&addr)
        } else {
            assert_eq!(0x100, BOOT_ROM.len());

            addr < 0x100
        }
    }

    pub(crate) fn on_read(&self, addr: u16) -> u8 {
        if self.use_boot_rom && self.in_boot_rom(addr) {
            BOOT_ROM[addr as usize]
        } else {
            self.cartridge.on_read(addr)
        }
    }

    pub(crate) fn disable_boot_rom(&mut self, _v: u8) {
        info!("Disable boot ROM");
        self.use_boot_rom = false;
    }

    pub(crate) fn on_write(&mut self, addr: u16, value: u8) {
        if self.use_boot_rom && addr < 0x100 {
            unreachable!("Writing to boot ROM")
        } else if addr == 0xff50 {
            self.use_boot_rom = false;
        } else {
            self.cartridge.on_write(addr, value)
        }
    }
}
