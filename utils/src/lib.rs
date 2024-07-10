#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::{vec, vec::Vec};

use rgy::{Hardware, Key, VRAM_HEIGHT, VRAM_WIDTH};

pub trait Loader {
    fn roms(&mut self) -> Vec<String>;

    fn load(&mut self, rom: &str) -> Vec<u8>;
}

struct List {
    roms: Vec<String>,
    base: u32,
    selected: u32,
    height: u32,
}

impl List {
    fn new(roms: Vec<String>) -> Self {
        Self {
            roms,
            base: 0,
            selected: 0,
            height: VRAM_HEIGHT as u32 / 10,
        }
    }

    fn down(&mut self) {
        let max = self.roms.len() as u32 - 1;
        self.selected = max.min(self.selected.saturating_add(1));
        self.base = self.base.max(self.selected.saturating_sub(self.height - 1));
    }

    fn up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
        self.base = self.base.min(self.selected);
    }

    fn selected(&self) -> &str {
        &self.roms[self.selected as usize]
    }

    fn draw(&mut self, h: &mut dyn Hardware, d: &mut Display) {
        for i in 0..self.height {
            let index = i + self.base;

            if let Some(rom) = self.roms.get(index as usize) {
                let msg = format!("{}{}", if index == self.selected { ">" } else { " " }, rom);
                d.print(2, i * 10 + 2, &msg);
            }
        }

        d.fill(h);
    }
}

const FG: u32 = 0x555555;
const BG: u32 = 0xdddddd;

struct Display {
    vram: Vec<u32>,
}

impl Display {
    fn new() -> Self {
        Self {
            vram: vec![BG; VRAM_WIDTH * VRAM_HEIGHT],
        }
    }

    fn print(&mut self, x: u32, y: u32, msg: &str) {
        use font8x8::legacy::BASIC_LEGACY;

        let mut cxoff = 0;

        for yoff in 0..8 {
            for xoff in 0..VRAM_WIDTH {
                let index = (x + xoff as u32) + (y + yoff as u32) * (VRAM_WIDTH as u32);
                if let Some(p) = self.vram.get_mut(index as usize) {
                    *p = BG;
                }
            }
        }

        for c in msg.chars() {
            if let Some(glyph) = BASIC_LEGACY.get(c as usize) {
                for (yoff, g) in glyph.iter().enumerate() {
                    let yoff = yoff as u32;
                    for xoff in 0..8 {
                        let index = (x + cxoff + xoff) + (y + yoff) * (VRAM_WIDTH as u32);
                        match *g & 1 << xoff {
                            0 => self.vram.get_mut(index as usize).map(|p| *p = BG),
                            _ => self.vram.get_mut(index as usize).map(|p| *p = FG),
                        };
                    }
                }
            }
            cxoff += 8;

            if cxoff >= VRAM_WIDTH as u32 - 8 {
                break;
            }
        }
    }

    fn fill(&mut self, h: &mut dyn Hardware) {
        for y in 0..VRAM_HEIGHT {
            let b = y * VRAM_WIDTH;
            let e = b + VRAM_WIDTH;
            h.vram_update(y, &self.vram[b..e]);
        }
    }
}

struct KeyState {
    on: bool,
    time: u64,
    key: Key,
}

impl KeyState {
    fn new(key: Key) -> Self {
        Self {
            on: false,
            time: 0,
            key,
        }
    }

    fn pressed(&mut self, h: &mut dyn Hardware) -> bool {
        let on = h.joypad_pressed(self.key.clone());
        let now = h.clock();

        if !self.on && on {
            self.on = true;
            self.time = now;
            false
        } else if self.on && !on {
            self.on = false;
            true
        } else if self.on {
            if now.wrapping_sub(self.time) > 100_000 {
                self.time = now;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

pub fn select<L: Loader, H: Hardware>(loader: &mut L, mut hardware: H) -> (Vec<u8>, H) {
    let mut list = List::new(loader.roms());

    let mut display = Display::new();
    let mut up = KeyState::new(Key::Up);
    let mut down = KeyState::new(Key::Down);
    let mut start = KeyState::new(Key::Start);

    while hardware.sched() {
        list.draw(&mut hardware, &mut display);

        if up.pressed(&mut hardware) {
            list.up();
        }
        if down.pressed(&mut hardware) {
            list.down();
        }
        if start.pressed(&mut hardware) {
            break;
        }
    }

    (loader.load(list.selected()), hardware)
}
