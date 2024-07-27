use crate::{clock::PrescaledTimer, ic::Irq};
use bitfield_struct::bitfield;
use log::*;

pub struct Timer {
    ctrl: Ctrl,
    timer: PrescaledTimer,
    load: u8,
    irq: Irq,
}

#[bitfield(u8)]
struct Ctrl {
    #[bits(2)]
    select: usize,
    enable: bool,
    #[bits(5)]
    _unused: u8,
}

impl Timer {
    pub fn new(irq: Irq) -> Self {
        Self {
            timer: PrescaledTimer::builder()
                .disable()
                .frequency(4096)
                .interval(256)
                .build(),
            ctrl: Ctrl::default(),
            load: 0,
            irq,
        }
    }

    pub fn step(&mut self, cycles: usize) {
        if self.timer.step(cycles) {
            self.timer.set_counter(self.load as usize);
            self.irq.timer(true);
        }
    }

    fn reset(&mut self, select: usize) {
        let freq = match select {
            0 => 4096,
            1 => 262144,
            2 => 65536,
            3 => 16384,
            _ => unreachable!(),
        };
        self.timer.set_frequency(freq);
    }

    pub(crate) fn on_read(&self, addr: u16) -> u8 {
        info!("Timer read: {:04x}", addr);
        match addr {
            0xff05 => self.timer.counter() as u8,
            0xff06 => self.load,
            0xff07 => self.ctrl.into_bits(),
            _ => unreachable!("invalid timer read addr={:04x}", addr),
        }
    }

    pub(crate) fn on_write(&mut self, addr: u16, value: u8) {
        info!("Timer write: {:04x} {:02x}", addr, value);
        match addr {
            0xff05 => self.timer.set_counter(value as usize),
            0xff06 => self.load = value,
            0xff07 => {
                let prev_enable = self.ctrl.enable();

                self.ctrl = Ctrl::from_bits(value);

                if self.ctrl.enable() {
                    self.timer.enable();

                    if !prev_enable {
                        debug!("Timer started");
                        self.reset(self.ctrl.select());
                    }
                } else {
                    self.timer.disable();
                }
            }
            _ => unreachable!("invalid timer write addr={:04x}, value={:04x}", addr, value),
        }
    }
}
