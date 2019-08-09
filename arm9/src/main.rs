#![no_std]
#![no_main]

#[macro_use] extern crate bitflags;

mod i2c;

use core::ptr::{write_volatile, read_volatile};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        common::start();

        i2c::init();


        i2c::write_reg(i2c::DEVICE_MCU, 0x2A, 1);

        let mut pattern = [0; 0x64];
        pattern[0] = 64;
        pattern[1] = 0xFF;
        pattern[2] = 0;
        pattern[3] = 0;

        for i in 0..32 {
            let r = if i <= 10 { 0xFF } else { 0x00 };
            let g = if i > 10 && i <= 21 { 0xFF } else { 0x00 };
            let b = if i > 21 { 0xFF } else { 0x00 };
            pattern[i + 4 +  0] = r;
            pattern[i + 4 + 32] = g;
            pattern[i + 4 + 64] = b;
        }

        // pattern[0] = 0x01;
        // pattern[1] = 0x80;
        // pattern[2] = 0x01;
        // pattern[3] = 0x00;

        i2c::write_reg_buf(i2c::DEVICE_MCU, 0x2d, &pattern);

        busy_sleep(100);
        init_screens();
        busy_sleep(1_000);

        // shutdown
        i2c::write_reg(i2c::DEVICE_MCU, 0x20, 1);
    }

    loop {}
}

unsafe fn init_screens() {
    // Turn on backlight
    i2c::write_reg(i2c::DEVICE_MCU, 0x22, 0x2A);
}

fn busy_sleep(iterations: usize) {
    let n = 42;
    for _ in 0 .. 15 * iterations {
        unsafe {
            read_volatile(&n);
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
