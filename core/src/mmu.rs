use crate::dma::Dma;
use crate::gpu::Gpu;
use crate::hardware::HardwareHandle;
use crate::ic::{Ic, Irq};
use crate::joypad::Joypad;
use crate::mbc::Mbc;
use crate::serial::Serial;
use crate::sound::Sound;
use crate::timer::Timer;
use alloc::{vec, vec::Vec};
use log::*;

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
    sound: Sound,
    dma: Dma,
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
            joypad: Joypad::new(hw.clone(), irq),
            sound: Sound::new(hw),
            dma: Dma::new(),
        }
    }

    /// Get the interrupt vector address without clearing the interrupt flag state
    pub fn peek_int_vec(&self) -> Option<u8> {
        self.ic.peek()
    }

    /// Get the interrupt vector address clearing the interrupt flag state
    pub fn pop_int_vec(&self) -> Option<u8> {
        self.ic.pop()
    }

    /// Reads one byte from the given address in the memory.
    pub fn get8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7fff => self.mbc.on_read(addr),
            0x8000..=0x9fff => self.gpu.read_vram(addr),
            0xa000..=0xbfff => self.mbc.on_read(addr),
            0xc000..=0xfdff => self.wram.get8(addr),
            0xfe00..=0xfe9f => self.gpu.read_oam(addr),
            0xfea0..=0xfeff => unimplemented!("unusable: addr={:04x}", addr),
            0xff00..=0xff7f => self.io_read(addr),
            0xff80..=0xfffe => self.hram.get8(addr),
            0xffff..=0xffff => self.ic.read_enabled(),
        }
    }

    /// Writes one byte at the given address in the memory.
    pub fn set8(&mut self, addr: u16, v: u8) {
        match addr {
            0x0000..=0x7fff => self.mbc.on_write(addr, v),
            0x8000..=0x9fff => self.gpu.write_vram(addr, v),
            0xa000..=0xbfff => self.mbc.on_write(addr, v),
            0xc000..=0xfdff => self.wram.set8(addr, v),
            0xfe00..=0xfe9f => self.gpu.write_oam(addr, v),
            0xfea0..=0xfeff => unimplemented!("unusable: addr={:04x}, value={:04x}", addr, v),
            0xff00..=0xff7f => self.io_write(addr, v),
            0xff80..=0xfffe => self.hram.set8(addr, v),
            0xffff..=0xffff => self.ic.write_enabled(v),
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
            0xff0f => self.ic.read_flags(),
            0xff10 => self.sound.tone1().read_sweep(),
            0xff11 => self.sound.tone1().read_wave(),
            0xff12 => self.sound.tone1().read_envelop(),
            0xff13 => self.sound.tone1().read_freq_low(),
            0xff14 => self.sound.tone1().read_freq_high(),
            0xff16 => self.sound.tone2().read_wave(),
            0xff17 => self.sound.tone2().read_envelop(),
            0xff18 => self.sound.tone2().read_freq_low(),
            0xff19 => self.sound.tone2().read_freq_high(),
            0xff1a => self.sound.wave().read_enable(),
            0xff1b => self.sound.wave().read_len(),
            0xff1c => self.sound.wave().read_amp(),
            0xff1d => self.sound.wave().read_freq_low(),
            0xff1e => self.sound.wave().read_freq_low(),
            0xff20 => self.sound.noise().read_len(),
            0xff21 => self.sound.noise().read_envelop(),
            0xff22 => self.sound.noise().read_poly_counter(),
            0xff23 => self.sound.noise().read_select(),
            0xff24 => self.sound.mixer().read_ctrl(),
            0xff25 => self.sound.mixer().read_so_mask(),
            0xff26 => self.sound.mixer().read_enable(),
            0xff30..=0xff3f => self.sound.wave().read_wave_buf(addr),
            0xff40 => self.gpu.read_ctrl(),
            0xff41 => self.gpu.read_status(),
            0xff42 => self.gpu.read_scy(),
            0xff43 => self.gpu.read_scx(),
            0xff44 => self.gpu.read_ly(),
            0xff45 => self.gpu.read_lyc(),
            0xff46 => self.dma.read(),
            0xff47 => self.gpu.read_bg_palette(),
            0xff48 => self.gpu.read_obj_palette0(),
            0xff49 => self.gpu.read_obj_palette1(),
            0xff4a => self.gpu.read_wy(),
            0xff4b => self.gpu.read_wx(),
            0xff4d => 0, // cgb
            0xff4f => self.gpu.read_vram_bank_select(),
            0xff50..=0xff55 => todo!("hdma"),
            0xff56 => todo!("ir"),
            0xff68 => todo!("cgb bg palette index"),
            0xff69 => self.gpu.read_bg_color_palette(),
            0xff6a => todo!("cgb bg palette data"),
            0xff6b => self.gpu.read_obj_color_palette(),
            0x0000..=0xfeff | 0xff80..=0xffff => unreachable!("read non-i/o addr={:04x}", addr),
            _ => unimplemented!("read unknown i/o addr={:04x}", addr),
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
            0xff0f => self.ic.write_flags(v),
            0xff10 => self.sound.tone1_mut().write_sweep(v),
            0xff11 => self.sound.tone1_mut().write_wave(v),
            0xff12 => self.sound.tone1_mut().write_envelop(v),
            0xff13 => self.sound.tone1_mut().write_freq_low(v),
            0xff14 => self.sound.write_tone1_start(v),
            0xff16 => self.sound.tone2_mut().write_wave(v),
            0xff17 => self.sound.tone2_mut().write_envelop(v),
            0xff18 => self.sound.tone2_mut().write_freq_low(v),
            0xff19 => self.sound.write_tone2_start(v),
            0xff1a => self.sound.write_wave_enable(v),
            0xff1b => self.sound.wave_mut().write_len(v),
            0xff1c => self.sound.wave_mut().write_amp(v),
            0xff1d => self.sound.wave_mut().write_freq_low(v),
            0xff1e => self.sound.write_wave_start(v),
            0xff20 => self.sound.noise_mut().write_len(v),
            0xff21 => self.sound.noise_mut().write_envelop(v),
            0xff22 => self.sound.noise_mut().write_poly_counter(v),
            0xff23 => self.sound.write_noise_start(v),
            0xff24 => self.sound.mixer_mut().write_ctrl(v),
            0xff25 => self.sound.mixer_mut().write_so_mask(v),
            0xff26 => self.sound.mixer_mut().write_enable(v),
            0xff30..=0xff3f => self.sound.wave_mut().write_wave_buf(addr, v),
            0xff40 => self.gpu.write_ctrl(v),
            0xff41 => self.gpu.write_status(v),
            0xff42 => self.gpu.write_scy(v),
            0xff43 => self.gpu.write_scx(v),
            0xff44 => self.gpu.clear_ly(),
            0xff45 => self.gpu.write_lyc(v),
            0xff46 => self.dma.start(v),
            0xff47 => self.gpu.write_bg_palette(v),
            0xff48 => self.gpu.write_obj_palette0(v),
            0xff49 => self.gpu.write_obj_palette1(v),
            0xff4a => self.gpu.write_wy(v),
            0xff4b => self.gpu.write_wx(v),
            0xff4d => {} // cgb
            0xff4f => self.gpu.select_vram_bank(v),
            0xff50 => self.mbc.disable_boot_rom(v),
            0xff51..=0xff55 => todo!("hdma"),
            0xff56 => todo!("ir"),
            0xff68 => self.gpu.select_bg_color_palette(v),
            0xff69 => self.gpu.write_bg_color_palette(v),
            0xff6a => self.gpu.select_obj_color_palette(v),
            0xff6b => self.gpu.write_obj_color_palette(v),
            0x0000..=0xfeff | 0xff80..=0xffff => {
                unreachable!("write non-i/o addr={:04x}, v={:02x}", addr, v)
            }
            _ => unimplemented!("write unknown i/o addr={:04x}, v={:02x}", addr, v),
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

    /// Updates the machine state by the given cycles
    pub fn step(&mut self, cycles: usize) {
        for t in self.dma.step(cycles) {
            debug!("DMA Transfer: {:02x} to {:02x}", t.src, t.dst);
            self.set8(t.dst, self.get8(t.src));
        }
        self.joypad.poll();
        self.gpu.step(cycles);
        self.timer.step(cycles);
        self.serial.step(cycles);
    }
}
