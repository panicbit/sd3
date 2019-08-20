#![no_std]

#[macro_use] extern crate bitflags;

pub mod mem;
pub mod input;
pub mod util;

#[cfg(all(feature="arm9", feature="arm11"))]
compile_error!("arm9 and arm11 features are mutually exclusive");

pub unsafe fn start() {
    clear_bss();
}

unsafe fn clear_bss() {
    extern {
        static mut __bss_start: u8;
        static mut __bss_end: u8;
    }

    let bss_start = &mut __bss_start as *mut u8;
    let bss_end = &mut __bss_end as *mut u8;
    let len = bss_end as usize - bss_start as usize;

    bss_start.write_bytes(0, len);
}
