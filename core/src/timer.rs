use crate::ic::Irq;
use log::*;

pub struct Timer {
    irq: Irq,
    div: u8,
    div_clocks: usize,
    tim: u8,
    tim_clocks: usize,
    tim_load: u8,
    ctrl: u8,
}

impl Timer {
    pub fn new(irq: Irq) -> Self {
        Self {
            irq,
            div: 0,
            div_clocks: 0,
            tim: 0,
            tim_clocks: 0,
            tim_load: 0,
            ctrl: 0,
        }
    }

    fn tim_clock_reset(&mut self) {
        self.tim_clocks = match self.ctrl & 0x3 {
            0x0 => 1024, // 4096Hz = 1024 cpu clocks
            0x1 => 16,   // 262144Hz = 16 cpu clocks
            0x2 => 64,   // 65536Hz = 64 cpu clocks
            0x3 => 256,  // 16384Hz = 256 cpu clocks
            _ => unreachable!(),
        };
    }

    fn div_clock_reset(&mut self) {
        self.div_clocks = 256; // 16384Hz = 256 cpu clocks
    }

    pub fn step(&mut self, time: usize) {
        if self.div_clocks < time {
            self.div = self.div.wrapping_add(1);
            let rem = time - self.div_clocks;
            self.div_clock_reset();
            self.div_clocks -= rem;
        } else {
            self.div_clocks -= time;
        }

        if self.ctrl & 0x04 == 0 {
            return;
        }

        if self.tim_clocks < time {
            let mut rem = time - self.tim_clocks;

            loop {
                let (tim, of) = self.tim.overflowing_add(1);
                self.tim = tim;
                if of {
                    self.tim = self.tim_load;
                    info!("Timer interrupt");
                    self.irq.timer(true);
                }
                self.tim_clock_reset();
                if rem <= self.tim_clocks {
                    self.tim_clocks -= rem;
                    break;
                }
                rem -= self.tim_clocks;
            }
        } else {
            self.tim_clocks -= time;
        }
    }

    pub(crate) fn on_read(&self, addr: u16) -> u8 {
        info!("Timer read: {:04x}", addr);
        match addr {
            0xff04 => self.div,
            0xff05 => self.tim,
            0xff06 => self.tim_load,
            0xff07 => self.ctrl,
            _ => unreachable!("invalid timer read addr={:04x}", addr),
        }
    }

    pub(crate) fn on_write(&mut self, addr: u16, value: u8) {
        info!("Timer write: {:04x} {:02x}", addr, value);
        match addr {
            0xff04 => self.div = 0,
            0xff05 => self.tim = value,
            0xff06 => self.tim_load = value,
            0xff07 => {
                let old_ctrl = self.ctrl;
                self.ctrl = value;

                if old_ctrl & 4 == 0 && value & 4 != 0 {
                    debug!("Timer started");
                    self.tim_clock_reset();
                }
            }
            _ => unreachable!("invalid timer write addr={:04x}, value={:04x}", addr, value),
        }
    }
}
