#![no_std]
#![no_main]
#![feature(global_asm, asm)]
#![feature(panic_info_message)]

#[macro_use] extern crate bitflags;

use volatile::Volatile;
use lcd::*;
use gpu::FramebufferConfig;
use core::ptr::{read_volatile, write_volatile};
use core::{str, fmt, cmp};
use core::fmt::Write;
use common::input::PadState;
use common::util::reg::*;
use common::Console;

mod lcd;
mod gpu;
mod panic;
mod mpcore;
mod boot11;

const SCREEN_TOP_WIDTH: usize = 400;
const SCREEN_BOTTOM_WIDTH: usize = 320;
const SCREEN_HEIGHT: usize = 240;
const SCREEN_TOP_FBSIZE: usize = (3 * SCREEN_TOP_WIDTH * SCREEN_HEIGHT);
const SCREEN_BOTTOM_FBSIZE: usize = (3 * SCREEN_BOTTOM_WIDTH * SCREEN_HEIGHT);

extern {
    fn _start() -> !;
}

global_asm!(r#"
.section .text.start
.global _start
.align 4
.arm

_start:
    cpsid aif

    ldr r0, =0x24000000
    mov sp, r0

    blx _rust_start
.pool
"#);

const FERRIS: &[u8] = include_bytes!("../../ferris.data");

#[no_mangle]
pub extern "C" fn _rust_start() -> ! {
    unsafe {
        mpcore::enable_scu();
        mpcore::enable_smp_mode();
        mpcore::disable_interrupts();
        mpcore::clean_and_invalidate_data_cache();

        if mpcore::cpu_id() == 0 {
            boot11::start_cpu(1, _start);
            loop {}
        }

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

    let ref mut console = Console::new(top_fb, 400, 240);
    console.clear([0; 3]);

    'render: loop {
        let pad = PadState::read();

        if pad.x() {
            break;
        }

        console.go_to(0, 0);
        writeln!(console, "GPIO_DATA0 = 0b{:016b}", RO::<u16>::new(0x10147000).read());
        writeln!(console, "GPIO_DATA1 = 0b{:032b}", RO::<u32>::new(0x10147010).read());
        writeln!(console, "GPIO_DATA2 = 0b{:016b}", RO::<u16>::new(0x10147014).read());
        writeln!(console, "GPIO_DATA3 = 0b{:016b}", RO::<u16>::new(0x10147020).read());
        writeln!(console, "GPIO_?? = 0b{:016b}", RO::<u16>::new(0x10147024).read());
        writeln!(console, "GPIO_? = 0b{:016b}", RO::<u16>::new(0x10147026).read());
        writeln!(console, "GPIO_DATA4 = 0b{:016b}", RO::<u16>::new(0x10147028).read());
    }

    panic!("\
        end of program! :D\n\
        cpu id: 0x{id:X}\n\
        cpu status: 0b{status:06b}",
        id = mpcore::cpu_id(),
        status = mpcore::cpu_status(),
    );
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
