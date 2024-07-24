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
