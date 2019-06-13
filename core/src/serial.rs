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

        if self.clock < time {
            self.data = self.recv;

            // End of transfer
            self.ctrl &= !0x80;
            self.irq.serial(true);
        } else {
            self.clock -= time;
        }
    }
}

impl IoHandler for Serial {
    fn on_read(&mut self, _mmu: &Mmu, addr: u16) -> MemRead {
        if addr == 0xff01 {
            MemRead::Replace(self.data)
        } else if addr == 0xff01 {
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
                debug!("Serial transfer: {:02x}", self.data);

                self.clock = if self.ctrl & 0x01 != 0 {
                    // Internal clock is 8192 Hz = 512 cpu clocks
                    512 * 8
                } else {
                    // External clock can be any; assume 65536 Hz = 64 cpu clocks
                    64 * 8
                };

                // Do transfer one byte at once
                self.hw.get().borrow_mut().send_byte(self.data);
                self.recv = self.hw.get().borrow_mut().recv_byte().unwrap_or(0xff);
            }
            MemWrite::Block
        } else {
            unreachable!("Write to serial: {:04x} {:02x}", addr, value)
        }
    }
}
