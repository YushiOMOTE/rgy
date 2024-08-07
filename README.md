# rgy

No-std cross-platform Rust GameBoy emulator library. Rust GameboY (RGY, or Real GaY).

[![Latest version](https://img.shields.io/crates/v/rgy.svg)](https://crates.io/crates/rgy)
[![Documentation](https://docs.rs/rgy/badge.svg)](https://docs.rs/rgy)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Actions Status](https://github.com/YushiOMOTE/rgy/workflows/main/badge.svg?branch=master)](https://github.com/YushiOMOTE/rgy/actions)

<img src="https://raw.github.com/wiki/YushiOMOTE/gbr/media/demo.gif" width="450" />
<img src="https://raw.github.com/wiki/YushiOMOTE/gbr/media/demo_screens_2.jpg" width="450" />

## Try it on your PC

```
$ cargo run --example pc <a ROM file>
```

The example runs the GameBoy emulator in Mac/Linux/Windows.

### Dependencies

On Ubuntu, you need to install these packages:

```
$ sudo apt install libasound2-dev libxcursor-dev libudev-dev
```

### Key bindings

| Keyboard  | Gamepad      | GameBoy |
|-----------|--------------|---------|
| Up / W    | DPad Up      | Up      |
| Left / A  | DPad Left    | Left    |
| Down / S  | DPad Down    | Down    |
| Right / D | DPad Right   | Right   |
| K / X     | South / East | A       |
| J / Z     | West / North | B       |
| Space     | Select       | Select  |
| Enter     | Start        | Start   |
| Escape    | -            | Close   |

## Port it to a new environment

The library itself is environment independent. It can be even ported onto bare-metal. Once you implement environment-specific part, i.e. `Hardware` trait, you will get a GameBoy emulator for your environment.

```rust
struct Hardware;

// 1. Implement `rgy::Hardware`.
impl rgy::Hardware for Hardware {
    ...
}

// 2. Call `rgy::run`.
fn main() {
    let cfg = Config::new();
    let rom = include_bytes!("path_to_rom_file.gb");
    rgy::run(cfg, &rom, Hardware);
}
```

## Emulation Status

* CPU
    * Supports all the documented instructions.
    * Emulates accurate read/write timing.
* Interrupts
    * Supports all the interrupts.
    * Emulates halt bug.
* Graphics
    * The most features are functioning.
    * OAM bug is not yet supported.
* Sound
    * The most features are functioning.
    * PCM registers are always emulated for sound tests.
* Joypad
* Timer
* Serial
* Cartridge (MBC 1,2,3,5, HuC 1)
* Gameboy Color feature is under development.

## Test Status

Test status of [Blargg's Gameboy hardware test ROMs](https://github.com/retrio/gb-test-roms/tree/c240dd7d700e5c0b00a7bbba52b53e4ee67b5f15)

* [x] `cpu_instrs`
* [x] `instr_timing`
* [x] `mem_timing`
* [x] `mem_timing-2`
* [ ] `oam_bug`
* [x] `interrupt_time`
* [x] `dmg_sound`
    * [x] `01-registers`
    * [x] `02-len ctr`
    * [x] `03-trigger`
    * [x] `04-sweep`
    * [x] `05-sweep-details`
    * [x] `06-overflow on trigger`
    * [x] `07-len sweep period sync`
    * [x] `08-len ctr during power`
    * [x] `09-wave read while on`
    * [x] `10-wave trigger while on`
    * [x] `11-regs after power`
    * [x] `12-wave write while on`
* [ ] `cgb_sound`

Test status of [Same Suite](https://github.com/YushiOMOTE/SameSuite/tree/430ab7f68fc612e005ed5586990dfec0ea7a9ce5)

* APU
    * [x] `apu/div_write_trigger.gb`
    * [x] `apu/div_write_trigger_10.gb`
    * [x] `apu/div_write_trigger_volume.gb`
    * [x] `apu/div_write_trigger_volume_10.gb`
    * [x] `apu/div_trigger_volume_10.gb`
    * Channel 1
        * [x] `apu/channel_1/channel_1_delay.gb`
        * [x] `apu/channel_1/channel_1_duty_delay.gb`
        * [x] `apu/channel_1/channel_1_freq_change.gb`
        * [ ] `apu/channel_1/channel_1_align.gb` (CGB double speed)
        * [ ] `apu/channel_1/channel_1_align_cpu.gb` (CGB double speed)
        * [ ] `apu/channel_1/channel_1_duty.gb` (CGB double speed)
        * [ ] `apu/channel_1/channel_1_extra_length_clocking-cgb0B.gb` (CGB double speed)
        * [ ] `apu/channel_1/channel_1_freq_change_timing-A.gb` (CGB double speed)
        * [ ] `apu/channel_1/channel_1_freq_change_timing-cgb0BC.gb` (CGB double speed)
        * [ ] `apu/channel_1/channel_1_freq_change_timing-cgbDE.gb` (CGB double speed)
        * [ ] `apu/channel_1/channel_1_nrx2_glitch.gb`
        * [x] `apu/channel_1/channel_1_nrx2_speed_change.gb`
        * [x] `apu/channel_1/channel_1_restart.gb`
        * [x] `apu/channel_1/channel_1_restart_nrx2_glitch.gb`
        * [ ] `apu/channel_1/channel_1_stop_div.gb`
        * [ ] `apu/channel_1/channel_1_stop_restart.gb`
        * [ ] `apu/channel_1/channel_1_sweep.gb`
        * [ ] `apu/channel_1/channel_1_sweep_restart.gb`
        * [ ] `apu/channel_1/channel_1_sweep_restart_2.gb`
        * [ ] `apu/channel_1/channel_1_volume.gb`
        * [ ] `apu/channel_1/channel_1_volume_div.gb`
    * Channel 4
        * [x] `apu/channel_4/channel_4_lfsr.gb`
        * [x] `apu/channel_4/channel_4_lfsr15.gb`
        * [x] `apu/channel_4/channel_4_lfsr_7_15.gb`
        * [x] `apu/channel_4/channel_4_lfsr_15_7.gb`

## Projects

The following projects use this library to run a GameBoy emulator.

* [stickboy](https://github.com/yushiomote/stickboy) runs a GameBoy emulator on Macbook Pro (UEFI).
* [biboy](https://github.com/yushiomote/biboy) runs a GameBoy emulator on BIOS PC.
* [waboy](https://github.com/yushiomote/waboy) runs a GameBoy emulator on web browsers (wasm).
