use minifb::{Key, Scale, Window, WindowOptions};
use crate::gpu::Screen;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

enum Update {
    All(Vec<u32>),
    Line(usize, Vec<u32>),
}

#[derive(Clone)]
pub struct Handle {
    tx: Sender<Update>,
}

impl Screen for Handle {
    fn update(&self, buffer: &[u32]) {
        assert_eq!(buffer.len(), WIDTH * HEIGHT);
        let _ = self.tx.send(Update::All(buffer.into()));
    }

    fn update_line(&self, line: usize, buffer: &[u32]) {
        assert_eq!(buffer.len(), WIDTH);
        let _ = self.tx.send(Update::Line(line, buffer.into()));
    }
}

pub struct Lcd {
    vram: Vec<u32>,
    tx: Sender<Update>,
    rx: Receiver<Update>,
}

impl Lcd {
    pub fn new() -> Lcd {
        let (tx, rx) = mpsc::channel();

        Lcd {
            vram: vec![0; WIDTH * HEIGHT],
            tx,
            rx,
        }
    }

    pub fn handle(&self) -> Handle {
        Handle {
            tx: self.tx.clone(),
        }
    }

    pub fn run(&mut self) {
        let mut window = match Window::new(
            "Noise Test - Press ESC to exit",
            WIDTH,
            HEIGHT,
            WindowOptions {
                resize: true,
                scale: Scale::X4,
                ..WindowOptions::default()
            },
        ) {
            Ok(win) => win,
            Err(err) => {
                panic!("Unable to create window {}", err);
            }
        };

        // Initial update
        window.update_with_buffer(&self.vram).unwrap();

        // Update
        while window.is_open() && !window.is_key_down(Key::Escape) {
            match self.rx.try_recv() {
                Ok(update) => match update {
                    Update::All(buf) => {
                        self.vram = buf;
                    }
                    Update::Line(line, buf) => for i in 0..buf.len() {
                        let base = line * WIDTH;
                        self.vram[base + i] = buf[i];
                    },
                },
                Err(_) => {}
            }

            window.get_keys().map(|keys| {
                for t in keys {
                    match t {
                        Key::W => println!("holding w!"),
                        Key::T => println!("holding t!"),
                        _ => (),
                    }
                }
            });

            window.update_with_buffer(&self.vram).unwrap();
        }
    }
}
