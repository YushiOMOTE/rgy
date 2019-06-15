use core::cell::{Ref, RefCell, RefMut};
use alloc::rc::Rc;

use crate::mmu::{MemHandler, MemRead, MemWrite, Mmu};

pub struct Device<T>(Rc<RefCell<T>>);

impl<T> Device<T> {
    pub fn new(inner: T) -> Self {
        Self(Rc::new(RefCell::new(inner)))
    }

    pub fn borrow<'a>(&'a self) -> Ref<'a, T> {
        self.0.borrow()
    }

    pub fn borrow_mut<'a>(&'a self) -> RefMut<'a, T> {
        self.0.borrow_mut()
    }
}

impl<T: IoHandler> Device<T> {
    pub fn handler(&self) -> IoMemHandler<T> {
        IoMemHandler(self.0.clone())
    }
}

pub trait IoHandler {
    fn on_read(&mut self, mmu: &Mmu, addr: u16) -> MemRead;

    fn on_write(&mut self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite;
}

pub struct IoMemHandler<T>(Rc<RefCell<T>>);

impl<T: IoHandler> MemHandler for IoMemHandler<T> {
    fn on_read(&self, mmu: &Mmu, addr: u16) -> MemRead {
        // Don't hook if it's already hooked
        match self.0.try_borrow_mut() {
            Ok(mut inner) => inner.on_read(mmu, addr),
            Err(_) => MemRead::PassThrough,
        }
    }

    fn on_write(&self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        // Don't hook if it's already hooked
        match self.0.try_borrow_mut() {
            Ok(mut inner) => inner.on_write(mmu, addr, value),
            Err(_) => MemWrite::PassThrough,
        }
    }
}
