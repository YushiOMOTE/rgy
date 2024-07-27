use crate::{clock::ClockDivider, ic::Irq};
use log::*;

pub struct Timer {
    irq: Irq,
    div: Div,
    tim: u8,
    tim_clocks: usize,
    tim_load: u8,
    ctrl: u8,
    div_apu_bit: bool,
}

struct Div {
    enable: bool,
    count: u8,
    divider: ClockDivider,
}

impl Div {
    fn new() -> Self {
        Self {
            enable: true,
            count: 0,
            divider: ClockDivider::new(16384),
        }
    }

    fn count(&self) -> u8 {
        self.count
    }

    fn reset(&mut self) {
        self.count = 0;
        self.divider.reset();
    }

    fn step(&mut self, cycles: usize) {
        if !self.divider.step_one(cycles) {
            return;
        }

        if self.enable {
            self.count = self.count.wrapping_add(1);
        }
    }

    // TODO: To be used for STOP emulation where DIV doesn't ticks
    #[allow(dead_code)]
    fn enable(&mut self) {
        self.enable = true;
    }

    // TODO: To be used for STOP emulation where DIV doesn't ticks
    #[allow(dead_code)]
    fn diable(&mut self) {
        self.enable = false;
    }
}

impl Timer {
    pub fn new(irq: Irq) -> Self {
        Self {
            irq,
            div: Div::new(),
            tim: 0,
            tim_clocks: 0,
            tim_load: 0,
            ctrl: 0,
            div_apu_bit: false,
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

    pub fn step(&mut self, time: usize) -> bool {
        self.div.step(time);

        if self.ctrl & 0x04 == 0 {
            return false;
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

        false
    }

    pub(crate) fn on_read(&self, addr: u16) -> u8 {
        info!("Timer read: {:04x}", addr);
        match addr {
            0xff04 => self.div.count(),
            0xff05 => self.tim,
            0xff06 => self.tim_load,
            0xff07 => self.ctrl,
            _ => unreachable!("invalid timer read addr={:04x}", addr),
        }
    }

    pub(crate) fn on_write(&mut self, addr: u16, value: u8) {
        info!("Timer write: {:04x} {:02x}", addr, value);
        match addr {
            0xff04 => self.div.reset(),
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
