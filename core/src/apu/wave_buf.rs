#[derive(Debug, Clone)]
pub struct WaveRam {
    pub ram: [u8; 16],
}

impl WaveRam {
    pub fn new() -> Self {
        Self { ram: [0; 16] }
    }

    pub fn read_byte(&self, offset: u16) -> u8 {
        self.ram[offset as usize]
    }

    pub fn write_byte(&mut self, offset: u16, value: u8) {
        self.ram[offset as usize] = value;
    }

    pub fn read_waveform(&self, index: usize) -> u8 {
        if index % 2 == 0 {
            self.ram[index / 2] >> 4
        } else {
            self.ram[index / 2] & 0xf
        }
    }

    pub fn waveform_length(&self) -> usize {
        self.ram.len() * 2
    }
}

#[derive(Debug, Clone)]
pub struct WaveIndex {
    source_clock_rate: usize,
    clock: usize,
    index: usize,
    length: usize,
}

impl WaveIndex {
    pub fn new(source_clock_rate: usize, length: usize) -> Self {
        Self {
            source_clock_rate,
            clock: 0,
            index: 0,
            length,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn set_source_clock_rate(&mut self, source_clock_rate: usize) {
        self.source_clock_rate = source_clock_rate;
    }

    pub fn update_index(&mut self, freq: usize) {
        self.clock += freq;

        if self.clock >= self.source_clock_rate {
            self.clock -= self.source_clock_rate;
            self.index = (self.index + 1) % self.length;
        }
    }
}
