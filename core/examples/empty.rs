use rgy::{Config, Key, Stream, VRAM_HEIGHT, VRAM_WIDTH};

struct Hardware {
    display: Vec<Vec<u32>>,
}

impl Hardware {
    fn new() -> Self {
        // Create a frame buffer with the size VRAM_WIDTH * VRAM_HEIGHT.
        let display = vec![vec![0u32; VRAM_HEIGHT]; VRAM_WIDTH];

        Self { display }
    }
}

impl rgy::Hardware for Hardware {
    fn vram_update(&mut self, line: usize, buffer: &[u32]) {
        // `line` corresponds to the y coordinate.
        let y = line;

        for (x, col) in buffer.iter().enumerate() {
            self.display[x][y] = *col;
        }
    }

    fn joypad_pressed(&mut self, key: Key) -> bool {
        // Read a keyboard device and check if the `key` is pressed or not.
        println!("Check if {:?} is pressed", key);
        false
    }

    fn sound_play(&mut self, _stream: Box<dyn Stream>) {
        // Play the wave provided `Stream`.
    }

    fn clock(&mut self) -> u64 {
        // Return the epoch in microseconds.
        let epoch = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Couldn't get epoch");
        epoch.as_micros() as u64
    }

    fn send_byte(&mut self, _b: u8) {
        // Send a byte to a serial port.
    }

    fn recv_byte(&mut self) -> Option<u8> {
        // Try to read a byte from a serial port.
        None
    }

    fn sched(&mut self) -> bool {
        // `true` to continue, `false` to stop the emulator.
        println!("It's running!");
        true
    }

    fn load_ram(&mut self, size: usize) -> Vec<u8> {
        // Return save data.
        vec![0; size]
    }

    fn save_ram(&mut self, _ram: &[u8]) {
        // Store save data.
    }
}

fn main() {
    // Create the default config.
    let cfg = Config::new();

    // Create the hardware instance.
    let hw = Hardware::new();

    // The content of a ROM file, which can be downloaded from the Internet.
    let rom = vec![0u8; 1024];

    // Run the emulator.
    rgy::run(cfg, &rom, hw);
}
