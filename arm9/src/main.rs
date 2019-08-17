#![no_std]
#![no_main]

#[macro_use] extern crate bitflags;

mod i2c;

use core::ptr::{write_volatile, read_volatile};
use volatile::Volatile;

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

        init_screens();
        sleep_msecs(4_000);

        // shutdown
        i2c::write_reg(i2c::DEVICE_MCU, 0x20, 1);
    }

    loop {}
}

unsafe fn init_screens() {
    // Turn on backlight
    i2c::write_reg(i2c::DEVICE_MCU, 0x22, 0x2A);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[repr(C)]
struct Timer {
    value: Volatile<u16>,
    control: Volatile<u16>,
}

const TICKS_PER_SEC: u64 = 67_027_964;

unsafe fn timers() -> &'static mut [Timer; 4] {
    &mut *(0x10003000 as *mut [Timer; 4])
}

fn timer_start() -> u64 {
    let timers = unsafe { timers() };

    for timer in &mut *timers {
        timer.control.write(0);
        timer.value.write(0);
    }

    timers[0].control.write(0b1_0_000_0_00);

    for timer in &mut timers[1..] {
        timer.control.write(0b1_0_000_1_00);
    }

    timer_ticks(0)
}

fn timer_ticks(start_time: u64) -> u64 {
    let timers = unsafe { timers() };
    let mut ticks: u64 = 0;
    ticks |= (timers[0].value.read() as u64) << 0;
    ticks |= (timers[1].value.read() as u64) << 16;
    ticks |= (timers[2].value.read() as u64) << 32;
    ticks |= (timers[3].value.read() as u64) << 48;
    ticks - start_time
}

fn timer_msecs(start_time: u64) -> u64 {
    timer_ticks(start_time) / (TICKS_PER_SEC / 1000)
}

fn sleep_msecs(msecs: u64) {
    let start = timer_start();
    while timer_msecs(start) < msecs {}
}