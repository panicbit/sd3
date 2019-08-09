#![no_std]

use core::ptr;

mod mem;

#[cfg(all(feature="arm9", feature="arm11"))]
compile_error!("arm9 and arm11 features are mutually exclusive");

pub unsafe fn start() {
    clear_bss();
}

unsafe fn clear_bss() {
    extern {
        static __bss_start: *mut u8;
        static __bss_end: *mut u8;
    }

    let len = __bss_end as usize - __bss_start as usize;

    ptr::write_bytes(__bss_start, 0, len);
}
