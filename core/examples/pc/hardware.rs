use cpal;
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

use rgy::hardware::{self, Key, Stream, StreamId, VRAM_HEIGHT, VRAM_WIDTH};

#[derive(Clone)]
pub struct Hardware {
    rampath: Option<String>,
    vram: Arc<Mutex<Vec<u32>>>,
    pcms: Vec<SpeakerHandle>,
    keystate: Arc<Mutex<HashMap<Key, bool>>>,
    escape: Arc<AtomicBool>,
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
    ) -> Self {
        let window = match Window::new(
            "Gay Boy",
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
                    minifb::Key::Z => Key::A,
                    minifb::Key::X => Key::B,
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
    pub fn new(rampath: Option<String>) -> Self {
        let vram = Arc::new(Mutex::new(vec![0; VRAM_WIDTH * VRAM_HEIGHT]));

        let pcms = (0..4)
            .map(|_| {
                let pcm = Pcm::new();
                let handle = pcm.handle();

                pcm.run_forever();

                handle
            })
            .collect();

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

        Self {
            rampath,
            vram,
            pcms,
            keystate,
            escape,
        }
    }

    pub fn run(self) {
        let bg = Gui::new(
            self.vram.clone(),
            self.keystate.clone(),
            self.escape.clone(),
        );
        bg.run();
    }
}

impl hardware::Hardware for Hardware {
    fn vram_update(&mut self, line: usize, buf: &[u32]) {
        let mut vram = self.vram.lock().unwrap();
        for i in 0..buf.len() {
            let base = line * VRAM_WIDTH;
            vram[base + i] = buf[i];
        }
    }

    fn joypad_pressed(&mut self, key: Key) -> bool {
        *self
            .keystate
            .lock()
            .unwrap()
            .get(&key)
            .expect("Logic error in keystate map")
    }

    fn sound_play(&mut self, id: StreamId, stream: Box<dyn Stream>) {
        match id {
            StreamId::Tone1 => self.pcms[0].play(stream),
            StreamId::Tone2 => self.pcms[1].play(stream),
            StreamId::Wave => self.pcms[2].play(stream),
            StreamId::Noise => self.pcms[3].play(stream),
        }
    }

    fn sound_stop(&mut self, id: StreamId) {
        match id {
            StreamId::Tone1 => self.pcms[0].stop(),
            StreamId::Tone2 => self.pcms[1].stop(),
            StreamId::Wave => self.pcms[2].stop(),
            StreamId::Noise => self.pcms[3].stop(),
        }
    }

    fn send_byte(&mut self, b: u8) {
        info!("Send byte: {:02x}", b);
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
                    buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer),
                } => unimplemented!(),
                cpal::StreamData::Output {
                    buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer),
                } => unimplemented!(),
                cpal::StreamData::Output {
                    buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
                } => {
                    for sample in buffer.chunks_mut(format.channels as usize) {
                        let value = match &mut stream {
                            Some(s) => ((s.next(sample_rate) * 100 / s.max()) as f32 / 100.0),
                            None => 0.0,
                        };

                        for out in sample.iter_mut() {
                            *out = value;
                        }
                    }
                }
                _ => (),
            }
        });
    }
}

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

    fn stop(&self) {
        let _ = self.tx.send(SpeakerCmd::Stop);
    }
}
