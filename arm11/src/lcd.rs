
use volatile::Volatile;

const TOP_ADDR: usize = 0x10202200;
const BOTTOM_ADDR: usize = 0x10202A00;

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

pub unsafe fn init_screens() {
    let brightness_level = 0xFEFE;

    (*(0x10141200 as *mut Volatile<u32>)).write(0x1007F);

    let top = Lcd::new(TOP_ADDR);
    let bottom = Lcd::new(BOTTOM_ADDR);

    top.set_fill_color(FillColor { r: 0x00, g: 0xFF, b: 0x00 });
    bottom.set_fill_color(FillColor { r: 0x00, g: 0x00, b: 0xFF });
    
    
    (*(0x10202014 as *mut Volatile<u32>)).write(0x00000001);
    (*(0x1020200C as *mut Volatile<u32>)).update(|v| *v &= 0xFFFEFFFE);

    top.set_brightness(brightness_level);
    bottom.set_brightness(brightness_level);
    
    (*(0x10202244 as *mut Volatile<u32>)).write(0x1023E);
    (*(0x10202A44 as *mut Volatile<u32>)).write(0x1023E);

    //Top screen
    (*(0x10400400 as *mut Volatile<u32>)).write(0x000001c2);
    (*(0x10400404 as *mut Volatile<u32>)).write(0x000000d1);
    (*(0x10400408 as *mut Volatile<u32>)).write(0x000001c1);
    (*(0x1040040c as *mut Volatile<u32>)).write(0x000001c1);
    (*(0x10400410 as *mut Volatile<u32>)).write(0x00000000);
    (*(0x10400414 as *mut Volatile<u32>)).write(0x000000cf);
    (*(0x10400418 as *mut Volatile<u32>)).write(0x000000d1);
    (*(0x1040041c as *mut Volatile<u32>)).write(0x01c501c1);
    (*(0x10400420 as *mut Volatile<u32>)).write(0x00010000);
    (*(0x10400424 as *mut Volatile<u32>)).write(0x0000019d);
    (*(0x10400428 as *mut Volatile<u32>)).write(0x00000002);
    (*(0x1040042c as *mut Volatile<u32>)).write(0x00000192);
    (*(0x10400430 as *mut Volatile<u32>)).write(0x00000192);
    (*(0x10400434 as *mut Volatile<u32>)).write(0x00000192);
    (*(0x10400438 as *mut Volatile<u32>)).write(0x00000001);
    (*(0x1040043c as *mut Volatile<u32>)).write(0x00000002);
    (*(0x10400440 as *mut Volatile<u32>)).write(0x01960192);
    (*(0x10400444 as *mut Volatile<u32>)).write(0x00000000);
    (*(0x10400448 as *mut Volatile<u32>)).write(0x00000000);
    (*(0x1040045C as *mut Volatile<u32>)).write(0x00f00190);
    (*(0x10400460 as *mut Volatile<u32>)).write(0x01c100d1);
    (*(0x10400464 as *mut Volatile<u32>)).write(0x01920002);
    //(*(0x10400468 as *mut Volatile<u32>)).write((u32)fbs[0].top_left);
    //(*(0x1040046C as *mut Volatile<u32>)).write((u32)fbs[1].top_left);
    (*(0x10400470 as *mut Volatile<u32>)).write(0x80341);
    (*(0x10400474 as *mut Volatile<u32>)).write(0x00010501);
    (*(0x10400478 as *mut Volatile<u32>)).write(0);
    //(*(0x10400494 as *mut Volatile<u32>)).write((u32)fbs[0].top_right);
    //(*(0x10400498 as *mut Volatile<u32>)).write((u32)fbs[1].top_right);
    (*(0x10400490 as *mut Volatile<u32>)).write(0x000002D0);
    (*(0x1040049C as *mut Volatile<u32>)).write(0x00000000);

    //Disco register
    for i in 0..256 {
        (*(0x10400484 as *mut Volatile<u32>)).write(0x10101 * i);
    }

    // (*(0x10202204 as *mut Volatile<u32>)).write(0x0100FF00); //set LCD fill black to hide potential garbage -- NFIRM does it before firmlaunching

}

pub struct FillColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
