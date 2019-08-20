use core::ptr::{read_volatile, write_volatile};

pub struct RW<T>(*mut T);

impl<T> RW<T> {
    pub const fn new(addr: usize) -> Self {
        Self(addr as *mut T)
    }

    pub unsafe fn read(&self) -> T {
        read_volatile(self.0)
    }

    pub unsafe fn write(&mut self, t: T) {
        write_volatile(self.0, t)
    }
}

pub struct RO<T>(*const T);

impl<T> RO<T> {
    pub const fn new(addr: usize) -> Self {
        Self(addr as *const T)
    }

    pub unsafe fn read(&self) -> T {
        read_volatile(self.0)
    }
}

pub struct WO<T>(*mut T);

impl<T> WO<T> {
    pub const fn new(addr: usize) -> Self {
        Self(addr as *mut T)
    }

    pub unsafe fn write(&mut self, t: T) {
        write_volatile(self.0, t)
    }
}
