use crate::device::IoHandler;
use crate::hardware::{HardwareHandle, VRAM_HEIGHT, VRAM_WIDTH};
use crate::ic::Irq;
use crate::mmu::{MemRead, MemWrite, Mmu};
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

#[derive(Clone, Copy, Debug)]
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

impl From<Color> for u8 {
    fn from(c: Color) -> u8 {
        match c {
            Color::White => 0,
            Color::LightGray => 1,
            Color::DarkGray => 2,
            Color::Black => 3,
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
        }
    }

    pub fn step(&mut self, time: usize, mmu: &mut Mmu) {
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

                    if self.ly == 143 {
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
            unimplemented!("read ff4f (vram bank)")
        } else if addr == 0xff68 || addr == 0xff69 || addr == 0xff6a || addr == 0xff6b {
            unimplemented!("read color")
        } else {
            warn!("Unsupported GPU register read: {:04x}", addr);
            MemRead::Replace(0)
        }
    }

    fn on_write(&mut self, _mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        trace!("Write GPU register: {:04x} {:02x}", addr, value);

        if addr == 0xff40 {
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
            trace!("DMA is handled by MMU: {:02x}", value);
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
            unimplemented!("write ff4f (vram bank)")
        } else if addr == 0xff68 || addr == 0xff69 || addr == 0xff6a || addr == 0xff6b {
            unimplemented!("write color")
        } else {
            warn!(
                "Unsupported GPU register is written: {:04x} {:02x}",
                addr, value
            );
        }

        MemWrite::PassThrough
    }
}
