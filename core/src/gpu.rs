use crate::mmu::{MemHandler, MemRead, MemWrite, Mmu};
use crate::ic::Irq;
use std::rc::Rc;
use std::cell::RefCell;
use log::*;

pub trait Screen {
    fn width(&self) -> usize;

    fn height(&self) -> usize;

    fn update(&self, buffer: &[u32]);

    fn update_line(&self, line: usize, buffer: &[u32]);
}

#[derive(Debug, Clone)]
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
    irq: Irq,

    clocks: usize,

    lyc_interrupt: bool,
    oam_interrupt: bool,
    vblank_interrupt: bool,
    hblank_interrupt: bool,
    mode: Mode,

    ly: u8,
    lyc: u8,
    scy: u8,
    scx: u8,

    wx: u8,
    wy: u8,

    enable: bool,
    winbase: u16,
    winenable: bool,
    bgwinbase: u16,
    bgbase: u16,
    spsize: u16,
    spenable: bool,
    bgenable: bool,
    screen: Box<Screen>,

    bg_palette: Vec<Color>,
    obj_palette1: Vec<Color>,
    obj_palette2: Vec<Color>,
}

fn to_palette(p: u8) -> Vec<Color> {
    vec![
        ((p >> 0) & 0x3).into(),
        ((p >> 2) & 0x3).into(),
        ((p >> 4) & 0x3).into(),
        ((p >> 6) & 0x3).into(),
    ]
}

#[derive(Clone, Debug)]
enum Color {
    White,
    LightGray,
    DarkGray,
    Black,
}

impl From<Color> for u32 {
    fn from(c: Color) -> u32 {
        match c {
            Color::White => 0xdddddd,
            Color::LightGray => 0xaaaaaa,
            Color::DarkGray => 0x888888,
            Color::Black => 0x555555,
        }
    }
}

impl From<u8> for Color {
    fn from(v: u8) -> Color {
        match v {
            0 => Color::White,
            1 => Color::LightGray,
            2 => Color::DarkGray,
            3 => Color::Black,
            _ => unreachable!(),
        }
    }
}

impl Inner {
    fn new(screen: Box<Screen>, irq: Irq) -> Self {
        Self {
            irq: irq,
            clocks: 0,
            lyc_interrupt: false,
            oam_interrupt: false,
            vblank_interrupt: false,
            hblank_interrupt: false,
            mode: Mode::None,
            ly: 0,
            lyc: 0,
            scy: 0,
            scx: 0,
            wx: 0,
            wy: 0,
            enable: false,
            winbase: 0x9800,
            winenable: false,
            bgwinbase: 0x8800,
            bgbase: 0x9800,
            spsize: 8,
            spenable: false,
            bgenable: false,
            screen: screen,
            bg_palette: vec![
                Color::White,
                Color::LightGray,
                Color::DarkGray,
                Color::Black,
            ],
            obj_palette1: vec![
                Color::White,
                Color::LightGray,
                Color::DarkGray,
                Color::Black,
            ],
            obj_palette2: vec![
                Color::White,
                Color::LightGray,
                Color::DarkGray,
                Color::Black,
            ],
        }
    }

    fn step(&mut self, time: usize, mmu: &mut Mmu) {
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
                    self.draw(mmu);

                    (0, Mode::HBlank)
                } else {
                    (clocks, Mode::VRAM)
                }
            }
            Mode::HBlank => {
                if clocks >= 204 {
                    self.ly += 1;

                    if self.ly == 143 {
                        self.irq.vblank(true);

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
                    self.ly += 1;

                    if self.ly > 153 {
                        self.ly = 0;
                        self.irq.vblank(false);

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

    fn draw(&mut self, mmu: &Mmu) {
        if self.ly >= self.screen.height() as u8 {
            return;
        }

        let mut buf = vec![0; self.screen.width()];

        let tmapbase = self.bgbase;
        let tsetbase = self.bgwinbase;

        let yy = (self.ly as u16 + self.scy as u16) % 256;
        let ty = yy / 8;
        let tyoff = yy % 8;

        for x in 0..self.screen.width() as u16 {
            let xx = (x + self.scx as u16) % 256;
            let tx = xx / 8;
            let txoff = xx % 8;

            let ti = tx + ty * 32;
            let tbase = tsetbase + mmu.get8(tmapbase + ti) as u16 * 16;

            let l = mmu.get8(tbase + tyoff * 2) as u16;
            let h = mmu.get8(tbase + tyoff * 2 + 1) as u16;

            let l = (l >> (7 - txoff)) & 1;
            let h = ((h >> (7 - txoff)) & 1) << 1;
            let col = self.bg_palette[(h | l) as usize].clone().into();

            buf[x as usize] = col;
        }

        self.screen.update_line(self.ly as usize, &buf);
    }

    fn on_write_ctrl(&mut self, value: u8) {
        let old_enable = self.enable;

        self.enable = value & 0x80 != 0;
        self.winbase = if value & 0x40 != 0 { 0x9c00 } else { 0x9800 };
        self.winenable = value & 0x20 != 0;
        self.bgwinbase = if value & 0x10 != 0 { 0x8000 } else { 0x8800 };
        self.bgbase = if value & 0x08 != 0 { 0x9c00 } else { 0x9800 };
        self.spsize = if value & 0x04 != 0 { 16 } else { 8 };
        self.spenable = value & 0x02 != 0;
        self.bgenable = value & 0x01 != 0;

        if !old_enable && self.enable {
            info!("LCD enabled");
            self.clocks = 0;
            self.mode = Mode::OAM;
            self.irq.vblank(false);
        } else if old_enable && !self.enable {
            info!("LCD disabled");
            self.mode = Mode::None;
            self.irq.vblank(false);
        }

        info!("Window base: {:04x}", self.winbase);
        info!("Window enable: {}", self.winenable);
        info!("Bg/window base: {:04x}", self.bgwinbase);
        info!("Background base: {:04x}", self.bgbase);
        info!("Sprite size: 8x{}", self.spsize);
        info!("Sprite enable: {}", self.spenable);
        info!("Background enable: {}", self.bgenable);
    }

    fn on_write_status(&mut self, value: u8) {
        self.lyc_interrupt = value & 0x40 != 0;
        self.oam_interrupt = value & 0x20 != 0;
        self.vblank_interrupt = value & 0x10 != 0;
        self.hblank_interrupt = value & 0x08 != 0;

        info!("LYC interrupt: {}", self.lyc_interrupt);
        info!("OAM interrupt: {}", self.oam_interrupt);
        info!("VBlank interrupt: {}", self.vblank_interrupt);
        info!("HBlank interrupt: {}", self.hblank_interrupt);
    }

    fn on_read_ctrl(&mut self) -> u8 {
        let mut v = 0;
        v |= if self.enable { 0x80 } else { 0x00 };
        v |= if self.winbase == 0x9c00 { 0x40 } else { 0x00 };
        v |= if self.winenable { 0x20 } else { 0x00 };
        v |= if self.bgwinbase == 0x8000 { 0x10 } else { 0x00 };
        v |= if self.bgbase == 0x9c00 { 0x08 } else { 0x00 };
        v |= if self.spsize == 16 { 0x04 } else { 0x00 };
        v |= if self.spenable { 0x02 } else { 0x00 };
        v |= if self.bgenable { 0x01 } else { 0x00 };
        v
    }

    fn on_read_status(&mut self) -> u8 {
        let mut v = 0;
        v |= if self.lyc_interrupt { 0x40 } else { 0x00 };
        v |= if self.oam_interrupt { 0x20 } else { 0x00 };
        v |= if self.vblank_interrupt { 0x10 } else { 0x00 };
        v |= if self.hblank_interrupt { 0x08 } else { 0x00 };
        v |= if self.ly == self.lyc { 0x04 } else { 0x00 };
        v |= {
            let p: u8 = self.mode.clone().into();
            p
        };
        v
    }

    fn on_read(&mut self, mmu: &Mmu, addr: u16) -> MemRead {
        if addr != 0xff44 {
            trace!("Read GPU register: {:04x}", addr);
        }

        if addr == 0xff40 {
            MemRead::Replace(self.on_read_ctrl())
        } else if addr == 0xff41 {
            MemRead::Replace(self.on_read_status())
        } else if addr == 0xff42 {
            MemRead::Replace(self.scy)
        } else if addr == 0xff43 {
            MemRead::Replace(self.scx)
        } else if addr == 0xff44 {
            MemRead::Replace(self.ly)
        } else if addr == 0xff45 {
            MemRead::Replace(self.lyc)
        } else if addr == 0xff47 {
            unimplemented!("read ff47")
        } else if addr == 0xff48 {
            unimplemented!("read ff48")
        } else if addr == 0xff49 {
            unimplemented!("read ff49")
        } else {
            MemRead::PassThrough
        }
    }

    fn on_write(&mut self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        trace!("Write GPU register: {:04x} {:02x}", addr, value);

        if addr == 0xff40 {
            self.on_write_ctrl(value);
        } else if addr == 0xff41 {
            self.on_write_status(value);
        } else if addr == 0xff42 {
            self.scy = value;
        } else if addr == 0xff43 {
            self.scx = value;
        } else if addr == 0xff44 {
            self.ly = 0;
        } else if addr == 0xff45 {
            self.lyc = value;
        } else if addr == 0xff47 {
            self.bg_palette = to_palette(value);
            info!("Bg palette updated: {:?}", self.bg_palette);
        } else if addr == 0xff48 {
            self.obj_palette1 = to_palette(value);
            info!("Object palette 1 updated: {:?}", self.obj_palette2);
        } else if addr == 0xff49 {
            self.obj_palette2 = to_palette(value);
            info!("Object palette 2 updated: {:?}", self.obj_palette1);
        } else if addr == 0xff4a {
            info!("Window Y: {}", value);
            self.wy = value;
        } else if addr == 0xff4b {
            info!("Window X: {}", value);
            self.wx = value;
        }

        MemWrite::PassThrough
    }
}

impl Gpu {
    pub fn new(screen: Box<Screen>, irq: Irq) -> Gpu {
        Gpu {
            inner: Rc::new(RefCell::new(Inner::new(screen, irq))),
        }
    }

    pub fn handler(&self) -> GpuMemHandler {
        GpuMemHandler::new(self.inner.clone())
    }

    pub fn step(&self, time: usize, mmu: &mut Mmu) {
        self.inner.borrow_mut().step(time, mmu)
    }
}

pub struct GpuMemHandler {
    inner: Rc<RefCell<Inner>>,
}

impl GpuMemHandler {
    fn new(inner: Rc<RefCell<Inner>>) -> GpuMemHandler {
        GpuMemHandler { inner }
    }
}

impl MemHandler for GpuMemHandler {
    fn on_read(&self, mmu: &Mmu, addr: u16) -> MemRead {
        self.inner.borrow_mut().on_read(mmu, addr)
    }

    fn on_write(&self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        self.inner.borrow_mut().on_write(mmu, addr, value)
    }
}
