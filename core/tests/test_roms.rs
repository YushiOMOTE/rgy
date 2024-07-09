use std::{
    io::Write,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use rgy::{VRAM_HEIGHT, VRAM_WIDTH};

enum Expected {
    Serial(&'static str),
    Display(Vec<u32>),
}

impl Expected {
    fn from_file(path: &str) -> Self {
        let display: Vec<u32> = std::fs::read_to_string(path)
            .unwrap()
            .chars()
            .filter_map(|c| match c {
                '.' => Some(0xdddddd),
                '#' => Some(0x555555),
                _ => None,
            })
            .collect();
        assert_eq!(VRAM_HEIGHT * VRAM_WIDTH, display.len());
        Self::Display(display)
    }
}

struct TestHardware {
    expected: Expected,
    index: usize,
    is_done: bool,
    display: [u32; VRAM_HEIGHT * VRAM_WIDTH],
}

impl TestHardware {
    fn new(expected: Expected) -> Self {
        Self {
            expected,
            index: 0,
            is_done: false,
            display: [0; VRAM_HEIGHT * VRAM_WIDTH],
        }
    }
}

impl rgy::Hardware for TestHardware {
    fn vram_update(&mut self, ly: usize, buffer: &[u32]) {
        let Expected::Display(expected) = &self.expected else {
            return;
        };
        self.display[ly * VRAM_WIDTH..(ly + 1) * VRAM_WIDTH].copy_from_slice(buffer);

        if ly == VRAM_HEIGHT - 1 && self.display.as_slice() == expected.as_slice() {
            self.is_done = true;
        }

        // // print display to console
        // if ly == VRAM_HEIGHT - 1 {
        //     for (index, color) in self.display.iter().enumerate() {
        //         if *color == 0xdddddd {
        //             print!(".")
        //         } else {
        //             print!("#")
        //         }
        //         if index % VRAM_WIDTH == VRAM_WIDTH - 1 {
        //             println!();
        //         }
        //     }
        // }
    }

    fn joypad_pressed(&mut self, _: rgy::Key) -> bool {
        false
    }

    fn sound_play(&mut self, _: Box<dyn rgy::Stream>) {}

    fn clock(&mut self) -> u64 {
        let epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        epoch.as_micros() as u64
    }

    fn send_byte(&mut self, b: u8) {
        let Expected::Serial(expected) = self.expected else {
            return;
        };
        if self.is_done {
            return;
        }
        print!("{}", b as char);
        std::io::stdout().flush().unwrap();
        assert_eq!(
            expected.as_bytes()[self.index] as char,
            b as char,
            "error at index {}, expected: {:?}",
            self.index,
            &expected[0..=self.index]
        );
        self.index += 1;
        if self.index == expected.len() {
            self.is_done = true;
        }
    }

    fn recv_byte(&mut self) -> Option<u8> {
        None
    }

    fn load_ram(&mut self, len: usize) -> Vec<u8> {
        vec![0; len]
    }

    fn save_ram(&mut self, _: &[u8]) {}

    fn sched(&mut self) -> bool {
        !self.is_done
    }
}

fn test_rom(expected: Expected, path: &str) {
    let rom = std::fs::read(path).unwrap();
    let hw = TestHardware::new(expected);
    let mut sys = rgy::System::new(
        rgy::Config::new().native_speed(true),
        &rom,
        hw,
        rgy::debug::NullDebugger,
    );
    const TIMEOUT: Duration = Duration::from_secs(60);
    let now = Instant::now();
    while sys.poll() {
        if now.elapsed() >= TIMEOUT {
            panic!("timeout")
        }
    }
}

#[test]
fn cpu_instrs() {
    const EXPECTED: &str = "cpu_instrs\n\n01:ok  02:ok  03:ok  04:ok  05:ok  06:ok  07:ok  08:ok  09:ok  10:ok  11:ok  \n\nPassed all tests";
    test_rom(
        Expected::Serial(EXPECTED),
        "../roms/cpu_instrs/cpu_instrs.gb",
    );
}

#[test]
fn instr_timing() {
    const EXPECTED: &str = "instr_timing\n\n\nPassed";
    test_rom(
        Expected::Serial(EXPECTED),
        "../roms/instr_timing/instr_timing.gb",
    );
}

#[test]
fn mem_timing() {
    const EXPECTED: &str = "mem_timing\n\n01:ok  02:ok  03:ok  \n\nPassed all tests";
    test_rom(
        Expected::Serial(EXPECTED),
        "../roms/mem_timing/mem_timing.gb",
    );
}

#[test]
fn halt_bug() {
    test_rom(
        Expected::from_file("tests/halt_bug_success_image.txt"),
        "../roms/halt_bug.gb",
    );
}
