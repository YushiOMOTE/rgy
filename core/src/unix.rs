use cpal;
use minifb::{Scale, Window, WindowOptions};
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Instant;

use crate::hardware::{Hardware, Key, SoundId, Stream, VRAM_HEIGHT, VRAM_WIDTH};

pub struct HardwareImpl {
    vram: Vec<u32>,
    window: Window,
    pcms: Vec<SpeakerHandle>,
    inst: Instant,
    vramupdated: u64,
    keypolled: u64,
    keystate: HashMap<Key, bool>,
}

impl HardwareImpl {
    pub fn new() -> Self {
        let vram = vec![0; VRAM_WIDTH * VRAM_HEIGHT];

        let mut window = match Window::new(
            "Gay Boy",
            VRAM_WIDTH,
            VRAM_HEIGHT,
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
        window.update_with_buffer(&vram).unwrap();

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

        Self {
            vram,
            window,
            pcms,
            inst: Instant::now(),
            vramupdated: 0,
            keypolled: 0,
            keystate,
        }
    }
}

impl Hardware for HardwareImpl {
    fn vram_height(&self) -> usize {
        VRAM_HEIGHT
    }

    fn vram_width(&self) -> usize {
        VRAM_WIDTH
    }

    fn vram_update(&mut self, line: usize, buf: &[u32]) {
        for i in 0..buf.len() {
            let base = line * self.vram_width();
            self.vram[base + i] = buf[i];
        }

        let now = self.clock();
        if self.vramupdated == 0 || now.wrapping_sub(self.vramupdated) >= 10_000 {
            self.vramupdated = now;
            self.window.update_with_buffer(&self.vram).unwrap();
        }
    }

    fn joypad_pressed(&mut self, key: Key) -> bool {
        *self
            .keystate
            .get(&key)
            .expect("Logic error in keystate map")
    }

    fn sound_play(&mut self, id: SoundId, stream: Box<Stream>) {
        match id {
            SoundId::Tone1 => self.pcms[0].play(stream),
            SoundId::Tone2 => self.pcms[1].play(stream),
            SoundId::Wave => self.pcms[2].play(stream),
            SoundId::Noise => self.pcms[3].play(stream),
        }
    }

    fn sound_stop(&mut self, id: SoundId) {
        match id {
            SoundId::Tone1 => self.pcms[0].stop(),
            SoundId::Tone2 => self.pcms[1].stop(),
            SoundId::Wave => self.pcms[2].stop(),
            SoundId::Noise => self.pcms[3].stop(),
        }
    }

    fn clock(&mut self) -> u64 {
        let d = self.inst.elapsed();
        d.as_secs()
            .wrapping_mul(1000_000)
            .wrapping_add(d.subsec_micros().into())
    }

    fn sched(&mut self) -> bool {
        let now = self.clock();

        if !self.window.is_open() {
            return false;
        }

        if self.keypolled == 0 || now.wrapping_sub(self.keypolled) >= 20_000 {
            self.keypolled = now;

            match self.window.get_keys() {
                Some(keys) => {
                    for (_, v) in self.keystate.iter_mut() {
                        *v = false;
                    }
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
                            minifb::Key::Escape => return false,
                            _ => continue,
                        };

                        match self.keystate.get_mut(&gbk) {
                            Some(v) => *v = true,
                            None => unreachable!(),
                        }
                    }
                }
                None => {
                    for (_, v) in self.keystate.iter_mut() {
                        *v = false;
                    }
                }
            }
        }

        true
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
        let sample_rate = format.sample_rate.0 as f32;
        let event_loop = cpal::EventLoop::new();
        let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
        event_loop.play_stream(stream_id.clone());

        let mut stream = None;

        event_loop.run(move |_, data| {
            let cmd = match self.rx.try_recv() {
                Ok(SpeakerCmd::Play(s)) => {
                    stream = Some(s);
                }
                Ok(SpeakerCmd::Stop) => {
                    stream = None;
                }
                Err(_) => {}
            };

            match data {
                cpal::StreamData::Output {
                    buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer),
                } => {
                    for sample in buffer.chunks_mut(format.channels as usize) {
                        let value = match &mut stream {
                            Some(s) => match s(sample_rate) {
                                Some(ss) => ((ss * 0.5 + 0.5) * std::u16::MAX as f32) as u16,
                                None => u16::max_value() / 2,
                            },
                            None => u16::max_value() / 2,
                        };

                        for out in sample.iter_mut() {
                            *out = value;
                        }
                    }
                }
                cpal::StreamData::Output {
                    buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer),
                } => {
                    for sample in buffer.chunks_mut(format.channels as usize) {
                        let value = match &mut stream {
                            Some(s) => match s(sample_rate) {
                                Some(ss) => (ss * std::i16::MAX as f32) as i16,
                                None => 0,
                            },
                            None => 0,
                        };

                        for out in sample.iter_mut() {
                            *out = value;
                        }
                    }
                }
                cpal::StreamData::Output {
                    buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
                } => {
                    for sample in buffer.chunks_mut(format.channels as usize) {
                        let value = match &mut stream {
                            Some(s) => match s(sample_rate) {
                                Some(ss) => ss,
                                None => 0.0,
                            },
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
    Play(Box<Stream>),
    Stop,
}

#[derive(Clone)]
pub struct SpeakerHandle {
    tx: Sender<SpeakerCmd>,
}

impl SpeakerHandle {
    fn play(&self, stream: Box<Stream>) {
        let _ = self.tx.send(SpeakerCmd::Play(stream));
    }

    fn stop(&self) {
        let _ = self.tx.send(SpeakerCmd::Stop);
    }
}
