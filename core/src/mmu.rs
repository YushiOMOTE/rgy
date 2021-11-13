use crate::cgb::Cgb;
use crate::cpu::Sys;
use crate::dma::{Dma, DmaRequest};
use crate::gpu::Gpu;
use crate::hardware::HardwareHandle;
use crate::hram::Hram;
use crate::ic::{Ic, Irq};
use crate::joypad::Joypad;
use crate::mbc::Mbc;
use crate::serial::Serial;
use crate::sound::Sound;
use crate::timer::Timer;
use crate::wram::Wram;
use alloc::vec::Vec;
use log::*;

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
    cgb: Cgb,
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
            cgb: Cgb::new(),
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
            0xff1e => self.sound.wave().read_freq_high(),
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
            0xff4d => self.cgb.read_speed_switch(),
            0xff4f => self.gpu.read_vram_bank_select(),
            0xff51 => self.gpu.read_hdma_src_high(),
            0xff52 => self.gpu.read_hdma_src_low(),
            0xff53 => self.gpu.read_hdma_dst_high(),
            0xff54 => self.gpu.read_hdma_dst_low(),
            0xff55 => self.gpu.read_hdma_start(),
            0xff56 => todo!("ir"),
            0xff68 => todo!("cgb bg palette index"),
            0xff69 => self.gpu.read_bg_color_palette(),
            0xff6a => todo!("cgb bg palette data"),
            0xff6b => self.gpu.read_obj_color_palette(),
            0xff70 => self.wram.get_bank(),
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
            0xff4d => self.cgb.write_speed_switch(v),
            0xff4f => self.gpu.select_vram_bank(v),
            0xff50 => self.mbc.disable_boot_rom(v),
            0xff51 => self.gpu.write_hdma_src_high(v),
            0xff52 => self.gpu.write_hdma_src_low(v),
            0xff53 => self.gpu.write_hdma_dst_high(v),
            0xff54 => self.gpu.write_hdma_dst_low(v),
            0xff55 => self.gpu.write_hdma_start(v),
            0xff56 => todo!("ir"),
            0xff68 => self.gpu.select_bg_color_palette(v),
            0xff69 => self.gpu.write_bg_color_palette(v),
            0xff6a => self.gpu.select_obj_color_palette(v),
            0xff6b => self.gpu.write_obj_color_palette(v),
            0xff70 => self.wram.select_bank(v),
            0xff7f => {} // off-by-one error in Tetris
            0x0000..=0xfeff | 0xff80..=0xffff => {
                unreachable!("write non-i/o addr={:04x}, v={:02x}", addr, v)
            }
            _ => unimplemented!("write unknown i/o addr={:04x}, v={:02x}", addr, v),
        }
    }

    fn run_dma(&mut self, req: DmaRequest) {
        debug!(
            "DMA Transfer: {:04x} to {:04x} ({:04x} bytes)",
            req.src(),
            req.dst(),
            req.len()
        );
        for i in 0..req.len() {
            self.set8(req.dst() + i, self.get8(req.src() + i));
        }
    }
}

impl Sys for Mmu {
    /// Get the interrupt vector address without clearing the interrupt flag state
    fn peek_int_vec(&self) -> Option<u8> {
        self.ic.peek()
    }

    /// Get the interrupt vector address clearing the interrupt flag state
    fn pop_int_vec(&self) -> Option<u8> {
        self.ic.pop()
    }

    /// Reads one byte from the given address in the memory.
    fn get8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7fff => self.mbc.on_read(addr),
            0x8000..=0x9fff => self.gpu.read_vram(addr),
            0xa000..=0xbfff => self.mbc.on_read(addr),
            0xc000..=0xfdff => self.wram.get8(addr),
            0xfe00..=0xfe9f => self.gpu.read_oam(addr),
            0xfea0..=0xfeff => 0, // Unusable range
            0xff00..=0xff7f => self.io_read(addr),
            0xff80..=0xfffe => self.hram.get8(addr),
            0xffff..=0xffff => self.ic.read_enabled(),
        }
    }

    /// Writes one byte at the given address in the memory.
    fn set8(&mut self, addr: u16, v: u8) {
        match addr {
            0x0000..=0x7fff => self.mbc.on_write(addr, v),
            0x8000..=0x9fff => self.gpu.write_vram(addr, v),
            0xa000..=0xbfff => self.mbc.on_write(addr, v),
            0xc000..=0xfdff => self.wram.set8(addr, v),
            0xfe00..=0xfe9f => self.gpu.write_oam(addr, v),
            0xfea0..=0xfeff => {} // Unusable range
            0xff00..=0xff7f => self.io_write(addr, v),
            0xff80..=0xfffe => self.hram.set8(addr, v),
            0xffff..=0xffff => self.ic.write_enabled(v),
        }
    }

    /// Updates the machine state by the given cycles
    fn step(&mut self, cycles: usize) {
        for req in self.dma.step(cycles) {
            self.run_dma(req);
        }
        for req in self.gpu.step(cycles) {
            self.run_dma(req);
        }
        self.timer.step(cycles);
        self.serial.step(cycles);
        self.joypad.poll();
    }
}

/// Behaves as a byte array for unit tests
pub struct Ram {
    ram: [u8; 0x10000],
}

impl Ram {
    /// Create a new ram instance.
    pub fn new() -> Self {
        Self { ram: [0; 0x10000] }
    }

    /// Write a byte array at the beginnig of the memory.
    pub fn write(&mut self, m: &[u8]) {
        for i in 0..m.len() {
            self.set8(i as u16, m[i]);
        }
    }
}

impl Sys for Ram {
    fn peek_int_vec(&self) -> Option<u8> {
        None
    }

    fn pop_int_vec(&self) -> Option<u8> {
        None
    }

    fn get8(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    fn set8(&mut self, addr: u16, v: u8) {
        self.ram[addr as usize] = v;
    }

    fn step(&mut self, _: usize) {}
}
