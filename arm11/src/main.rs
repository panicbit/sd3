#![no_std]
#![no_main]
#![feature(global_asm)]

use volatile::Volatile;
use lcd::*;
use gpu::FramebufferConfig;
use core::ptr::{read_volatile, write_volatile};

mod lcd;
mod gpu;

const SCREEN_TOP_WIDTH: usize = 400;
const SCREEN_BOTTOM_WIDTH: usize = 320;
const SCREEN_HEIGHT: usize = 240;
const SCREEN_TOP_FBSIZE: usize = (3 * SCREEN_TOP_WIDTH * SCREEN_HEIGHT);
const SCREEN_BOTTOM_FBSIZE: usize = (3 * SCREEN_BOTTOM_WIDTH * SCREEN_HEIGHT);

// static FB_TOP: [u32; SCREEN_TOP_WIDTH * SCREEN_HEIGHT] = [0x80; SCREEN_TOP_WIDTH * SCREEN_HEIGHT];

// global_asm!(r#"
// .section .text.start
// .global _start
// .align 4
// @ .arm

// _start:
//     @ Disable caches / mpu
//     mrc p15, 0, r4, c1, c0, 0  @ read control register
//     bic r4, #(1<<12)           @ - instruction cache disable
//     bic r4, #(1<<2)            @ - data cache disable
//     bic r4, #(1<<0)            @ - mpu disable
//     mcr p15, 0, r4, c1, c0, 0  @ write control register

//     @ Clear bss
//     @ldr r0, =__bss_start
//     @ldr r1, =__end__
//     @mov r2, #0

//     @.bss_clr:
//     @    cmp r0, r1
//     @    strlt r2, [r0], #4
//     @    blt .bss_clr

//     @ Give read/write access to all the memory regions
//     ldr r5, =0x33333333
//     mcr p15, 0, r5, c5, c0, 2 @ write data access
//     mcr p15, 0, r5, c5, c0, 3 @ write instruction access

//     @ Sets MPU permissions and cache settings
//     ldr r0, =0xFFFF001F	@ ffff0000 64k  | bootrom (unprotected / protected)
//     ldr r1, =0x3000801B	@ 30000000 16k  | dtcm
//     ldr r2, =0x01FF801D	@ 01ff8000 32k  | itcm
//     ldr r3, =0x08000029	@ 08000000 2M   | arm9 mem (O3DS / N3DS) 
//     ldr r4, =0x10000029	@ 10000000 2M   | io mem (ARM9 / first 2MB)
//     ldr r5, =0x20000037	@ 20000000 256M | fcram (O3DS / N3DS)
//     ldr r6, =0x1FF00027	@ 1FF00000 1M   | dsp / axi wram
//     ldr r7, =0x1800002D	@ 18000000 8M   | vram (+ 2MB)
//     mov r8, #0x2D
//     mcr p15, 0, r0, c6, c0, 0
//     mcr p15, 0, r1, c6, c1, 0
//     mcr p15, 0, r2, c6, c2, 0
//     mcr p15, 0, r3, c6, c3, 0
//     mcr p15, 0, r4, c6, c4, 0
//     mcr p15, 0, r5, c6, c5, 0
//     mcr p15, 0, r6, c6, c6, 0
//     mcr p15, 0, r7, c6, c7, 0
//     mcr p15, 0, r8, c3, c0, 0	@ Write bufferable 0, 2, 5
//     mcr p15, 0, r8, c2, c0, 0	@ Data cacheable 0, 2, 5
//     mcr p15, 0, r8, c2, c0, 1	@ Inst cacheable 0, 2, 5

//     @ Enable dctm
//     ldr r1, =0x3000800A        @ set dtcm
//     mcr p15, 0, r1, c9, c1, 0  @ set the dtcm Region Register

//     @ Enable caches
//     mrc p15, 0, r4, c1, c0, 0  @ read control register
//     orr r4, r4, #(1<<18)       @ - itcm enable
//     orr r4, r4, #(1<<16)       @ - dtcm enable
//     orr r4, r4, #(1<<12)       @ - instruction cache enable
//     orr r4, r4, #(1<<2)        @ - data cache enable
//     orr r4, r4, #(1<<0)        @ - mpu enable
//     mcr p15, 0, r4, c1, c0, 0  @ write control register

//     @ Flush caches
//     mov r5, #0
//     mcr p15, 0, r5, c7, c5, 0  @ flush I-cache
//     mcr p15, 0, r5, c7, c6, 0  @ flush D-cache
//     mcr p15, 0, r5, c7, c10, 4 @ drain write buffer

//     @ Fixes mounting of SDMC
//     ldr r0, =0x10000020
//     mov r1, #0x340
//     str r1, [r0]

//     mov sp, #0x27000000

//     blx _rust_start
//     b _start

// .pool
// "#);

// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     unsafe {
//         go(0x30000000, main);

//         loop {}
//     }
// }


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

#[no_mangle]
pub extern "C" fn _rust_start() -> ! {
    unsafe {
        common::start();
        busy_sleep(1000);

        let fb_top = core::slice::from_raw_parts_mut::<[u8; 3]>(0x18000000 as *mut _, SCREEN_TOP_FBSIZE / 3);

        for b in fb_top.iter_mut() {
            *b = [0x00, 0xFF, 0x00];
        }

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

    // busy_sleep(4000);

    // Set color to yellow
    // (*(0x10202204 as *mut Volatile<u32>)).write(0xFFFF0000);


    // for b in top_fb.iter_mut() {
    //     core::ptr::write_volatile(&mut b[0], 0x00);
    //     core::ptr::write_volatile(&mut b[1], 0x00);
    //     core::ptr::write_volatile(&mut b[2], 0x00);
    // }

    for (i, b) in top_fb.iter_mut().enumerate() {
        let color = match i / ((SCREEN_TOP_WIDTH * SCREEN_HEIGHT) / 3) {
            0 => [0x00, 0x00, 0xFF],
            1 => [0x00, 0xFF, 0x00],
            2 => [0xFF, 0x00, 0x00],
            _ => [0x00, 0x00, 0xFF],
        };

        core::ptr::write_volatile(b, color);
    }

    loop {}
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