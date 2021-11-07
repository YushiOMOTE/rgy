use log::*;

pub struct DmaRequest {
    src: u16,
    dst: u16,
    len: u16,
}

impl DmaRequest {
    pub fn new(src: u16, dst: u16, len: u16) -> Self {
        Self { src, dst, len }
    }

    pub fn src(&self) -> u16 {
        self.src
    }

    pub fn dst(&self) -> u16 {
        self.dst
    }

    pub fn len(&self) -> u16 {
        self.len
    }
}

pub struct Dma {
    src: u16,
    dst: u16,
    cycles: usize,
}

impl Dma {
    pub fn new() -> Self {
        Self {
            src: 0,
            dst: 0,
            cycles: 0,
        }
    }

    pub fn step(&mut self, cycles: usize) -> Option<DmaRequest> {
        if self.cycles == 0 {
            return None;
        }

        // Ensure cpu cycles = machine cycles * 4
        assert!(cycles % 4 == 0);
        assert!(self.cycles % 4 == 0);

        // Copy 1 byte per a machine cycle
        let len = (cycles / 4).min(self.cycles / 4) as u16;
        let req = DmaRequest::new(self.src, self.dst, len);

        self.src += len;
        self.dst += len;
        self.cycles = self.cycles.saturating_sub(cycles);

        Some(req)
    }

    /// Write DMA register (0xff46)
    pub fn start(&mut self, value: u8) {
        assert!(value <= 0xdf);
        self.cycles = 160 * 4; // 160 machine cycles (* 4 for cpu cycles)
        self.src = (value as u16) << 8;
        self.dst = 0xfe00;
        debug!("Start DMA transfer: {:02x} to {:02x}", self.src, self.dst);
    }

    /// Read DMA register (0xff46)
    pub fn read(&self) -> u8 {
        // Write only
        0xff
    }
}
