use crate::dma::DmaRequest;
use crate::hardware::{HardwareHandle, VRAM_HEIGHT, VRAM_WIDTH};
use crate::ic::Irq;
use alloc::{vec, vec::Vec};
use log::*;

#[derive(Debug, Clone, PartialEq, Eq)]
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

    oam: Vec<u8>,

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

#[allow(unused)]
struct SpriteAttribute<'a> {
    ypos: u16,
    xpos: u16,
    ti: u16,
    attr: MapAttribute<'a>,
}

struct MapAttribute<'a> {
    palette: &'a [Color],
    vram_bank: usize,
    xflip: bool,
    yflip: bool,
    priority: bool,
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

    /// Write HDMA5 register (0xff55)
    fn start(&mut self, value: u8) {
        if self.on && self.hblank && value & 0x80 == 0 {
            self.on = false;
            self.hblank = false;

            debug!("Cancel HDMA transfer");
        } else {
            self.hblank = value & 0x80 != 0;
            self.len = value & 0x7f;
            self.src_wip = ((self.src_high as u16) << 8 | self.src_low as u16) & !0x000f;
            self.dst_wip = ((self.dst_high as u16) << 8 | self.dst_low as u16) & !0xe00f | 0x8000;
            self.on = true;

            info!(
                "Start HDMA transfer: {:04x} -> {:04x} ({}) {}",
                self.src_wip, self.dst_wip, self.len, self.hblank
            );
        }
    }

    /// Read HDMA5 register (0xff55)
    fn status(&self) -> u8 {
        self.len | if self.on { 0x80 } else { 0x00 }
    }

    fn run(&mut self, hblank: bool) -> Option<DmaRequest> {
        if !self.on {
            return None;
        }

        // H-blank mode runs only in hblank.
        if self.hblank && !hblank {
            return None;
        }

        let size = if self.hblank {
            // H-blank mode copies 16 bytes.
            0x10
        } else {
            // General mode copies all bytes at once.
            (self.len as u16 + 1) * 0x10
        };

        info!(
            "HDMA transfer: {:04x} -> {:04x} ({})",
            self.src_wip, self.dst_wip, size
        );

        let req = DmaRequest::new(self.src_wip, self.dst_wip, size);

        self.src_wip += size;
        self.dst_wip += size;
        let (rem, of) = self.len.overflowing_sub(1);

        self.len = if self.hblank { rem } else { 0xff };
        self.on = if self.hblank { !of } else { false };

        Some(req)
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
            oam: vec![0; 0xa0],
            hdma: Hdma::new(),
        }
    }

    pub fn step(&mut self, time: usize) -> Option<DmaRequest> {
        let clocks = self.clocks + time;

        let (clocks, mode) = match &self.mode {
            Mode::OAM => {
                if clocks >= 80 {
                    (clocks - 80, Mode::VRAM)
                } else {
                    (clocks, Mode::OAM)
                }
            }
            Mode::VRAM => {
                if clocks >= 172 {
                    self.draw();

                    if self.hblank_interrupt {
                        self.irq.lcd(true);
                    }

                    (clocks - 172, Mode::HBlank)
                } else {
                    (clocks, Mode::VRAM)
                }
            }
            Mode::HBlank => {
                if clocks >= 204 {
                    self.ly += 1;

                    // ly becomes 144 before vblank interrupt
                    if self.ly > 143 {
                        self.irq.vblank(true);

                        if self.vblank_interrupt {
                            self.irq.lcd(true);
                        }

                        (clocks - 204, Mode::VBlank)
                    } else {
                        if self.oam_interrupt {
                            self.irq.lcd(true);
                        }

                        (clocks - 204, Mode::OAM)
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

                        (clocks - 456, Mode::OAM)
                    } else {
                        (clocks - 456, Mode::VBlank)
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

        let enter_hblank = self.mode != Mode::HBlank && mode == Mode::HBlank;

        self.clocks = clocks;
        self.mode = mode;

        self.hdma.run(enter_hblank)
    }

    fn draw(&mut self) {
        let height = VRAM_HEIGHT;
        let width = VRAM_WIDTH;

        if self.ly >= height as u8 {
            return;
        }

        let mut buf = vec![0; width];
        let mut bgbuf = vec![0; width];

        if self.bgenable {
            let mapbase = self.bgmap;

            let yy = (self.ly as u16 + self.scy as u16) % 256;
            let ty = yy / 8;
            let tyoff = yy % 8;

            for x in 0..width as u16 {
                let xx = (x + self.scx as u16) % 256;
                let tx = xx / 8;
                let txoff = xx % 8;

                let tbase = self.get_tile_base(mapbase, tx, ty);
                let tattr = self.get_tile_attr(mapbase, tx, ty);

                let tyoff = if tattr.yflip { 7 - tyoff } else { tyoff };
                let txoff = if tattr.xflip { 7 - txoff } else { txoff };

                #[cfg(feature = "color")]
                {
                    assert_eq!(tattr.priority, false);
                }

                let coli = self.get_tile_byte(tbase, txoff, tyoff, tattr.vram_bank);
                let col = tattr.palette[coli].into();

                buf[x as usize] = col;
                bgbuf[x as usize] = coli;
            }
        }

        if self.winenable {
            let mapbase = self.winmap;

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

                    let tbase = self.get_tile_base(mapbase, tx, ty);
                    let tattr = self.get_tile_attr(mapbase, tx, ty);

                    let coli = self.get_tile_byte(tbase, txoff, tyoff, tattr.vram_bank);
                    let col = tattr.palette[coli].into();

                    buf[x as usize] = col;
                }
            }
        }

        if self.spenable {
            for i in 0..40 {
                let oam = i * 4;
                let ypos = self.oam[oam] as u16;
                let xpos = self.oam[oam + 1] as u16;
                let ti = self.oam[oam + 2];
                let attr = self.get_sp_attr(self.oam[oam + 3]);

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
                let tyoff = if attr.yflip {
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
                    let txoff = if attr.xflip { 7 - txoff } else { txoff };

                    let tbase = tiles + ti as u16 * 16;

                    let coli = self.get_tile_byte(tbase, txoff, tyoff, attr.vram_bank);

                    if coli == 0 {
                        // Color index 0 means transparent
                        continue;
                    }

                    let col = attr.palette[coli];

                    let bgcoli = bgbuf[x as usize];

                    if attr.priority && bgcoli != 0 {
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

    /// Write CTRL register (0xff40)
    pub(crate) fn write_ctrl(&mut self, value: u8) {
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

        debug!("Write ctrl: {:02x}", value);
        debug!("Window base: {:04x}", self.winmap);
        debug!("Window enable: {}", self.winenable);
        debug!("Bg/window base: {:04x}", self.tiles);
        debug!("Background base: {:04x}", self.bgmap);
        debug!("Sprite size: 8x{}", self.spsize);
        debug!("Sprite enable: {}", self.spenable);
        debug!("Background enable: {}", self.bgenable);
    }

    /// Write STAT register (0xff41)
    pub(crate) fn write_status(&mut self, value: u8) {
        self.lyc_interrupt = value & 0x40 != 0;
        self.oam_interrupt = value & 0x20 != 0;
        self.vblank_interrupt = value & 0x10 != 0;
        self.hblank_interrupt = value & 0x08 != 0;

        debug!("LYC interrupt: {}", self.lyc_interrupt);
        debug!("OAM interrupt: {}", self.oam_interrupt);
        debug!("VBlank interrupt: {}", self.vblank_interrupt);
        debug!("HBlank interrupt: {}", self.hblank_interrupt);
    }

    // Read CTRL register (0xff40)
    pub(crate) fn read_ctrl(&self) -> u8 {
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

    // Write STAT register (0xff41)
    pub(crate) fn read_status(&self) -> u8 {
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

    /// Read OAM region (0xfe00 - 0xfe9f)
    pub(crate) fn read_oam(&self, addr: u16) -> u8 {
        self.oam[addr as usize - 0xfe00]
    }

    /// Write OAM region (0xfe00 - 0xfe9f)
    pub(crate) fn write_oam(&mut self, addr: u16, v: u8) {
        self.oam[addr as usize - 0xfe00] = v;
    }

    /// Read VRAM region (0x8000 - 0x9fff)
    pub(crate) fn read_vram(&self, addr: u16) -> u8 {
        self.read_vram_bank(addr, self.vram_select)
    }

    /// Write VRAM region (0x8000 - 0x9fff)
    pub(crate) fn write_vram(&mut self, addr: u16, v: u8) {
        self.write_vram_bank(addr, v, self.vram_select)
    }

    /// Read SCY register (0xff42)
    pub(crate) fn read_scy(&self) -> u8 {
        self.scy
    }

    /// Write SCY register (0xff42)
    pub(crate) fn write_scy(&mut self, v: u8) {
        self.scy = v;
    }

    /// Read SCX register (0xff43)
    pub(crate) fn read_scx(&self) -> u8 {
        self.scx
    }

    /// Write SCX register (0xff43)
    pub(crate) fn write_scx(&mut self, v: u8) {
        self.scx = v;
    }

    /// Read LY register (0xff44)
    pub(crate) fn read_ly(&self) -> u8 {
        self.ly
    }

    /// Clear LY register (0xff44)
    pub(crate) fn clear_ly(&mut self) {
        self.ly = 0;
    }

    /// Read LYC register (0xff45)
    pub(crate) fn read_lyc(&self) -> u8 {
        self.lyc
    }

    /// Write LYC register (0xff45)
    pub(crate) fn write_lyc(&mut self, v: u8) {
        self.lyc = v;
    }

    /// Read BGP register (0xff47)
    pub(crate) fn read_bg_palette(&self) -> u8 {
        debug!("Read Bg palette");
        from_palette(self.bg_palette.clone())
    }

    /// Write BGP register (0xff47)
    pub(crate) fn write_bg_palette(&mut self, v: u8) {
        self.bg_palette = to_palette(v);
        debug!("Bg palette updated: {:?}", self.bg_palette);
    }

    /// Read OBP0 register (0xff48)
    pub(crate) fn read_obj_palette0(&self) -> u8 {
        debug!("Read Object palette 0");
        from_palette(self.obj_palette0.clone())
    }

    /// Write OBP0 register (0xff48)
    pub(crate) fn write_obj_palette0(&mut self, v: u8) {
        self.obj_palette0 = to_palette(v);
        debug!("Object palette 0 updated: {:?}", self.obj_palette0);
    }

    /// Read OBP1 register (0xff49)
    pub(crate) fn read_obj_palette1(&self) -> u8 {
        debug!("Read Object palette 1");
        from_palette(self.obj_palette1.clone())
    }

    /// Write OBP1 register (0xff49)
    pub(crate) fn write_obj_palette1(&mut self, v: u8) {
        self.obj_palette1 = to_palette(v);
        debug!("Object palette 1 updated: {:?}", self.obj_palette1);
    }

    /// Read WY register (0xff4a)
    pub(crate) fn read_wy(&self) -> u8 {
        self.wy
    }

    /// Write WY register (0xff4a)
    pub(crate) fn write_wy(&mut self, v: u8) {
        self.wy = v;
    }

    /// Read WX register (0xff4b)
    pub(crate) fn read_wx(&self) -> u8 {
        self.wx
    }

    /// Write WX register (0xff4b)
    pub(crate) fn write_wx(&mut self, v: u8) {
        self.wx = v;
    }

    /// Read VBK register (0xff4f)
    pub(crate) fn read_vram_bank_select(&self) -> u8 {
        self.vram_select as u8 & 0xfe
    }

    /// Write VBK register (0xff4f)
    pub(crate) fn select_vram_bank(&mut self, v: u8) {
        self.vram_select = v as usize & 1;
    }

    /// Read HDMA1 register (0xff51)
    pub(crate) fn read_hdma_src_high(&self) -> u8 {
        self.hdma.src_high
    }

    /// Write HDMA1 register (0xff51)
    pub(crate) fn write_hdma_src_high(&mut self, v: u8) {
        self.hdma.src_high = v;
    }

    /// Read HDMA2 register (0xff52)
    pub(crate) fn read_hdma_src_low(&self) -> u8 {
        self.hdma.src_low
    }

    /// Write HDMA2 register (0xff52)
    pub(crate) fn write_hdma_src_low(&mut self, v: u8) {
        self.hdma.src_low = v;
    }

    /// Read HDMA3 register (0xff53)
    pub(crate) fn read_hdma_dst_high(&self) -> u8 {
        self.hdma.dst_high
    }

    /// Write HDMA3 register (0xff53)
    pub(crate) fn write_hdma_dst_high(&mut self, v: u8) {
        self.hdma.dst_high = v;
    }

    /// Read HDMA4 register (0xff54)
    pub(crate) fn read_hdma_dst_low(&self) -> u8 {
        self.hdma.dst_low
    }

    /// Write HDMA4 register (0xff54)
    pub(crate) fn write_hdma_dst_low(&mut self, v: u8) {
        self.hdma.dst_low = v;
    }

    /// Read HDMA5 register (0xff55)
    pub(crate) fn read_hdma_start(&self) -> u8 {
        self.hdma.status()
    }

    /// Write HDMA5 register (0xff55)
    pub(crate) fn write_hdma_start(&mut self, v: u8) {
        self.hdma.start(v);
    }

    /// Write BCPS/BGPI register (0xff68)
    pub(crate) fn select_bg_color_palette(&mut self, v: u8) {
        self.bg_color_palette.select(v);
    }

    /// Read BCPD/BGPD register (0xff69)
    pub(crate) fn read_bg_color_palette(&self) -> u8 {
        self.bg_color_palette.read()
    }

    /// Write BCPD/BGPD register (0xff69)
    pub(crate) fn write_bg_color_palette(&mut self, v: u8) {
        self.bg_color_palette.write(v);
    }

    /// Write OCPS/OBPI register (0xff6a)
    pub(crate) fn select_obj_color_palette(&mut self, v: u8) {
        self.obj_color_palette.select(v);
    }

    /// Read OCPD/OBPD register (0xff6b)
    pub(crate) fn read_obj_color_palette(&self) -> u8 {
        self.obj_color_palette.read()
    }

    /// Write OCPD/OBPD register (0xff6b)
    pub(crate) fn write_obj_color_palette(&mut self, v: u8) {
        self.obj_color_palette.write(v);
    }

    fn read_vram_bank(&self, addr: u16, bank: usize) -> u8 {
        let off = addr as usize - 0x8000;
        self.vram[bank][off]
    }

    fn write_vram_bank(&mut self, addr: u16, value: u8, bank: usize) {
        let off = addr as usize - 0x8000;
        self.vram[bank][off] = value;
    }

    fn get_tile_base(&self, mapbase: u16, tx: u16, ty: u16) -> u16 {
        let ti = tx + ty * 32;
        let num = self.read_vram_bank(mapbase + ti, 0);

        if self.tiles == 0x8000 {
            self.tiles + num as u16 * 16
        } else {
            self.tiles + (0x800 + num as i8 as i16 * 16) as u16
        }
    }

    fn get_tile_attr(&self, mapbase: u16, tx: u16, ty: u16) -> MapAttribute {
        if cfg!(feature = "color") {
            let ti = tx + ty * 32;
            let attr = self.read_vram_bank(mapbase + ti, 1) as usize;

            MapAttribute {
                palette: &self.bg_color_palette.cols[attr & 0x7][..],
                vram_bank: (attr >> 3) & 1,
                xflip: attr & 0x20 != 0,
                yflip: attr & 0x40 != 0,
                priority: attr & 0x80 != 0,
            }
        } else {
            MapAttribute {
                palette: &self.bg_palette,
                vram_bank: 0,
                xflip: false,
                yflip: false,
                priority: false,
            }
        }
    }

    fn get_sp_attr(&self, attr: u8) -> MapAttribute {
        if cfg!(feature = "color") {
            let attr = attr as usize;

            MapAttribute {
                palette: &self.obj_color_palette.cols[attr & 0x7][..],
                vram_bank: (attr >> 3) & 1,
                xflip: attr & 0x20 != 0,
                yflip: attr & 0x40 != 0,
                priority: attr & 0x80 != 0,
            }
        } else {
            let palette = if attr & 0x10 != 0 {
                &self.obj_palette1
            } else {
                &self.obj_palette0
            };

            MapAttribute {
                palette,
                vram_bank: 0,
                xflip: attr & 0x20 != 0,
                yflip: attr & 0x40 != 0,
                priority: attr & 0x80 != 0,
            }
        }
    }

    fn get_tile_byte(&self, tilebase: u16, txoff: u16, tyoff: u16, bank: usize) -> usize {
        let l = self.read_vram_bank(tilebase + tyoff * 2, bank);
        let h = self.read_vram_bank(tilebase + tyoff * 2 + 1, bank);

        let l = (l >> (7 - txoff)) & 1;
        let h = ((h >> (7 - txoff)) & 1) << 1;

        (h | l) as usize
    }
}
