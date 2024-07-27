use crate::{
    clock::{ClockDivider, PrescaledTimer},
    ic::Irq,
};
use bitfield_struct::bitfield;
use log::*;

pub struct Timer {
    div: Div,
    tim: Tim,
    ctrl: Ctrl,
    div_apu_bit: bool,
}

struct Div {
    timer: PrescaledTimer,
}

impl Div {
    fn new() -> Self {
        Self {
            timer: PrescaledTimer::builder()
                .enable()
                .frequency(16384)
                .interval(256)
                .build(),
        }
    }

    fn counter(&self) -> u8 {
        self.timer.counter() as u8
    }

    fn reset(&mut self) {
        self.timer.reset();
    }

    fn step(&mut self, cycles: usize) {
        self.timer.step(cycles);
    }

    // TODO: To be used for STOP emulation where DIV doesn't ticks
    #[allow(dead_code)]
    fn enable(&mut self) {
        self.timer.enable();
    }

    // TODO: To be used for STOP emulation where DIV doesn't ticks
    #[allow(dead_code)]
    fn diable(&mut self) {
        self.timer.disable();
    }
}

struct Tim {
    timer: PrescaledTimer,
    load: u8,
    irq: Irq,
}

impl Tim {
    fn new(irq: Irq) -> Self {
        Self {
            timer: PrescaledTimer::builder()
                .disable()
                .frequency(4096)
                .interval(256)
                .build(),
            load: 0,
            irq,
        }
    }

    fn enable(&mut self) {
        self.timer.enable();
    }

    fn disable(&mut self) {
        self.timer.disable();
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

    fn step(&mut self, cycles: usize) {
        if self.timer.step(cycles) {
            self.timer.set_counter(self.load as usize);
            self.irq.timer(true);
        }
    }

    fn counter(&self) -> u8 {
        self.timer.counter() as u8
    }

    fn set_counter(&mut self, counter: u8) {
        self.timer.set_counter(counter as usize)
    }

    fn load(&self) -> u8 {
        self.load
    }

    fn set_load(&mut self, load: u8) {
        self.load = load;
    }
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
            div: Div::new(),
            tim: Tim::new(irq),
            ctrl: Ctrl::default(),
            div_apu_bit: false,
        }
    }

    pub fn step(&mut self, time: usize) -> bool {
        self.div.step(time);
        self.tim.step(time);
        false
    }

    pub(crate) fn on_read(&self, addr: u16) -> u8 {
        info!("Timer read: {:04x}", addr);
        match addr {
            0xff04 => self.div.counter(),
            0xff05 => self.tim.counter(),
            0xff06 => self.tim.load(),
            0xff07 => self.ctrl.into_bits(),
            _ => unreachable!("invalid timer read addr={:04x}", addr),
        }
    }

    pub(crate) fn on_write(&mut self, addr: u16, value: u8) {
        info!("Timer write: {:04x} {:02x}", addr, value);
        match addr {
            0xff04 => self.div.reset(),
            0xff05 => self.tim.set_counter(value),
            0xff06 => self.tim.set_load(value),
            0xff07 => {
                let prev_enable = self.ctrl.enable();

                self.ctrl = Ctrl::from_bits(value);

                if self.ctrl.enable() {
                    self.tim.enable();

                    if !prev_enable {
                        debug!("Timer started");
                        self.tim.reset(self.ctrl.select());
                    }
                } else {
                    self.tim.disable();
                }
            }
            _ => unreachable!("invalid timer write addr={:04x}, value={:04x}", addr, value),
        }
    }
}
