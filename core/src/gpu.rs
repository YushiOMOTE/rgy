use crate::device::IoHandler;
use crate::hardware::{HardwareHandle, VRAM_HEIGHT, VRAM_WIDTH};
use crate::ic::Irq;
use crate::mmu::{MemRead, MemWrite, Mmu};
use alloc::{vec, vec::Vec};
use log::*;

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
    winmap: u16,
    winenable: bool,
    tiles: u16,
    bgmap: u16,
    spsize: u16,
    spenable: bool,
    bgenable: bool,
    hw: HardwareHandle,

    bg_palette: Vec<Color>,
    obj_palette0: Vec<Color>,
    obj_palette1: Vec<Color>,
    bg_color_palette: ColorPalette,
    obj_color_palette: ColorPalette,
    vram: Vec<Vec<u8>>,
    vram_select: usize,

    dma: Dma,
    hdma: Hdma,
}

fn to_palette(p: u8) -> Vec<Color> {
    vec![
        ((p >> 0) & 0x3).into(),
        ((p >> 2) & 0x3).into(),
        ((p >> 4) & 0x3).into(),
        ((p >> 6) & 0x3).into(),
    ]
}

fn from_palette(p: Vec<Color>) -> u8 {
    assert_eq!(p.len(), 4);

    u8::from(p[0]) | u8::from(p[1]) << 2 | u8::from(p[2]) << 4 | u8::from(p[3]) << 6
}

struct ColorPalette {
    cols: Vec<Vec<Color>>,
    index: usize,
    auto_inc: bool,
}

impl ColorPalette {
    fn new() -> Self {
        Self {
            cols: vec![vec![Color::rgb(); 4]; 8],
            index: 0,
            auto_inc: false,
        }
    }

    fn select(&mut self, value: u8) {
        self.auto_inc = value & 0x80 != 0;
        self.index = value as usize & 0x3f;
    }

    fn read(&self) -> u8 {
        let idx = self.index / 8;
        let off = self.index % 8;

        if off % 2 == 0 {
            self.cols[idx][off / 2].get_low()
        } else {
            self.cols[idx][off / 2].get_high()
        }
    }

    fn write(&mut self, value: u8) {
        let idx = self.index / 8;
        let off = self.index % 8;

        if off % 2 == 0 {
            self.cols[idx][off / 2].set_low(value)
        } else {
            self.cols[idx][off / 2].set_high(value)
        }

        if self.auto_inc {
            self.index = (self.index + 1) % 0x40;
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Color {
    White,
    LightGray,
    DarkGray,
    Black,
    Rgb(u8, u8, u8),
}

impl Color {
    fn rgb() -> Self {
        Color::Rgb(0, 0, 0)
    }

    fn set_low(&mut self, low: u8) {
        match *self {
            Color::Rgb(_, g, b) => {
                let nr = low & 0x1f;
                let ng = g & !0x7 | low >> 5;
                *self = Color::Rgb(nr, ng, b);
            }
            _ => unreachable!(),
        }
    }

    fn set_high(&mut self, high: u8) {
        match *self {
            Color::Rgb(r, g, _) => {
                let ng = g & !0x18 | (high & 0x3) << 3;
                let nb = (high >> 2) & 0x1f;
                *self = Color::Rgb(r, ng, nb);
            }
            _ => unreachable!(),
        }
    }

    fn get_low(&self) -> u8 {
        match *self {
            Color::Rgb(r, g, _) => (r & 0x1f) | (g & 0x7) << 5,
            _ => unreachable!(),
        }
    }

    fn get_high(&self) -> u8 {
        match *self {
            Color::Rgb(_, g, b) => ((g >> 3) & 0x3) | (b & 0x1f) << 2,
            _ => unreachable!(),
        }
    }
}

fn color_adjust(v: u8) -> u32 {
    let v = v as u32;

    if v >= 0x10 {
        0xff - (0x1f - v)
    } else {
        v
    }
}

impl From<Color> for u32 {
    fn from(c: Color) -> u32 {
        match c {
            Color::White => 0xdddddd,
            Color::LightGray => 0xaaaaaa,
            Color::DarkGray => 0x888888,
            Color::Black => 0x555555,
            Color::Rgb(r, g, b) => {
                let mut c = 0;
                c |= color_adjust(r) << 16;
                c |= color_adjust(g) << 8;
                c |= color_adjust(b);
                c
            }
        }
    }
}

impl From<Color> for u8 {
    fn from(c: Color) -> u8 {
        match c {
            Color::White => 0,
            Color::LightGray => 1,
            Color::DarkGray => 2,
            Color::Black => 3,
            _ => unreachable!(),
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

struct Dma {
    on: bool,
    src: u8,
}

impl Dma {
    fn new() -> Self {
        Self { on: false, src: 0 }
    }

    fn run(&mut self, mmu: &mut Mmu) {
        if self.on {
            debug!("Trigger DMA transfer: {:02x}", self.src);

            let src = (self.src as u16) << 8;
            for i in 0..0xa0 {
                mmu.set8(0xfe00 + i, mmu.get8(src + i));
            }

            self.on = false;
        }
    }
}

struct Hdma {
    on: bool,
    src_low: u8,
    src_high: u8,
    dst_low: u8,
    dst_high: u8,
    src_wip: u16,
    dst_wip: u16,
    len: u8,
    hblank: bool,
}

impl Hdma {
    fn new() -> Self {
        Self {
            on: false,
            src_low: 0,
            src_high: 0,
            dst_low: 0,
            dst_high: 0,
            src_wip: 0,
            dst_wip: 0,
            len: 0,
            hblank: false,
        }
    }

    fn start(&mut self, value: u8) {
        if self.on && self.hblank && value & 0x80 == 0 {
            self.on = false;
            self.hblank = false;

            debug!("Cancel HDMA transfer");
        } else {
            self.hblank = value & 0x80 != 0;
            self.len = value & 0x7f;
            self.src_wip = ((self.src_high as u16) << 8 | self.src_low as u16) & !0x000f;
            self.dst_wip = ((self.dst_high as u16) << 8 | self.dst_low as u16) & !0xe00f;
            self.on = true;

            debug!(
                "Start HDMA transfer: {:04x} -> {:04x} ({})",
                self.src_wip, self.dst_wip, self.len
            );
        }
    }

    fn run(&mut self, mmu: &mut Mmu) {
        if self.on {
            let size = (self.len as u16 + 1) * 0x10;
            let size = if self.hblank { size.min(0x10) } else { size };

            debug!(
                "HDMA transfer: {:04x} -> {:04x} ({})",
                self.src_wip, self.dst_wip, size
            );

            for i in 0..size {
                mmu.set8(self.dst_wip + i, mmu.get8(self.src_wip + i));
            }

            self.src_wip += size;
            self.dst_wip += size;
            self.len = self.len.saturating_sub(1);

            self.on = if self.hblank { self.len > 0 } else { false };
        }
    }
}

impl Gpu {
    pub fn new(hw: HardwareHandle, irq: Irq) -> Self {
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
            winmap: 0x9800,
            winenable: false,
            tiles: 0x8800,
            bgmap: 0x9800,
            spsize: 8,
            spenable: false,
            bgenable: false,
            hw,
            bg_palette: vec![
                Color::White,
                Color::LightGray,
                Color::DarkGray,
                Color::Black,
            ],
            obj_palette0: vec![
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
            bg_color_palette: ColorPalette::new(),
            obj_color_palette: ColorPalette::new(),
            vram: vec![vec![0; 0x2000]; 2],
            vram_select: 0,
            dma: Dma::new(),
            hdma: Hdma::new(),
        }
    }

    pub fn step(&mut self, time: usize, mmu: &mut Mmu) {
        self.dma.run(mmu);

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
                    self.hdma.run(mmu);

                    if self.hblank_interrupt {
                        self.irq.lcd(true);
                    }

                    (0, Mode::HBlank)
                } else {
                    (clocks, Mode::VRAM)
                }
            }
            Mode::HBlank => {
                if clocks >= 204 {
                    self.ly += 1;

                    if self.ly >= 143 {
                        self.irq.vblank(true);

                        if self.vblank_interrupt {
                            self.irq.lcd(true);
                        }

                        (0, Mode::VBlank)
                    } else {
                        if self.oam_interrupt {
                            self.irq.lcd(true);
                        }

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

                        if self.oam_interrupt {
                            self.irq.lcd(true);
                        }

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

        if self.lyc_interrupt && self.lyc == self.ly {
            self.irq.lcd(true);
        }

        self.clocks = clocks;
        self.mode = mode;
    }

    fn draw(&mut self, mmu: &Mmu) {
        let height = VRAM_HEIGHT;
        let width = VRAM_WIDTH;

        if self.ly >= height as u8 {
            return;
        }

        let mut buf = vec![0; width];
        let mut bgbuf = vec![0; width];

        if self.bgenable {
            let mapbase = self.bgmap;
            let tiles = self.tiles;

            let yy = (self.ly as u16 + self.scy as u16) % 256;
            let ty = yy / 8;
            let tyoff = yy % 8;

            for x in 0..width as u16 {
                let xx = (x + self.scx as u16) % 256;
                let tx = xx / 8;
                let txoff = xx % 8;

                let ti = tx + ty * 32;
                let tbase = if tiles == 0x8000 {
                    tiles + mmu.get8(mapbase + ti) as u16 * 16
                } else {
                    tiles + (0x800 + mmu.get8(mapbase + ti) as i8 as i16 * 16) as u16
                };

                let l = mmu.get8(tbase + tyoff * 2) as u16;
                let h = mmu.get8(tbase + tyoff * 2 + 1) as u16;

                let l = (l >> (7 - txoff)) & 1;
                let h = ((h >> (7 - txoff)) & 1) << 1;
                let coli = (h | l) as usize;
                let col = self.bg_palette[coli].into();

                buf[x as usize] = col;
                bgbuf[x as usize] = coli;
            }
        }

        if self.winenable {
            let mapbase = self.winmap;
            let tiles = self.tiles;

            if self.ly >= self.wy {
                let yy = (self.ly - self.wy) as u16;
                let ty = yy / 8;
                let tyoff = yy % 8;

                for x in 0..width as u16 {
                    if x + 7 < self.wx as u16 {
                        continue;
                    }
                    let xx = (x + 7 - self.wx as u16) as u16; // x - (wx - 7)
                    let tx = xx / 8;
                    let txoff = xx % 8;

                    let ti = tx + ty * 32;
                    let tbase = if tiles == 0x8000 {
                        tiles + mmu.get8(mapbase + ti) as u16 * 16
                    } else {
                        tiles + (0x800 + mmu.get8(mapbase + ti) as i8 as i16 * 16) as u16
                    };

                    let l = mmu.get8(tbase + tyoff * 2) as u16;
                    let h = mmu.get8(tbase + tyoff * 2 + 1) as u16;

                    let l = (l >> (7 - txoff)) & 1;
                    let h = ((h >> (7 - txoff)) & 1) << 1;
                    let col = self.bg_palette[(h | l) as usize].into();

                    buf[x as usize] = col;
                }
            }
        }

        if self.spenable {
            for i in 0..40 {
                let oam = 0xfe00 + i * 4;
                let ypos = mmu.get8(oam + 0) as u16;
                let xpos = mmu.get8(oam + 1) as u16;
                let ti = mmu.get8(oam + 2);
                let attr = mmu.get8(oam + 3);
                let behind_bg = attr & 0x80 != 0;
                let yflip = attr & 0x40 != 0;
                let xflip = attr & 0x20 != 0;
                let palette = if attr & 0x10 == 0 {
                    &self.obj_palette0
                } else {
                    &self.obj_palette1
                };

                let ly = self.ly as u16;
                if ly + 16 < ypos {
                    // This sprite doesn't hit the current ly
                    continue;
                }
                let tyoff = ly as u16 + 16 - ypos; // ly - (ypos - 16)
                if tyoff >= self.spsize {
                    // This sprite doesn't hit the current ly
                    continue;
                }
                let tyoff = if yflip {
                    self.spsize - 1 - tyoff
                } else {
                    tyoff
                };
                let ti = if self.spsize == 16 {
                    if tyoff >= 8 {
                        ti | 1
                    } else {
                        ti & 0xfe
                    }
                } else {
                    ti
                };
                let tyoff = tyoff % 8;

                let tiles = 0x8000;

                for x in 0..width as u16 {
                    if x + 8 < xpos {
                        continue;
                    }
                    let txoff = x + 8 - xpos; // x - (xpos - 8)
                    if txoff >= 8 {
                        continue;
                    }
                    let txoff = if xflip { 7 - txoff } else { txoff };

                    let tbase = tiles + ti as u16 * 16;

                    let l = mmu.get8(tbase + tyoff * 2) as u16;
                    let h = mmu.get8(tbase + tyoff * 2 + 1) as u16;

                    let l = (l >> (7 - txoff)) & 1;
                    let h = ((h >> (7 - txoff)) & 1) << 1;
                    let coli = (h | l) as usize;

                    if coli == 0 {
                        // Color index 0 means transparent
                        continue;
                    }

                    let col = palette[coli];

                    let bgcoli = bgbuf[x as usize];

                    if behind_bg && bgcoli != 0 {
                        // If priority is lower than bg color 1-3, don't draw
                        continue;
                    }

                    buf[x as usize] = col.into();
                }
            }
        }

        self.hw
            .get()
            .borrow_mut()
            .vram_update(self.ly as usize, &buf);
    }

    fn on_write_ctrl(&mut self, value: u8) {
        let old_enable = self.enable;

        self.enable = value & 0x80 != 0;
        self.winmap = if value & 0x40 != 0 { 0x9c00 } else { 0x9800 };
        self.winenable = value & 0x20 != 0;
        self.tiles = if value & 0x10 != 0 { 0x8000 } else { 0x8800 };
        self.bgmap = if value & 0x08 != 0 { 0x9c00 } else { 0x9800 };
        self.spsize = if value & 0x04 != 0 { 16 } else { 8 };
        self.spenable = value & 0x02 != 0;
        self.bgenable = value & 0x01 != 0;

        if !old_enable && self.enable {
            info!("LCD enabled");
            self.clocks = 0;
            self.mode = Mode::HBlank;
            self.irq.vblank(false);
        } else if old_enable && !self.enable {
            info!("LCD disabled");
            self.mode = Mode::None;
            self.irq.vblank(false);
        }

        info!("Write ctrl: {:02x}", value);
        info!("Window base: {:04x}", self.winmap);
        info!("Window enable: {}", self.winenable);
        info!("Bg/window base: {:04x}", self.tiles);
        info!("Background base: {:04x}", self.bgmap);
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
        v |= if self.winmap == 0x9c00 { 0x40 } else { 0x00 };
        v |= if self.winenable { 0x20 } else { 0x00 };
        v |= if self.tiles == 0x8000 { 0x10 } else { 0x00 };
        v |= if self.bgmap == 0x9c00 { 0x08 } else { 0x00 };
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
        trace!("Read Status: {:02x}", v);
        v
    }
}

impl IoHandler for Gpu {
    fn on_read(&mut self, _mmu: &Mmu, addr: u16) -> MemRead {
        if addr >= 0x8000 && addr <= 0x9fff {
            let off = addr as usize - 0x8000;
            MemRead::Replace(self.vram[self.vram_select][off])
        } else if addr == 0xff40 {
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
        } else if addr == 0xff46 {
            warn!("Read DMA transfer");
            MemRead::PassThrough
        } else if addr == 0xff47 {
            debug!("Read Bg palette");
            MemRead::Replace(from_palette(self.bg_palette.clone()))
        } else if addr == 0xff48 {
            debug!("Read Object palette 0");
            MemRead::Replace(from_palette(self.obj_palette0.clone()))
        } else if addr == 0xff49 {
            debug!("Read Object palette 1");
            MemRead::Replace(from_palette(self.obj_palette1.clone()))
        } else if addr == 0xff4a {
            MemRead::Replace(self.wy)
        } else if addr == 0xff4b {
            MemRead::Replace(self.wx)
        } else if addr == 0xff4f {
            MemRead::Replace(self.vram_select as u8)
        } else if addr == 0xff51 {
            MemRead::Replace(self.hdma.src_high)
        } else if addr == 0xff52 {
            MemRead::Replace(self.hdma.src_low)
        } else if addr == 0xff53 {
            MemRead::Replace(self.hdma.dst_high)
        } else if addr == 0xff54 {
            MemRead::Replace(self.hdma.dst_low)
        } else if addr == 0xff55 {
            let mut v = 0;
            v |= self.hdma.len & 0x7f;
            v |= if self.hdma.hblank { 0x80 } else { 0x00 };
            MemRead::Replace(v)
        } else if addr == 0xff68 {
            MemRead::PassThrough
        } else if addr == 0xff69 {
            MemRead::Replace(self.bg_color_palette.read())
        } else if addr == 0xff6a {
            MemRead::PassThrough
        } else if addr == 0xff6b {
            MemRead::Replace(self.obj_color_palette.read())
        } else {
            warn!("Unsupported GPU register read: {:04x}", addr);
            MemRead::Replace(0)
        }
    }

    fn on_write(&mut self, _mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        trace!("Write GPU register: {:04x} {:02x}", addr, value);
        if addr >= 0x8000 && addr <= 0x9fff {
            let off = addr as usize - 0x8000;
            self.vram[self.vram_select][off] = value;
        } else if addr == 0xff40 {
            self.on_write_ctrl(value);
        } else if addr == 0xff41 {
            self.on_write_status(value);
        } else if addr == 0xff42 {
            self.scy = value;
        } else if addr == 0xff43 {
            info!("Write SCX: {}", value);
            self.scx = value;
        } else if addr == 0xff44 {
            self.ly = 0;
        } else if addr == 0xff45 {
            self.lyc = value;
        } else if addr == 0xff46 {
            debug!("Request DMA: {:02x}", value);
            self.dma.on = true;
            self.dma.src = value;
        } else if addr == 0xff47 {
            self.bg_palette = to_palette(value);
            debug!("Bg palette updated: {:?}", self.bg_palette);
        } else if addr == 0xff48 {
            self.obj_palette0 = to_palette(value);
            debug!("Object palette 0 updated: {:?}", self.obj_palette0);
        } else if addr == 0xff49 {
            self.obj_palette1 = to_palette(value);
            debug!("Object palette 1 updated: {:?}", self.obj_palette1);
        } else if addr == 0xff4a {
            info!("Window Y: {}", value);
            self.wy = value;
        } else if addr == 0xff4b {
            info!("Window X: {}", value);
            self.wx = value;
        } else if addr == 0xff4f {
            self.vram_select = value as usize & 1;
        } else if addr == 0xff51 {
            self.hdma.src_high = value;
        } else if addr == 0xff52 {
            self.hdma.src_low = value;
        } else if addr == 0xff53 {
            self.hdma.dst_high = value;
        } else if addr == 0xff54 {
            self.hdma.dst_low = value;
        } else if addr == 0xff55 {
            self.hdma.start(value);
        } else if addr == 0xff68 {
            self.bg_color_palette.select(value);
        } else if addr == 0xff69 {
            self.bg_color_palette.write(value);
        } else if addr == 0xff6a {
            self.obj_color_palette.select(value);
        } else if addr == 0xff6b {
            self.obj_color_palette.write(value);
        } else {
            warn!(
                "Unsupported GPU register is written: {:04x} {:02x}",
                addr, value
            );
        }

        MemWrite::PassThrough
    }
}
