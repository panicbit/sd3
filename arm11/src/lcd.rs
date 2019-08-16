
use volatile::Volatile;

pub const TOP_ADDR: usize = 0x10202200;
pub const BOTTOM_ADDR: usize = 0x10202A00;

#[repr(C)]
pub struct Lcd {
    _unknown1: Volatile<u32>,
    fill_color: Volatile<u32>,
    _unknown2: Volatile<u32>,
    _unknown3: Volatile<u32>,
    _unknown4: Volatile<u32>,
    _unknown5: Volatile<u32>,
    _unknown6: Volatile<u32>,
    _unknown7: Volatile<u32>,
    _unknown8: Volatile<u32>,
    _unknown9: Volatile<u32>,
    _unknown10: Volatile<u32>,
    _unknown11: Volatile<u32>,
    _unknown12: Volatile<u32>,
    brightness: Volatile<u32>,
    _unknown13: Volatile<u32>,
    _unknown14: Volatile<u32>,
    _unknown15: Volatile<u32>,
    _unknown16: Volatile<u32>,
    _unknown17: Volatile<u32>,
    _unknown18: Volatile<u32>,
    calibration: Volatile<[u8; 100]>,
}

impl Lcd {
    pub unsafe fn new(addr: usize) -> &'static mut Lcd {
        &mut *(addr as *mut Lcd)
    }

    /// Sets the color to fill the screen with
    /// instead of showing the buffer's contents.
    pub fn set_fill_color(&mut self, color: impl Into<Option<FillColor>>) {
        let color = match color.into() {
            None => 0,
            Some(FillColor { r, g, b }) => {
                  (r as u32) << 0
                | (g as u32) << 8
                | (b as u32) << 16
                | 1 << 24 // enable
            }
        };

        self.fill_color.write(color);
    }

    pub fn set_brightness(&mut self, brightness: u32) {
        self.brightness.write(brightness);
    }
}

pub struct FillColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
