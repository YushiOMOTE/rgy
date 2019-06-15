use crate::device::IoHandler;
use crate::hardware::HardwareHandle;
use crate::ic::Irq;
use crate::mmu::{MemRead, MemWrite, Mmu};
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
}

impl IoHandler for Serial {
    fn on_read(&mut self, _mmu: &Mmu, addr: u16) -> MemRead {
        if addr == 0xff01 {
            MemRead::Replace(self.data)
        } else if addr == 0xff02 {
            MemRead::Replace(self.ctrl)
        } else {
            unreachable!("Read from serial: {:04x}", addr)
        }
    }

    fn on_write(&mut self, _mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        if addr == 0xff01 {
            self.data = value;
            MemWrite::Block
        } else if addr == 0xff02 {
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
            MemWrite::Block
        } else {
            unreachable!("Write to serial: {:04x} {:02x}", addr, value)
        }
    }
}
