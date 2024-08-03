use std::{
    io::Write,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use rgy::{VRAM_HEIGHT, VRAM_WIDTH};

const WHITE: u32 = 0xdddddd;
const BLACK: u32 = 0x555555;

const PRINT_DISPLAY: &'static str = "PRINT_DISPLAY";

fn print_display() -> bool {
    std::env::var(PRINT_DISPLAY).as_deref() == Ok("1")
}

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
                '.' => Some(WHITE),
                '#' => Some(BLACK),
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

        // Simplify to 2 tones as it suffices for the current test roms.
        let buffer: Vec<_> = buffer
            .iter()
            .map(|color| if *color == WHITE { WHITE } else { BLACK })
            .collect();

        self.display[ly * VRAM_WIDTH..(ly + 1) * VRAM_WIDTH].copy_from_slice(&buffer);

        if ly == VRAM_HEIGHT - 1 && self.display.as_slice() == expected.as_slice() {
            self.is_done = true;
        }

        if !print_display() {
            return;
        }

        // print display to console
        if ly == VRAM_HEIGHT - 1 {
            println!();
            for (index, color) in self.display.iter().enumerate() {
                if *color == WHITE {
                    print!(".")
                } else {
                    print!("#")
                }
                if index % VRAM_WIDTH == VRAM_WIDTH - 1 {
                    println!();
                }
            }
        }
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
    let timeout = Duration::from_secs(if print_display() { 10 } else { 60 });
    let now = Instant::now();
    while sys.poll() {
        if now.elapsed() >= timeout {
            panic!("timeout")
        }
    }
}

#[test]
fn cpu_instrs() {
    const EXPECTED: &str = "cpu_instrs\n\n01:ok  02:ok  03:ok  04:ok  05:ok  06:ok  07:ok  08:ok  09:ok  10:ok  11:ok  \n\nPassed all tests";
    test_rom(
        Expected::Serial(EXPECTED),
        "../roms/blargg/cpu_instrs/cpu_instrs.gb",
    );
}

#[test]
fn instr_timing() {
    const EXPECTED: &str = "instr_timing\n\n\nPassed";
    test_rom(
        Expected::Serial(EXPECTED),
        "../roms/blargg/instr_timing/instr_timing.gb",
    );
}

#[test]
fn mem_timing() {
    const EXPECTED: &str = "mem_timing\n\n01:ok  02:ok  03:ok  \n\nPassed all tests";
    test_rom(
        Expected::Serial(EXPECTED),
        "../roms/blargg/mem_timing/mem_timing.gb",
    );
}

#[test]
fn mem_timing2() {
    test_rom(
        Expected::from_file("tests/expects/mem_timing2.txt"),
        "../roms/blargg/mem_timing-2/mem_timing.gb",
    );
}

#[test]
fn halt_bug() {
    test_rom(
        Expected::from_file("tests/expects/halt_bug.txt"),
        "../roms/blargg/halt_bug.gb",
    );
}

#[test]
fn interrupt_time() {
    // The ROM test is supposed to fail in DMG mode as the CPU speed is fixed to 00 (no double-speed mode) and therefore the checksum never be correct.
    // The test compares with the expected display result in DMG mode that consumes 13 cycles when serial interrupt triggered manually.
    test_rom(
        Expected::from_file("tests/expects/interrupt_time.txt"),
        "../roms/blargg/interrupt_time/interrupt_time.gb",
    );
}

#[test]
fn dmg_sound_01_registers() {
    test_rom(
        Expected::from_file("tests/expects/dmg_sound_01_registers.txt"),
        "../roms/blargg/dmg_sound/rom_singles/01-registers.gb",
    );
}

#[test]
fn dmg_sound_02_len_ctr() {
    test_rom(
        Expected::from_file("tests/expects/dmg_sound_02_len_ctr.txt"),
        "../roms/blargg/dmg_sound/rom_singles/02-len ctr.gb",
    );
}

#[test]
fn dmg_sound_03_trigger() {
    test_rom(
        Expected::from_file("tests/expects/dmg_sound_03_trigger.txt"),
        "../roms/blargg/dmg_sound/rom_singles/03-trigger.gb",
    );
}

#[test]
fn dmg_sound_04_sweep() {
    test_rom(
        Expected::from_file("tests/expects/dmg_sound_04_sweep.txt"),
        "../roms/blargg/dmg_sound/rom_singles/04-sweep.gb",
    );
}

#[test]
fn dmg_sound_05_sweep_details() {
    test_rom(
        Expected::from_file("tests/expects/dmg_sound_05_sweep_details.txt"),
        "../roms/blargg/dmg_sound/rom_singles/05-sweep details.gb",
    );
}

#[test]
fn dmg_sound_06_overflow_on_trigger() {
    test_rom(
        Expected::from_file("tests/expects/dmg_sound_06_overflow_on_trigger.txt"),
        "../roms/blargg/dmg_sound/rom_singles/06-overflow on trigger.gb",
    );
}

#[test]
fn dmg_sound_07_len_sweep_period_sync() {
    test_rom(
        Expected::from_file("tests/expects/dmg_sound_07_len_sweep_period_sync.txt"),
        "../roms/blargg/dmg_sound/rom_singles/07-len sweep period sync.gb",
    );
}

#[test]
fn dmg_sound_08_len_ctr_during_power() {
    test_rom(
        Expected::from_file("tests/expects/dmg_sound_08_len_ctr_during_power.txt"),
        "../roms/blargg/dmg_sound/rom_singles/08-len ctr during power.gb",
    );
}

#[test]
fn dmg_sound_09_wave_read_while_on() {
    test_rom(
        Expected::from_file("tests/expects/dmg_sound_09_wave_read_while_on.txt"),
        "../roms/blargg/dmg_sound/rom_singles/09-wave read while on.gb",
    );
}

#[test]
fn dmg_sound_10_wave_trigger_while_on() {
    test_rom(
        Expected::from_file("tests/expects/dmg_sound_10_wave_trigger_while_on.txt"),
        "../roms/blargg/dmg_sound/rom_singles/10-wave trigger while on.gb",
    );
}

#[test]
fn dmg_sound_11_regs_after_power() {
    test_rom(
        Expected::from_file("tests/expects/dmg_sound_11_regs_after_power.txt"),
        "../roms/blargg/dmg_sound/rom_singles/11-regs after power.gb",
    );
}

#[test]
fn dmg_sound_12_wave_write_while_on() {
    test_rom(
        Expected::from_file("tests/expects/dmg_sound_12_wave_write_while_on.txt"),
        "../roms/blargg/dmg_sound/rom_singles/12-wave write while on.gb",
    );
}

#[test]
fn same_suite_div_write_trigger() {
    test_rom(
        Expected::from_file("tests/expects/same_suite_div_write_trigger.txt"),
        "../roms/same_suite/apu/div_write_trigger.gb",
    );
}

#[test]
fn same_suite_div_write_trigger_10() {
    test_rom(
        Expected::from_file("tests/expects/same_suite_div_write_trigger_10.txt"),
        "../roms/same_suite/apu/div_write_trigger_10.gb",
    );
}

#[test]
fn same_suite_div_write_trigger_volume() {
    test_rom(
        Expected::from_file("tests/expects/same_suite_div_write_trigger_volume.txt"),
        "../roms/same_suite/apu/div_write_trigger_volume.gb",
    );
}

#[test]
fn same_suite_div_write_trigger_volume_10() {
    test_rom(
        Expected::from_file("tests/expects/same_suite_div_write_trigger_volume_10.txt"),
        "../roms/same_suite/apu/div_write_trigger_volume_10.gb",
    );
}

#[test]
fn same_suite_div_trigger_volume_10() {
    test_rom(
        Expected::from_file("tests/expects/same_suite_div_trigger_volume_10.txt"),
        "../roms/same_suite/apu/div_trigger_volume_10.gb",
    );
}

#[test]
fn same_suite_channel_4_lfsr() {
    test_rom(
        Expected::from_file("tests/expects/same_suite_channel_4_lfsr.txt"),
        "../roms/same_suite/apu/channel_4/channel_4_lfsr.gb",
    );
}

#[test]
fn same_suite_channel_4_lfsr15() {
    test_rom(
        Expected::from_file("tests/expects/same_suite_channel_4_lfsr15.txt"),
        "../roms/same_suite/apu/channel_4/channel_4_lfsr15.gb",
    );
}

#[test]
fn same_suite_channel_4_lfsr_7_15() {
    test_rom(
        Expected::from_file("tests/expects/same_suite_channel_4_lfsr_7_15.txt"),
        "../roms/same_suite/apu/channel_4/channel_4_lfsr_7_15.gb",
    );
}

#[test]
fn same_suite_channel_4_lfsr_15_7() {
    test_rom(
        Expected::from_file("tests/expects/same_suite_channel_4_lfsr_15_7.txt"),
        "../roms/same_suite/apu/channel_4/channel_4_lfsr_15_7.gb",
    );
}

#[test]
fn same_suite_channel_1_delay() {
    test_rom(
        Expected::from_file("tests/expects/same_suite_channel_1_delay.txt"),
        "../roms/same_suite/apu/channel_1/channel_1_delay.gb",
    );
}

#[test]
fn same_suite_channel_1_duty_delay() {
    test_rom(
        Expected::from_file("tests/expects/same_suite_channel_1_duty_delay.txt"),
        "../roms/same_suite/apu/channel_1/channel_1_duty_delay.gb",
    );
}

#[test]
fn same_suite_channel_1_freq_change() {
    test_rom(
        Expected::from_file("tests/expects/same_suite_channel_1_freq_change.txt"),
        "../roms/same_suite/apu/channel_1/channel_1_freq_change.gb",
    );
}

#[test]
fn same_suite_channel_1_nrx2_speed_change() {
    test_rom(
        Expected::from_file("tests/expects/same_suite_channel_1_nrx2_speed_change.txt"),
        "../roms/same_suite/apu/channel_1/channel_1_nrx2_speed_change.gb",
    );
}

#[test]
fn same_suite_channel_1_restart() {
    test_rom(
        Expected::from_file("tests/expects/same_suite_channel_1_restart.txt"),
        "../roms/same_suite/apu/channel_1/channel_1_restart.gb",
    );
}

#[test]
fn same_suite_channel_1_restart_nrx2_glitch() {
    test_rom(
        Expected::from_file("tests/expects/same_suite_channel_1_restart_nrx2_glitch.txt"),
        "../roms/same_suite/apu/channel_1/channel_1_restart_nrx2_glitch.gb",
    );
}
