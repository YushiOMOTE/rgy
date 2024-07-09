use std::{
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};

struct TestHardware {
    expected: &'static str,
    index: usize,
    is_done: bool,
}

impl TestHardware {
    fn new(expected: &'static str) -> Self {
        Self {
            expected,
            index: 0,
            is_done: false,
        }
    }
}

impl rgy::Hardware for TestHardware {
    fn vram_update(&mut self, _: usize, _: &[u32]) {}

    fn joypad_pressed(&mut self, _: rgy::Key) -> bool {
        false
    }

    fn sound_play(&mut self, _: Box<dyn rgy::Stream>) {}

    fn clock(&mut self) -> u64 {
        let epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        epoch.as_micros() as u64
    }

    fn send_byte(&mut self, b: u8) {
        if self.is_done {
            return;
        }
        print!("{}", b as char);
        std::io::stdout().flush().unwrap();
        assert_eq!(
            self.expected.as_bytes()[self.index] as char,
            b as char,
            "error at index {}, expected: {:?}",
            self.index,
            &self.expected[0..=self.index]
        );
        self.index += 1;
        if self.index == self.expected.len() {
            self.is_done = true;
        }
    }

    fn recv_byte(&mut self) -> Option<u8> {
        None
    }

    fn load_ram(&mut self, _: usize) -> Vec<u8> {
        Default::default()
    }

    fn save_ram(&mut self, _: &[u8]) {}

    fn sched(&mut self) -> bool {
        !self.is_done
    }
}

fn test_rom(expected: &'static str, path: &str) {
    let rom = std::fs::read(path).unwrap();
    let hw = TestHardware::new(expected);
    let mut sys = rgy::System::new(
        rgy::Config::new().native_speed(true),
        &rom,
        hw,
        rgy::debug::NullDebugger,
    );
    while sys.poll() {}
}

#[test]
fn cpu_instrs() {
    const EXPECTED: &str = "cpu_instrs\n\n01:ok  02:ok  03:ok  04:ok  05:ok  06:ok  07:ok  08:ok  09:ok  10:ok  11:ok  \n\nPassed all tests";
    test_rom(EXPECTED, "../roms/cpu_instrs/cpu_instrs.gb");
}

#[test]
fn instr_timing() {
    const EXPECTED: &str = "instr_timing\n\n\nPassed";
    test_rom(EXPECTED, "../roms/instr_timing/instr_timing.gb");
}

#[test]
fn mem_timing() {
    const EXPECTED: &str = "mem_timing\n\n01:ok  02:ok  03:ok  \n\nPassed all tests";
    test_rom(EXPECTED, "../roms/mem_timing/mem_timing.gb");
}
