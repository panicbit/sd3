#![no_std]
#![no_main]

use core::ptr::write_volatile;
use volatile::Volatile;

mod lcd;

use lcd::FillColor;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        common::start();

        // write_volatile(0x10141200 as *mut u32, 0x1007F);

        // let color = 0x01FF00FF;
        // let addr = (LCD_TOP_CONF + 0x4) as *mut u32;
        // write_volatile(addr, color);

        // let backlight = 0x5F;
        // let addr = (LCD_TOP_CONF + 0x40) as *mut u32;
        // write_volatile(addr, backlight);

        // write_volatile(0x10202A44 as *mut u32, 0x1023E);

        lcd::init_screens();

        loop {}
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}