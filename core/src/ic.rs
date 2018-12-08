use std::sync::{Arc, Mutex};
use crate::mmu::Mmu;
use crate::mmu::{MemHandler, MemRead, MemWrite};

pub struct Ic {
    inner: Arc<Inner>,
}

impl Ic {
    pub fn new() -> Ic {
        Ic {
            inner: Arc::new(Inner::new()),
        }
    }

    pub fn handler(&self) -> IcMemHandler {
        IcMemHandler::new(self.inner.clone())
    }

    pub fn irq(&self) -> Irq {
        Irq::new(self.inner.clone())
    }

    pub fn poll(&self) -> Option<u8> {
        let e = self.inner.enable.lock().unwrap();
        let mut r = self.inner.request.lock().unwrap();

        if e.vblank && r.vblank {
            r.vblank = false;
            Some(0x40)
        } else if e.lcd && r.lcd {
            r.lcd = false;
            Some(0x48)
        } else if e.timer && r.timer {
            r.timer = false;
            Some(0x50)
        } else if e.serial && r.serial {
            r.serial = false;
            Some(0x58)
        } else if e.joypad && r.joypad {
            r.joypad = false;
            Some(0x60)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Irq {
    inner: Arc<Inner>,
}

impl Irq {
    fn new(inner: Arc<Inner>) -> Irq {
        Irq { inner }
    }

    pub fn vblank(&self, v: bool) {
        self.inner.request.lock().unwrap().vblank = v;
    }

    pub fn lcd(&self, v: bool) {
        self.inner.request.lock().unwrap().lcd = v;
    }

    pub fn timer(&self, v: bool) {
        self.inner.request.lock().unwrap().timer = v;
    }

    pub fn serial(&self, v: bool) {
        self.inner.request.lock().unwrap().serial = v;
    }

    pub fn joypad(&self, v: bool) {
        self.inner.request.lock().unwrap().joypad = v;
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

struct Inner {
    enable: Mutex<Ints>,
    request: Mutex<Ints>,
}

impl Inner {
    fn new() -> Inner {
        Inner {
            enable: Mutex::new(Ints::default()),
            request: Mutex::new(Ints::default()),
        }
    }
}

pub struct IcMemHandler {
    inner: Arc<Inner>,
}

impl IcMemHandler {
    fn new(inner: Arc<Inner>) -> IcMemHandler {
        IcMemHandler { inner }
    }
}

impl MemHandler for IcMemHandler {
    fn on_read(&self, mmu: &Mmu, addr: u16) -> MemRead {
        if addr == 0xffff {
            let v = self.inner.enable.lock().unwrap().get();
            info!("Read interrupt enable: {:02x}", v);
            MemRead::Replace(v)
        } else if addr == 0xff0f {
            let v = self.inner.request.lock().unwrap().get();
            info!("Read interrupt: {:02x}", v);
            MemRead::Replace(v)
        } else {
            MemRead::PassThrough
        }
    }

    fn on_write(&self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        if addr == 0xffff {
            info!("Read interrupt enable: {:02x}", value);
            self.inner.enable.lock().unwrap().set(value);
            MemWrite::Block
        } else if addr == 0xff0f {
            info!("Write interrupt: {:02x}", value);
            self.inner.request.lock().unwrap().set(value);
            MemWrite::Block
        } else {
            MemWrite::PassThrough
        }
    }
}
