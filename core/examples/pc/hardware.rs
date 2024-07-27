use gilrs::{Button, GamepadId, Gilrs};
use log::*;
use minifb::{Scale, Window, WindowOptions};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rgy::{Key, Stream, VRAM_HEIGHT, VRAM_WIDTH};

#[derive(Clone)]
pub struct Hardware {
    rampath: Option<String>,
    vram: Arc<Mutex<Vec<u32>>>,
    pcm: SpeakerHandle,
    keystate: Arc<Mutex<HashMap<Key, bool>>>,
    escape: Arc<AtomicBool>,
    color: bool,
    gamepad: Arc<Mutex<Gilrs>>,
    gamepad_id: Option<GamepadId>,
}

struct Gui {
    window: Window,
    vram: Arc<Mutex<Vec<u32>>>,
    keystate: Arc<Mutex<HashMap<Key, bool>>>,
    escape: Arc<AtomicBool>,
}

impl Gui {
    fn new(
        vram: Arc<Mutex<Vec<u32>>>,
        keystate: Arc<Mutex<HashMap<Key, bool>>>,
        escape: Arc<AtomicBool>,
        color: bool,
    ) -> Self {
        let title = if color { "Gay Boy Color" } else { "Gay Boy" };
        let window = match Window::new(
            title,
            VRAM_WIDTH,
            VRAM_HEIGHT,
            WindowOptions {
                resize: false,
                scale: Scale::X4,
                ..WindowOptions::default()
            },
        ) {
            Ok(win) => win,
            Err(err) => {
                panic!("Unable to create window {}", err);
            }
        };

        Self {
            window,
            vram,
            keystate,
            escape,
        }
    }

    fn run(mut self) {
        while !self.escape.load(Ordering::Relaxed) {
            std::thread::sleep(Duration::from_millis(10));
            self.vramupdate();
            self.keyupdate();
        }
    }

    fn vramupdate(&mut self) {
        let vram = self.vram.lock().unwrap().clone();
        self.window.update_with_buffer(&vram).unwrap();
    }

    fn keyupdate(&mut self) {
        if !self.window.is_open() {
            self.escape.store(true, Ordering::Relaxed);
        }

        for (_, v) in self.keystate.lock().unwrap().iter_mut() {
            *v = false;
        }

        if let Some(keys) = self.window.get_keys() {
            for k in keys {
                let gbk = match k {
                    minifb::Key::Right => Key::Right,
                    minifb::Key::Left => Key::Left,
                    minifb::Key::Up => Key::Up,
                    minifb::Key::Down => Key::Down,
                    minifb::Key::D => Key::Right,
                    minifb::Key::A => Key::Left,
                    minifb::Key::W => Key::Up,
                    minifb::Key::S => Key::Down,
                    minifb::Key::Z => Key::B,
                    minifb::Key::X => Key::A,
                    minifb::Key::J => Key::B,
                    minifb::Key::K => Key::A,
                    minifb::Key::Space => Key::Select,
                    minifb::Key::Enter => Key::Start,
                    minifb::Key::Escape => {
                        self.escape.store(true, Ordering::Relaxed);
                        return;
                    }
                    _ => continue,
                };

                match self.keystate.lock().unwrap().get_mut(&gbk) {
                    Some(v) => *v = true,
                    None => unreachable!(),
                }
            }
        }
    }
}

impl Hardware {
    pub fn new(rampath: Option<String>, color: bool) -> Self {
        let vram = Arc::new(Mutex::new(vec![0; VRAM_WIDTH * VRAM_HEIGHT]));

        let pcm = Pcm::new();
        let handle = pcm.handle();
        pcm.run_forever();

        let mut keystate = HashMap::new();
        keystate.insert(Key::Right, false);
        keystate.insert(Key::Left, false);
        keystate.insert(Key::Up, false);
        keystate.insert(Key::Down, false);
        keystate.insert(Key::A, false);
        keystate.insert(Key::B, false);
        keystate.insert(Key::Select, false);
        keystate.insert(Key::Start, false);
        let keystate = Arc::new(Mutex::new(keystate));

        let escape = Arc::new(AtomicBool::new(false));

        let gamepad = Arc::new(Mutex::new(Gilrs::new().unwrap()));

        Self {
            color,
            rampath,
            vram,
            pcm: handle,
            keystate,
            escape,
            gamepad,
            gamepad_id: None,
        }
    }

    pub fn run(self) {
        let bg = Gui::new(
            self.vram.clone(),
            self.keystate.clone(),
            self.escape.clone(),
            self.color,
        );
        bg.run();
    }
}

impl rgy::Hardware for Hardware {
    fn vram_update(&mut self, line: usize, buf: &[u32]) {
        let mut vram = self.vram.lock().unwrap();
        for i in 0..buf.len() {
            let base = line * VRAM_WIDTH;
            vram[base + i] = buf[i];
        }
    }

    fn joypad_pressed(&mut self, key: Key) -> bool {
        let keyboard_pressed = *self
            .keystate
            .lock()
            .unwrap()
            .get(&key)
            .expect("Logic error in keystate map");

        let gamepad = self.gamepad.lock().unwrap();
        let gamepad_pressed = match self.gamepad_id.map(|id| gamepad.gamepad(id)) {
            Some(g) => match key {
                Key::Right => g.is_pressed(Button::DPadRight),
                Key::Left => g.is_pressed(Button::DPadLeft),
                Key::Up => g.is_pressed(Button::DPadUp),
                Key::Down => g.is_pressed(Button::DPadDown),
                Key::A => g.is_pressed(Button::North) | g.is_pressed(Button::East),
                Key::B => g.is_pressed(Button::South) | g.is_pressed(Button::West),
                Key::Select => g.is_pressed(Button::Select),
                Key::Start => g.is_pressed(Button::Start),
            },
            None => false,
        };

        keyboard_pressed || gamepad_pressed
    }

    fn sound_play(&mut self, stream: Box<dyn Stream>) {
        self.pcm.play(stream)
    }

    fn send_byte(&mut self, b: u8) {
        print!("{}", b as char);
        std::io::stdout().flush().unwrap();
    }

    fn recv_byte(&mut self) -> Option<u8> {
        None
    }

    fn clock(&mut self) -> u64 {
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Couldn't get epoch");
        epoch.as_micros() as u64
    }

    fn load_ram(&mut self, size: usize) -> Vec<u8> {
        let mut ram = vec![0; size];

        match &self.rampath {
            Some(path) => match File::open(path) {
                Ok(mut fs) => {
                    fs.read_exact(&mut ram).expect("Couldn't read file");
                    ram
                }
                Err(e) => {
                    warn!("Couldn't open RAM file `{}`: {}", path, e);
                    ram
                }
            },
            None => ram,
        }
    }

    fn save_ram(&mut self, ram: &[u8]) {
        match &self.rampath {
            Some(path) => {
                let mut fs = File::create(path).expect("Couldn't open file");
                fs.write_all(ram).expect("Couldn't write file");
            }
            None => {}
        }
    }

    fn sched(&mut self) -> bool {
        while let Some(e) = self.gamepad.lock().unwrap().next_event() {
            self.gamepad_id = Some(e.id);
        }

        !self.escape.load(Ordering::Relaxed)
    }
}

pub struct Pcm {
    tx: Sender<SpeakerCmd>,
    rx: Receiver<SpeakerCmd>,
}

impl Pcm {
    pub fn new() -> Pcm {
        let (tx, rx) = mpsc::channel();

        Pcm { tx, rx }
    }

    pub fn handle(&self) -> SpeakerHandle {
        SpeakerHandle {
            tx: self.tx.clone(),
        }
    }

    pub fn run_forever(self) {
        std::thread::spawn(move || {
            self.run();
        });
    }

    pub fn run(self) {
        let device = cpal::default_output_device().expect("Failed to get default output device");
        let format = device
            .default_output_format()
            .expect("Failed to get default output format");
        let sample_rate = format.sample_rate.0;
        let event_loop = cpal::EventLoop::new();
        let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
        event_loop.play_stream(stream_id.clone());

        let mut stream = None;

        event_loop.run(move |_, data| {
            match self.rx.try_recv() {
                Ok(SpeakerCmd::Play(s)) => {
                    stream = Some(s);
                }
                Ok(SpeakerCmd::Stop) => {
                    stream = None;
                }
                Err(_) => {}
            }

            match data {
                cpal::StreamData::Output {
                    buffer: cpal::UnknownTypeOutputBuffer::U16(_buffer),
                } => unimplemented!(),
                cpal::StreamData::Output {
                    buffer: cpal::UnknownTypeOutputBuffer::I16(_buffer),
                } => unimplemented!(),
                cpal::StreamData::Output {
                    buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
                } => {
                    for sample in buffer.chunks_mut(format.channels as usize) {
                        let value = match &mut stream {
                            Some(s) => {
                                let (l, r) = s.next_dual(sample_rate);
                                (adjust(l, s.max()), adjust(r, s.max()))
                            }
                            None => (0.0, 0.0),
                        };

                        sample[0] = value.0;
                        sample[1] = value.1;
                    }
                }
                _ => (),
            }
        });
    }
}

fn adjust(value: u16, max: u16) -> f32 {
    (value as u64 * 100 / max as u64) as f32 / 100.0
}

#[allow(unused)]
enum SpeakerCmd {
    Play(Box<dyn Stream>),
    Stop,
}

#[derive(Clone)]
pub struct SpeakerHandle {
    tx: Sender<SpeakerCmd>,
}

impl SpeakerHandle {
    fn play(&self, stream: Box<dyn Stream>) {
        let _ = self.tx.send(SpeakerCmd::Play(stream));
    }

    #[allow(unused)]
    fn stop(&self) {
        let _ = self.tx.send(SpeakerCmd::Stop);
    }
}
