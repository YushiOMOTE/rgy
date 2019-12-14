# rgy

No-std cross-platform Rust GameBoy emulator library. Rust GameboY (RGY, or Real GaY).

[![Latest version](https://img.shields.io/crates/v/rgy.svg)](https://crates.io/crates/rgy)
[![Documentation](https://docs.rs/rgy/badge.svg)](https://docs.rs/rgy)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Actions Status](https://github.com/YushiOMOTE/rgy/workflows/Rust/badge.svg)](https://github.com/YushiOMOTE/rgy/actions)

<img src="https://raw.github.com/wiki/YushiOMOTE/gbr/media/demo.gif" width="450" />
<img src="https://raw.github.com/wiki/YushiOMOTE/gbr/media/demo_screens_2.jpg" width="450" />

### Usage

Once you implement OS-specific part, i.e. `Hardware` trait, you will get a GameBoy emulator for your environment.

```rust
struct Hardware;

// 1. Implement `rgy::Hardware`.
impl rgy::Hardware for Hardware {
    ...
}

// 2. Call `rgy::run`.
fn main() {
    let cfg = Config::new();
    let rom = include_bytes!("rom,gb");
    rgy::run(cfg, &rom, Hardware);
}
```

### Example

```
$ cargo run --example pc <a ROM file>
```

The example runs the GameBoy emulator in UNIX environment. It depends on `libasound2-dev` and `libxcursor-dev`.
The ROM files can be easily downloaded from the Internet.

### Projects

The following projects use this library to run a GameBoy emulator.

* [stickboy](https://github.com/yushiomote/stickboy) runs a GameBoy emulator on Macbook Pro (UEFI).
* [biboy](https://github.com/yushiomote/biboy) runs a GameBoy emulator on BIOS PC.
* [waboy](https://github.com/yushiomote/waboy) runs a GameBoy emulator on web browsers (wasm).
