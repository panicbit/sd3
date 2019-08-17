#![no_std]
#![no_main]
#![feature(global_asm)]

use volatile::Volatile;
use lcd::*;
use gpu::FramebufferConfig;
use core::ptr::{read_volatile, write_volatile};
use common::input::PadState;

mod lcd;
mod gpu;

const SCREEN_TOP_WIDTH: usize = 400;
const SCREEN_BOTTOM_WIDTH: usize = 320;
const SCREEN_HEIGHT: usize = 240;
const SCREEN_TOP_FBSIZE: usize = (3 * SCREEN_TOP_WIDTH * SCREEN_HEIGHT);
const SCREEN_BOTTOM_FBSIZE: usize = (3 * SCREEN_BOTTOM_WIDTH * SCREEN_HEIGHT);

global_asm!(r#"
.section .text.start
.global _start
.align 4
.arm

_start:
    ldr r0, =0x24000000
    mov sp, r0

    blx _rust_start
.pool
"#);

const FERRIS: &[u8] = include_bytes!("../../ferris.data");

#[no_mangle]
pub extern "C" fn _rust_start() -> ! {
    unsafe {
        common::start();
        busy_sleep(1000);

        let fb_top = core::slice::from_raw_parts_mut::<[u8; 3]>(0x18000000 as *mut _, SCREEN_TOP_FBSIZE / 3);

        init_screens(fb_top);
    }

    loop {}
}

// #[no_mangle]
// pub unsafe fn go(addr: u32, entry_point: fn()) {
//     asm!("mov sp, r0" :: "{r0}"(addr) : /* "sp" */);
//     entry_point()
// }

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub unsafe fn init_screens(top_fb: &mut [[u8; 3]]) {
    let brightness_level = 0xFEFE;

    (*(0x10141200 as *mut Volatile<u32>)).write(0x1007F);

    (*(0x10202204 as *mut Volatile<u32>)).write(0x01000000); //set LCD fill black to hide potential garbage -- NFIRM does it before firmlaunching
    (*(0x10202A04 as *mut Volatile<u32>)).write(0x01000000);

    (*(0x10202014 as *mut Volatile<u32>)).write(0x00000001);

    (*(0x1020200C as *mut Volatile<u32>)).update(|v| *v &= 0xFFFEFFFE);
    (*(0x10202240 as *mut Volatile<u32>)).write(brightness_level);
    (*(0x10202A40 as *mut Volatile<u32>)).write(brightness_level);
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

    (*(0x10400468 as *mut Volatile<u32>)).write(top_fb.as_ptr() as _);
    (*(0x1040046C as *mut Volatile<u32>)).write(top_fb.as_ptr() as _);

    (*(0x10400470 as *mut Volatile<u32>)).write(0x80341);
    (*(0x10400474 as *mut Volatile<u32>)).write(0x00010501);
    (*(0x10400478 as *mut Volatile<u32>)).write(0);

    (*(0x10400494 as *mut Volatile<u32>)).write(top_fb.as_ptr() as _);
    (*(0x10400498 as *mut Volatile<u32>)).write(top_fb.as_ptr() as _);

    (*(0x10400490 as *mut Volatile<u32>)).write(0x000002D0);
    (*(0x1040049C as *mut Volatile<u32>)).write(0x00000000);

        //Disco register
    for i in 0 ..= 255 {
        (*(0x10400484 as *mut Volatile<u32>)).write(0x10101 * i);
    }

    setup_framebuffers(top_fb.as_ptr() as _);

    for (pixel, ferris_pixel) in top_fb.iter_mut().zip(FERRIS.chunks(3)) {
        write_volatile(&mut pixel[0], ferris_pixel[2]);
        write_volatile(&mut pixel[1], ferris_pixel[1]);
        write_volatile(&mut pixel[2], ferris_pixel[0]);
    }

    loop {
        let pad = PadState::read();
        write_volatile(&mut top_fb[0], [0, 0, pad.a() as u8 * 0xFF]);
        write_volatile(&mut top_fb[1], [0, pad.b() as u8 * 0xFF, 0]);
        write_volatile(&mut top_fb[2], [pad.select() as u8 * 0xFF, 0, 0]);
        write_volatile(&mut top_fb[3], [0, 0, pad.start() as u8 * 0xFF]);
        write_volatile(&mut top_fb[4], [0, pad.right() as u8 * 0xFF, 0]);
        write_volatile(&mut top_fb[5], [pad.left() as u8 * 0xFF, 0, 0]);
        write_volatile(&mut top_fb[6], [0, 0, pad.up() as u8 * 0xFF]);
        write_volatile(&mut top_fb[7], [0, pad.down() as u8 * 0xFF, 0]);
        write_volatile(&mut top_fb[8], [pad.r() as u8 * 0xFF, 0, 0]);
        write_volatile(&mut top_fb[9], [0, 0, pad.l() as u8 * 0xFF]);
        write_volatile(&mut top_fb[10], [0, pad.x() as u8 * 0xFF, 0]);
        write_volatile(&mut top_fb[11], [pad.y() as u8 * 0xFF, 0, 0]);
    }
}

unsafe fn setup_framebuffers(addr: u32) {
    (*(0x10202204 as *mut Volatile<u32>)).write(0x01000000); //set LCD fill black to hide potential garbage -- NFIRM does it before firmlaunching
    (*(0x10202A04 as *mut Volatile<u32>)).write(0x01000000);

    (*(0x10400468 as *mut Volatile<u32>)).write(addr);
    (*(0x1040046c as *mut Volatile<u32>)).write(addr);
    (*(0x10400494 as *mut Volatile<u32>)).write(addr);
    (*(0x10400498 as *mut Volatile<u32>)).write(addr);
    // (*(0x10400568 as *mut Volatile<u32>)).write((u32)fbs[0].bottom);
    // (*(0x1040056c as *mut Volatile<u32>)).write((u32)fbs[1].bottom);

    //Set framebuffer format, framebuffer select and stride
    (*(0x10400470 as *mut Volatile<u32>)).write(0x80341);
    (*(0x10400478 as *mut Volatile<u32>)).write(0);
    (*(0x10400490 as *mut Volatile<u32>)).write(0x2D0);
    (*(0x10400570 as *mut Volatile<u32>)).write(0x80301);
    (*(0x10400578 as *mut Volatile<u32>)).write(0);
    (*(0x10400590 as *mut Volatile<u32>)).write(0x2D0);

    (*(0x10202204 as *mut Volatile<u32>)).write(0x00000000); //unset LCD fill
    (*(0x10202A04 as *mut Volatile<u32>)).write(0x00000000);
}

fn busy_sleep(iterations: usize) {
    let n = 42;
    for _ in 0 .. 15 * iterations {
        unsafe {
            read_volatile(&n);
        }
    }
}