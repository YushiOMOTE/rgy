use minifb::{Key, Scale, Window, WindowOptions};
use crate::gpu::Screen;
use crate::sound::{Speaker, Stream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;
use cpal;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

enum Update {
    All(Vec<u32>),
    Line(usize, Vec<u32>),
}

#[derive(Clone)]
pub struct ScreenHandle {
    tx: Sender<Update>,
}

impl Screen for ScreenHandle {
    fn width(&self) -> usize {
        WIDTH
    }

    fn height(&self) -> usize {
        HEIGHT
    }

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

    pub fn handle(&self) -> ScreenHandle {
        ScreenHandle {
            tx: self.tx.clone(),
        }
    }

    pub fn run(&mut self) {
        let mut window = match Window::new(
            "Gay Boy",
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

        let fint = Duration::from_millis(1000 / 60);

        // Update
        while window.is_open() && !window.is_key_down(Key::Escape) {
            let last = std::time::Instant::now();

            while std::time::Instant::now() - last < fint {
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
                } => for sample in buffer.chunks_mut(format.channels as usize) {
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
                },
                cpal::StreamData::Output {
                    buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer),
                } => for sample in buffer.chunks_mut(format.channels as usize) {
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
                },
                cpal::StreamData::Output {
                    buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
                } => for sample in buffer.chunks_mut(format.channels as usize) {
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
                },
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

impl Speaker for SpeakerHandle {
    fn play(&self, stream: Box<Stream>) {
        let _ = self.tx.send(SpeakerCmd::Play(stream));
    }

    fn stop(&self) {
        let _ = self.tx.send(SpeakerCmd::Stop);
    }
}
