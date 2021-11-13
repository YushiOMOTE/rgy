use alloc::rc::Rc;
use core::cell::RefCell;
use log::*;

/// Sharable handle for I/O devices to request/cancel interrupts
#[derive(Clone)]
pub struct Irq {
    enable: Rc<RefCell<Ints>>,
    request: Rc<RefCell<Ints>>,
}

impl Irq {
    pub fn new() -> Irq {
        Irq {
            enable: Rc::new(RefCell::new(Ints::default())),
            request: Rc::new(RefCell::new(Ints::default())),
        }
    }

    /// Request/cacnel vblank interrupt
    pub fn vblank(&self, v: bool) {
        self.request.borrow_mut().vblank = v;
    }

    /// Request/cancel LCD interrupt
    pub fn lcd(&self, v: bool) {
        self.request.borrow_mut().lcd = v;
    }

    /// Request/cancel timer interrupt
    pub fn timer(&self, v: bool) {
        self.request.borrow_mut().timer = v;
    }

    /// Request/cancel serial interrupt
    pub fn serial(&self, v: bool) {
        self.request.borrow_mut().serial = v;
    }

    /// Request/cancel joypad interrupt
    pub fn joypad(&self, v: bool) {
        self.request.borrow_mut().joypad = v;
    }
}

#[derive(Debug, Default)]
struct Ints {
    vblank: bool,
    lcd: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
}

impl Ints {
    fn set(&mut self, value: u8) {
        self.vblank = value & 0x01 != 0;
        self.lcd = value & 0x02 != 0;
        self.timer = value & 0x04 != 0;
        self.serial = value & 0x08 != 0;
        self.joypad = value & 0x10 != 0;
    }

    fn get(&self) -> u8 {
        let mut v = 0;
        v |= if self.vblank { 0x01 } else { 0x00 };
        v |= if self.lcd { 0x02 } else { 0x00 };
        v |= if self.timer { 0x04 } else { 0x00 };
        v |= if self.serial { 0x08 } else { 0x00 };
        v |= if self.joypad { 0x10 } else { 0x00 };
        v
    }
}

/// Interrupt controller
pub struct Ic {
    irq: Irq,
}

impl Ic {
    pub fn new(irq: Irq) -> Ic {
        Ic { irq }
    }

    /// Get the interrupt vector address without clearing the interrupt flag state
    pub fn peek(&self) -> Option<u8> {
        self.check(false)
    }

    /// Get the interrupt vector address clearing the interrupt flag state
    pub fn pop(&self) -> Option<u8> {
        self.check(true)
    }

    fn check(&self, consume: bool) -> Option<u8> {
        let e = self.irq.enable.borrow();
        let mut r = self.irq.request.borrow_mut();

        if e.vblank && r.vblank {
            r.vblank = !consume;
            Some(0x40)
        } else if e.lcd && r.lcd {
            r.lcd = !consume;
            Some(0x48)
        } else if e.timer && r.timer {
            r.timer = !consume;
            Some(0x50)
        } else if e.serial && r.serial {
            r.serial = !consume;
            Some(0x58)
        } else if e.joypad && r.joypad {
            r.joypad = !consume;
            Some(0x60)
        } else {
            None
        }
    }

    /// Read IE register (0xffff)
    pub fn read_enabled(&self) -> u8 {
        let v = self.irq.enable.borrow().get();
        info!("Read interrupt enable: {:02x}", v);
        v
    }

    /// Write IF register (0xff0f)
    pub fn read_flags(&self) -> u8 {
        let v = self.irq.request.borrow().get();
        info!("Read interrupt: {:02x}", v);
        v | 0xe0
    }

    /// Write IE register (0xffff)
    pub fn write_enabled(&mut self, value: u8) {
        info!("Write interrupt enable: {:02x}", value);
        self.irq.enable.borrow_mut().set(value);
    }

    /// Write IF register (0xff0f)
    pub fn write_flags(&mut self, value: u8) {
        info!("Write interrupt: {:02x}", value);
        self.irq.request.borrow_mut().set(value);
    }
}
