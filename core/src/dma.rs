use alloc::{vec, vec::Vec};
use log::*;

pub struct DmaRequest {
    pub src: u16,
    pub dst: u16,
}

impl DmaRequest {
    fn new(src: u16, dst: u16) -> Self {
        Self { src, dst }
    }
}

pub struct Dma {
    on: bool,
    src: u16,
    dst: u16,
    cycles: usize,
}

impl Dma {
    pub fn new() -> Self {
        Self {
            on: false,
            src: 0,
            dst: 0,
            cycles: 0,
        }
    }

    pub fn step(&mut self, mut cycles: usize) -> Vec<DmaRequest> {
        if !self.on {
            return vec![];
        }

        let mut reqs = vec![];

        while cycles > 0 || self.cycles > 0 {
            assert!(self.cycles >= 4);
            assert!(cycles >= 4);

            self.cycles -= 4;
            cycles -= 4;

            let req = DmaRequest::new(self.src, self.dst);
            self.src += 1;
            self.dst += 1;
            reqs.push(req)
        }

        if self.cycles == 0 {
            self.on = false;
        }

        reqs
    }

    /// Write DMA register (0xff46)
    pub fn start(&mut self, value: u8) {
        assert!(value <= 0xdf);
        self.on = true;
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
