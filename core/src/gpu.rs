use crate::mmu::{Mmu, ReadHandler, WriteHandler};

use std::rc::Rc;
use std::cell::{Cell, RefCell};

pub trait Screen {
    fn update(&self, buffer: &[u32]);

    fn update_line(&self, line: usize, buffer: &[u32]);
}

#[derive(Debug)]
enum Mode {
    OAM,
    VRAM,
    HBlank,
    VBlank,
    None,
}

impl From<Mode> for u8 {
    fn from(v: Mode) -> u8 {
        match v {
            Mode::HBlank => 0,
            Mode::VBlank => 1,
            Mode::OAM => 2,
            Mode::VRAM => 3,
            Mode::None => 0,
        }
    }
}

impl From<u8> for Mode {
    fn from(v: u8) -> Mode {
        match v {
            0 => Mode::HBlank,
            1 => Mode::VBlank,
            2 => Mode::OAM,
            3 => Mode::VRAM,
            _ => Mode::None,
        }
    }
}

pub struct Gpu {
    inner: Rc<RefCell<Inner>>,
}

struct Inner {
    clocks: usize,
    mode: Mode,
    line: u8,
    enable: bool,
    winbase: u16,
    winenable: bool,
    bgwinbase: u16,
    bgbase: u16,
    spsize: u16,
    spenable: bool,
    bgenable: bool,
    vram: Vec<u32>,
    screen: Box<Screen>,
}

impl Inner {
    fn new(screen: Box<Screen>) -> Self {
        Self {
            clocks: 0,
            mode: Mode::None,
            line: 0,
            enable: false,
            winbase: 0x9800,
            winenable: false,
            bgwinbase: 0x8800,
            bgbase: 0x9800,
            spsize: 8,
            spenable: false,
            bgenable: false,
            vram: vec![0; 256 * 256],
            screen: screen,
        }
    }

    pub fn step(&mut self, time: usize) {
        let clocks = self.clocks + time;

        let (clocks, mode) = match &self.mode {
            Mode::OAM => {
                if clocks >= 80 {
                    (0, Mode::VRAM)
                } else {
                    (clocks, Mode::OAM)
                }
            }
            Mode::VRAM => {
                if clocks >= 172 {
                    (0, Mode::HBlank)
                } else {
                    (clocks, Mode::VRAM)
                }
            }
            Mode::HBlank => {
                if clocks >= 204 {
                    self.line += 1;

                    if self.line == 143 {
                        (0, Mode::VBlank)
                    } else {
                        (0, Mode::OAM)
                    }
                } else {
                    (clocks, Mode::HBlank)
                }
            }
            Mode::VBlank => {
                if clocks >= 456 {
                    self.line += 1;

                    if self.line > 153 {
                        (0, Mode::OAM)
                    } else {
                        (0, Mode::VBlank)
                    }
                } else {
                    (clocks, Mode::VBlank)
                }
            }
            Mode::None => (0, Mode::None),
        };

        self.clocks = clocks;
        self.mode = mode;
    }

    pub fn fetch(&self, mmu: &Mmu) {}

    fn on_write_ctrl(&mut self, value: u8) {
        let old_enable = self.enable;

        self.enable = value & 0x80 != 0;
        self.winbase = if value & 0x40 != 0 { 0x9c00 } else { 0x9800 };
        self.winenable = value & 0x20 != 0;
        self.bgwinbase = if value & 0x10 != 0 { 0x8000 } else { 0x8800 };
        self.bgbase = if value & 0x08 != 0 { 0x9c00 } else { 0x9800 };
        self.spsize = if value & 0x04 != 0 { 8 } else { 16 };
        self.spenable = value & 0x02 != 0;
        self.bgenable = value & 0x01 != 0;

        if !old_enable && self.enable {
            info!("LCD enabled");
            self.clocks = 0;
            self.mode = Mode::OAM;
        } else if old_enable && !self.enable {
            info!("LCD disabled");
            self.mode = Mode::None;
        }
    }

    fn on_read(&mut self, mmu: &Mmu, addr: u16) -> Option<u8> {
        trace!("Read GPU register: {:04x}", addr);

        if addr == 0xff44 {
            Some(self.line)
        } else {
            None
        }
    }

    fn on_write(&mut self, mmu: &Mmu, addr: u16, value: u8) -> Option<u8> {
        trace!("Write GPU register: {:04x} {:02x}", addr, value);

        if addr == 0xff40 {
            self.on_write_ctrl(value);
        }

        None
    }
}

impl Gpu {
    pub fn new(screen: Box<Screen>) -> Gpu {
        Gpu {
            inner: Rc::new(RefCell::new(Inner::new(screen))),
        }
    }

    pub fn handler(&self) -> GpuHandler {
        GpuHandler::new(self.inner.clone())
    }

    pub fn step(&self, time: usize) {
        self.inner.borrow_mut().step(time)
    }
}

#[derive(Clone)]
pub struct GpuHandler {
    inner: Rc<RefCell<Inner>>,
}

impl GpuHandler {
    fn new(inner: Rc<RefCell<Inner>>) -> GpuHandler {
        GpuHandler { inner }
    }
}

impl ReadHandler for GpuHandler {
    fn on_read(&mut self, mmu: &Mmu, addr: u16) -> Option<u8> {
        self.inner.borrow_mut().on_read(mmu, addr)
    }
}

impl WriteHandler for GpuHandler {
    fn on_write(&mut self, mmu: &Mmu, addr: u16, value: u8) -> Option<u8> {
        self.inner.borrow_mut().on_write(mmu, addr, value)
    }
}
