use common::util::reg::*;
use volatile::Volatile;

#[allow(non_camel_case_types)]
type vu32 = Volatile<u32>;

const REG_BACKGROUND_CONTROL: *mut vu32 = 0x104000C0 as _;

#[repr(C)]
struct Gpu {
}

#[repr(C)]
pub struct FramebufferConfig {
    base: usize,
}

impl FramebufferConfig {
    pub unsafe fn top() -> FramebufferConfig {
        FramebufferConfig { base: 0x10400400 }
    }

    pub fn reg(&mut self, offset: usize) -> RW<u32> {
        RW::new(self.base + offset)
    }

    pub unsafe fn set_pixel_clock(&mut self, value: u32) {
        self.reg(0x00).write(value)
    }

    pub unsafe fn set_hblank_timer(&mut self, value: u32) {
        self.reg(0x04).write(value)
    }

    pub unsafe fn set_window_x_start(&mut self, value: u32) {
        self.reg(0x10).write(value)
    }

    pub unsafe fn set_window_x_end(&mut self, value: u32) {
        self.reg(0x14).write(value)
    }

    pub unsafe fn set_window_y_start(&mut self, value: u32) {
        self.reg(0x18).write(value)
    }

    pub unsafe fn set_window_y_end(&mut self, value: u32) {
        self.reg(0x20).write(value)
    }

    pub unsafe fn set_vblank_timer(&mut self, value: u32) {
        self.reg(0x24).write(value)
    }

    pub unsafe fn set_vtotal(&mut self, value: u32) {
        self.reg(0x30).write(value)
    }

    pub unsafe fn set_vdisp(&mut self, value: u32) {
        self.reg(0x34).write(value)
    }

    pub unsafe fn set_vertical_data_offset(&mut self, value: u32) {
        self.reg(0x38).write(value)
    }

    pub unsafe fn set_overscan_fillcolor(&mut self, value: u32) {
        self.reg(0x4c).write(value)
    }

    pub unsafe fn set_buffer0(&mut self, value: u32) {
        self.reg(0x68).write(value)
    }

    pub unsafe fn set_buffer1(&mut self, value: u32) {
        self.reg(0x6c).write(value)
    }

    pub unsafe fn set_buffer_format(&mut self, value: u32) {
        self.reg(0x70).write(value)
    }

    pub unsafe fn set_shown_buffer(&mut self, value: u32) {
        self.reg(0x78).write(value)
    }

    pub unsafe fn set_color_lut_index(&mut self, value: u32) {
        self.reg(0x80).write(value)
    }

    pub unsafe fn set_color_lut_color(&mut self, value: u32) {
        self.reg(0x84).write(value)
    }

    pub unsafe fn set_buffer_stride(&mut self, value: u32) {
        self.reg(0x90).write(value)
    }

    pub unsafe fn set_alt_buffer0(&mut self, value: u32) {
        self.reg(0x94).write(value)
    }

    pub unsafe fn set_alt_buffer1(&mut self, value: u32) {
        self.reg(0x98).write(value)
    }
}
