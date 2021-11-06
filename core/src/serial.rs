use crate::hardware::HardwareHandle;
use crate::ic::Irq;
use log::*;

pub struct Serial {
    hw: HardwareHandle,
    irq: Irq,
    data: u8,
    recv: u8,
    ctrl: u8,
    clock: usize,
}

impl Serial {
    pub fn new(hw: HardwareHandle, irq: Irq) -> Self {
        Self {
            hw,
            irq,
            data: 0,
            recv: 0,
            ctrl: 0,
            clock: 0,
        }
    }

    pub fn step(&mut self, time: usize) {
        if self.ctrl & 0x80 == 0 {
            // No transfer
            return;
        }

        if self.ctrl & 0x01 != 0 {
            if self.clock < time {
                debug!("Serial transfer completed");
                self.data = self.recv;

                // End of transfer
                self.ctrl &= !0x80;
                self.irq.serial(true);
            } else {
                self.clock -= time;
            }
        } else {
            if let Some(data) = self.hw.get().borrow_mut().recv_byte() {
                self.hw.get().borrow_mut().send_byte(self.data);
                self.data = data;

                // End of transfer
                self.ctrl &= !0x80;
                self.irq.serial(true);
            }
        }
    }

    pub(crate) fn get_data(&self) -> u8 {
        self.data
    }

    pub(crate) fn get_ctrl(&self) -> u8 {
        self.ctrl
    }

    pub(crate) fn set_data(&mut self, value: u8) {
        self.data = value;
    }

    pub(crate) fn set_ctrl(&mut self, value: u8) {
        self.ctrl = value;

        if self.ctrl & 0x80 != 0 {
            if self.ctrl & 0x01 != 0 {
                debug!("Serial transfer (Internal): {:02x}", self.data);

                // Internal clock is 8192 Hz = 512 cpu clocks
                self.clock = 512 * 8;

                // Do transfer one byte at once
                self.hw.get().borrow_mut().send_byte(self.data);
                self.recv = self.hw.get().borrow_mut().recv_byte().unwrap_or(0xff);
            } else {
                debug!("Serial transfer (External): {:02x}", self.data);
            }
        }
    }
}
