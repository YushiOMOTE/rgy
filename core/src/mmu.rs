use crate::gpu::Gpu;
use crate::hardware::HardwareHandle;
use crate::ic::{Ic, Irq};
use crate::joypad::Joypad;
use crate::mbc::Mbc;
use crate::serial::Serial;
use crate::timer::Timer;
use alloc::{vec, vec::Vec};
use core::cell::RefCell;

/// Handles work ram access between 0xc000 - 0xdfff
pub struct Wram {
    n: usize,
    bank: Vec<Vec<u8>>,
}

impl Wram {
    fn new() -> Self {
        Self {
            n: 1,
            bank: vec![vec![0; 0x1000]; 8],
        }
    }

    fn switch_bank(&mut self, n: u8) {
        self.n = n as usize;
    }

    fn get8(&self, addr: u16) -> u8 {
        match addr {
            0xc000..=0xcfff => self.bank[0][addr as usize - 0xc000],
            0xd000..=0xdfff => self.bank[self.n][addr as usize - 0xd000],
            0xe000..=0xfdff => self.get8(addr - 0xe000 + 0xc000),
            _ => unreachable!("read attemp to wram addr={:04x}", addr),
        }
    }

    fn set8(&mut self, addr: u16, v: u8) {
        match addr {
            0xc000..=0xcfff => self.bank[0][addr as usize - 0xc000] = v,
            0xd000..=0xdfff => self.bank[self.n][addr as usize - 0xd000] = v,
            0xe000..=0xfdff => self.set8(addr - 0xe000 + 0xc000, v),
            _ => unreachable!("write attemp to wram addr={:04x} v={:02x}", addr, v),
        }
    }
}

/// Handles high ram access between 0xff80 - 0xfffe
pub struct Hram {
    bank: Vec<u8>,
}

impl Hram {
    fn new() -> Self {
        Self {
            bank: vec![0; 0x7f],
        }
    }

    fn get8(&self, addr: u16) -> u8 {
        self.bank[addr as usize - 0xff80]
    }

    fn set8(&mut self, addr: u16, v: u8) {
        self.bank[addr as usize - 0xff80] = v;
    }
}

/// The memory management unit (MMU)
///
/// This unit holds a memory byte array which represents address space of the memory.
/// It provides the logic to intercept access from the CPU to the memory byte array,
/// and to modify the memory access behaviour.
pub struct Mmu {
    wram: Wram,
    hram: Hram,
    gpu: Gpu,
    mbc: Mbc,
    timer: Timer,
    ic: Ic,
    serial: Serial,
    joypad: Joypad,
    irq: Irq,
}

impl Mmu {
    /// Create a new MMU instance.
    pub fn new(hw: HardwareHandle, rom: Vec<u8>) -> Mmu {
        let irq = Irq::new();

        Mmu {
            wram: Wram::new(),
            hram: Hram::new(),
            gpu: Gpu::new(hw.clone(), irq.clone()),
            mbc: Mbc::new(hw.clone(), rom),
            timer: Timer::new(irq.clone()),
            ic: Ic::new(irq.clone()),
            serial: Serial::new(hw.clone(), irq.clone()),
            joypad: Joypad::new(hw, irq.clone()),
            irq,
        }
    }

    pub fn irq(&self) -> &Irq {
        &self.irq
    }

    /// Reads one byte from the given address in the memory.
    pub fn get8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7fff => self.mbc.on_read(addr),
            0x8000..=0x9fff => self.gpu.on_read(addr),
            0xa000..=0xbfff => self.mbc.on_read(addr),
            0xc000..=0xfdff => self.wram.get8(addr),
            0xfe00..=0xfe9f => self.gpu.on_read_oam(addr),
            0xfea0..=0xfeff => unreachable!("access to unusable memory"),
            0xff00..=0xff7f => self.io_read(addr),
            0xff80..=0xfffe => self.hram.get8(addr),
            0xffff..=0xffff => self.ic.get_enabled(),
        }
    }

    /// Writes one byte at the given address in the memory.
    pub fn set8(&mut self, addr: u16, v: u8) {
        match addr {
            0x0000..=0x7fff => self.mbc.on_write(addr, v),
            0x8000..=0x9fff => self.gpu.on_write(addr, v),
            0xa000..=0xbfff => self.mbc.on_write(addr, v),
            0xc000..=0xfdff => self.wram.set8(addr, v),
            0xfe00..=0xfe9f => self.gpu.on_write_oam(addr, v),
            0xfea0..=0xfeff => unreachable!("access to unusable memory"),
            0xff00..=0xff7f => self.io_write(addr, v),
            0xff80..=0xfffe => self.hram.set8(addr, v),
            0xffff..=0xffff => self.ic.set_enabled(v),
        }
    }

    fn io_read(&self, addr: u16) -> u8 {
        match addr {
            0xff00 => self.joypad.read(),
            0xff01 => self.serial.get_data(),
            0xff02 => self.serial.get_ctrl(),
            0xff03 => todo!("i/o write: addr={:04x}", addr),
            0xff04..=0xff07 => self.timer.on_read(addr),
            0xff08..=0xff0e => todo!("i/o read: addr={:04x}", addr),
            0xff0f => self.ic.get_flags(),
            0xff10..=0xff3f => 0, // sound
            0xff40..=0xff6b => self.gpu.on_read(addr),
            0xff6c..=0xff7f => todo!("i/o read: addr={:04x}", addr),
            _ => unreachable!("read attempt to i/o addr={:04x}", addr),
        }
    }

    fn io_write(&mut self, addr: u16, v: u8) {
        match addr {
            0xff00 => self.joypad.write(v),
            0xff01 => self.serial.set_data(v),
            0xff02 => self.serial.set_ctrl(v),
            0xff03 => todo!("i/o write: addr={:04x}, v={:02x}", addr, v),
            0xff04..=0xff07 => self.timer.on_write(addr, v),
            0xff08..=0xff0e => todo!("i/o write: addr={:04x}, v={:02x}", addr, v),
            0xff0f => self.ic.set_flags(v),
            0xff10..=0xff3f => {} // sound
            0xff40..=0xff4f => self.gpu.on_write(addr, v),
            0xff50 => self.mbc.disable_boot_rom(v),
            0xff51..=0xff6b => self.gpu.on_write(addr, v),
            0xff6c..=0xff7f => todo!("i/o write: addr={:04x}, v={:02x}", addr, v),
            _ => unreachable!("write attempt to i/o addr={:04x}, v={:04x}", addr, v),
        }
    }

    /// Reads two bytes from the given addresss in the memory.
    pub fn get16(&self, addr: u16) -> u16 {
        let l = self.get8(addr);
        let h = self.get8(addr + 1);
        (h as u16) << 8 | l as u16
    }

    /// Writes two bytes at the given address in the memory.
    pub fn set16(&mut self, addr: u16, v: u16) {
        self.set8(addr, v as u8);
        self.set8(addr + 1, (v >> 8) as u8);
    }

    pub fn step(&mut self, cycles: usize) {
        self.joypad.poll();
        self.gpu.step(cycles);
        self.timer.step(cycles);
        self.serial.step(cycles);
    }
}
