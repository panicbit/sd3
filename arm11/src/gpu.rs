use volatile::Volatile;

#[allow(non_camel_case_types)]
type vu32 = Volatile<u32>;

const REG_BACKGROUND_CONTROL: *mut vu32 = 0x104000C0 as _;

#[repr(C)]
struct Gpu {
    _unknown1: vu32,
    _unknown2: vu32

}

pub unsafe fn volatile<T: Copy>(addr: usize) -> &'static mut Volatile<T> {
    (addr as *mut Volatile<T>).as_mut().unwrap()
}

#[repr(C)]
pub struct FramebufferConfig {
    base: usize,
}

impl FramebufferConfig {
    pub unsafe fn top() -> FramebufferConfig {
        FramebufferConfig { base: 0x10400400 }
    }

    pub fn reg(&mut self, offset: usize) -> &mut Volatile<u32> {
        unsafe {
            volatile(self.base + offset)
        }
    }

    pub fn set_pixel_clock(&mut self, value: u32) {
        self.reg(0x00).write(value)
    }

    pub fn set_hblank_timer(&mut self, value: u32) {
        self.reg(0x04).write(value)
    }

    pub fn set_window_x_start(&mut self, value: u32) {
        self.reg(0x10).write(value)
    }

    pub fn set_window_x_end(&mut self, value: u32) {
        self.reg(0x14).write(value)
    }

    pub fn set_window_y_start(&mut self, value: u32) {
        self.reg(0x18).write(value)
    }

    pub fn set_window_y_end(&mut self, mut value: u16) {
        self.reg(0x20).write(value as u32);
    }
}
