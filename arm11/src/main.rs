#![no_std]
#![no_main]
#![feature(global_asm, asm, naked_functions)]
#![feature(panic_info_message)]

#[macro_use] extern crate bitflags;

use volatile::Volatile;
use lcd::*;
use gpu::FramebufferConfig;
use core::ptr::{read_volatile, write_volatile};
use core::{str, fmt, cmp, mem};
use core::fmt::{Write, UpperHex, Binary};
use common::input::GamePad;
use common::util::reg::*;
use common::Console;
use common::mem::arm11::*;
use num_traits::PrimInt;

mod lcd;
mod gpu;
mod panic;
mod mpcore;
mod boot11;
mod exceptions;

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
    cpsid aif, #0x13

    ldr r0, =0x24000000
    mov sp, r0

    blx _rust_start
.pool
"#);

const FERRIS: &[u8] = include_bytes!("../../ferris.data");

#[no_mangle]
pub unsafe extern "C" fn _rust_start() -> ! {
    exceptions::install_handlers();
    // mpcore::enable_scu();
    // mpcore::enable_smp_mode();
    // mpcore::disable_interrupts();
    // mpcore::clean_and_invalidate_data_cache();

    // if mpcore::cpu_id() == 0 {
    //     boot11::start_cpu(1, _start);
    //     loop {}
    // }

    common::start();

    busy_sleep(1000);

    let fb_top = core::slice::from_raw_parts_mut::<[u8; 3]>(0x18000000 as *mut _, SCREEN_TOP_FBSIZE / 3);

    init_screens(fb_top);

    {
        for (pixel, ferris_pixel) in fb_top.iter_mut().zip(FERRIS.chunks(3)) {
            write_volatile(&mut pixel[0], ferris_pixel[2]);
            write_volatile(&mut pixel[1], ferris_pixel[1]);
            write_volatile(&mut pixel[2], ferris_pixel[0]);
        }
    }

    let ref mut console = Console::new(fb_top, 400, 240);
    // console.clear([0; 3]);/*  */

    let mut pad = GamePad::new();
    let mut bg_color = U32HexEditor::new(0);
    let mut fg_color = U32HexEditor::new(0xFFFFFF00);
    let mut fg_selected = false;

    loop {
        console.go_to(0, 0);

        let base = AXI_WRAM.end - 0x60;
        print_addr::<u32>(console, base + 0x10, "svc vector instr");
        print_addr::<u32>(console, base + 0x10, "svc vector addr");
        print_addr_bin::<u16>(console, 0x10146000, "pad");

        writeln!(console, "cpsr = 0b{:032b}", mpcore::cpu_status_reg()).ok();

        static mut N: u32 = 0;
        writeln!(console, "frame {}", N).ok();
        N = N.wrapping_add(1);

        if !pad.l() && pad.y_once() {
            fg_selected = !fg_selected;
        }

        console.set_bg(u32_to_rgb(bg_color.value()));
        console.set_fg(u32_to_rgb(fg_color.value()));

        {
            if !fg_selected {
                bg_color.manipulate(&pad);
            }
            write!(console, "bg_color = ").ok();
            bg_color.render_with_cursor(console, !fg_selected);
            writeln!(console, "").ok();
        }

        {
            if fg_selected {
                fg_color.manipulate(&pad);
            }
            write!(console, "fg_color = ").ok();
            fg_color.render_with_cursor(console, fg_selected);
            writeln!(console, "").ok();
        }

        // trigger svc
        if pad.l() && pad.a() {
            asm!("svc 42");
        }

        // trigger data abort
        if pad.l() && pad.b() {
            RW::<usize>::new(0).write(42);
        }

        // trigger prefetch abort
        if pad.l() && pad.y() {
            asm!("bkpt");
        }

        // trigger undefined instruction
        if pad.l() && pad.x() {
            asm!(".word 0xFFFFFFFF");
        }

        pad.poll();
    }
}

struct U32HexEditor {
    cursor_pos: usize,
    value: u32,
}

impl U32HexEditor {
    const fn new(value: u32) -> Self {
        Self {
            cursor_pos: 0,
            value,
        }
    }
    fn cursor_left(&mut self) {
        self.cursor_pos += 1;
        self.cursor_pos %= 8;
    }

    fn cursor_right(&mut self) {
        self.cursor_pos += 8 - 1;
        self.cursor_pos %= 8;
    }

    fn increment(&mut self) {
        self.modify(|digit| {
            *digit += 1;
            *digit %= 16;
        })
    }

    fn decrement(&mut self) {
        self.modify(|digit| {
            *digit += 16 - 1;
            *digit %= 16;
        })
    }

    fn modify(&mut self, f: impl FnOnce(&mut u32)) {
        let pos = self.cursor_pos * 4;

        // Extract digit
        let mut digit = (self.value >> pos) & 0xF;

        f(&mut digit);
        digit &= 0xF;

        // Clear digit
        self.value &= !(0xF << pos);

        // Insert digit
        self.value |= digit << pos;
    }

    fn render(&self, console: &mut Console) {
        self.render_with_cursor(console, true);
    }

    fn render_with_cursor(&self, console: &mut Console, with_cursor: bool) {
        write!(console, "0x").ok();

        for cursor_pos in (0..8).rev() {
            let pos = cursor_pos * 4;
            let digit = (self.value >> pos) & 0xF;

            if with_cursor && cursor_pos == self.cursor_pos {
                console.swap_colors();
            }

            write!(console, "{:X}", digit).ok();

            if with_cursor && cursor_pos == self.cursor_pos {
                console.swap_colors();
            }
        }
    }

    fn manipulate(&mut self, pad: &GamePad) {
        if pad.left_once() {
            self.cursor_left();
        }

        if pad.right_once() {
            self.cursor_right();
        }

        if pad.up_once() {
            self.increment();
        }

        if pad.down_once() {
            self.decrement();
        }
    }

    fn set_value(&mut self, value: u32) {
        self.value = value;
    }

    fn value(&self) -> u32 {
        self.value
    }
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
    let mut top_fb_conf = gpu::FramebufferConfig::top();
    top_fb_conf.set_pixel_clock(0x1c2);
    top_fb_conf.set_hblank_timer(0xd1);
    top_fb_conf.reg(0x08).write(0x1c1);
    top_fb_conf.reg(0x0c).write(0x1c1);
    top_fb_conf.set_window_x_start(0);
    top_fb_conf.set_window_x_end(0xcf);
    top_fb_conf.set_window_y_start(0xd1);
    top_fb_conf.reg(0x1c).write(0x01c501c1);
    top_fb_conf.set_window_y_end(0x10000);
    top_fb_conf.set_vblank_timer(0x19d);
    top_fb_conf.reg(0x28).write(0x2);
    top_fb_conf.reg(0x2c).write(0x192);
    top_fb_conf.set_vtotal(0x192);
    top_fb_conf.set_vdisp(0x192);
    top_fb_conf.set_vertical_data_offset(0x1);
    top_fb_conf.reg(0x3c).write(0x2);
    top_fb_conf.reg(0x40).write(0x01960192);
    top_fb_conf.reg(0x44).write(0);
    top_fb_conf.reg(0x48).write(0);
    top_fb_conf.reg(0x5C).write(0x00f00190);
    top_fb_conf.reg(0x60).write(0x01c100d1);
    top_fb_conf.reg(0x64).write(0x01920002);

    top_fb_conf.set_buffer0(top_fb.as_ptr() as _);
    top_fb_conf.set_buffer1(top_fb.as_ptr() as _);

    top_fb_conf.set_buffer_format(0x80341);
    top_fb_conf.reg(0x74).write(0x10501);
    top_fb_conf.set_shown_buffer(0);

    top_fb_conf.set_alt_buffer0(top_fb.as_ptr() as _);
    top_fb_conf.set_alt_buffer1(top_fb.as_ptr() as _);

    top_fb_conf.set_buffer_stride(0x2D0);
    top_fb_conf.reg(0x9C).write(0);

    // Set up color LUT
    top_fb_conf.set_color_lut_index(0);
    for i in 0 ..= 255 {
        top_fb_conf.set_color_lut_color(0x10101 * i);
    }

    setup_framebuffers(top_fb.as_ptr() as _);
}

unsafe fn setup_framebuffers(addr: u32) {
    (*(0x10202204 as *mut Volatile<u32>)).write(0x01000000); //set LCD fill black to hide potential garbage -- NFIRM does it before firmlaunching
    (*(0x10202A04 as *mut Volatile<u32>)).write(0x01000000);

    let mut top_fb_conf = gpu::FramebufferConfig::top();
    top_fb_conf.reg(0x68).write(addr);
    top_fb_conf.reg(0x6c).write(addr);
    top_fb_conf.reg(0x94).write(addr);
    top_fb_conf.reg(0x98).write(addr);
    // (*(0x10400568 as *mut Volatile<u32>)).write((u32)fbs[0].bottom);
    // (*(0x1040056c as *mut Volatile<u32>)).write((u32)fbs[1].bottom);

    //Set framebuffer format, framebuffer select and stride
    top_fb_conf.reg(0x70).write(0x80341);
    top_fb_conf.reg(0x78).write(0);
    top_fb_conf.reg(0x90).write(0x2D0);
    (*(0x10400570 as *mut Volatile<u32>)).write(0x80301);
    (*(0x10400578 as *mut Volatile<u32>)).write(0);
    (*(0x10400590 as *mut Volatile<u32>)).write(0x2D0);

    (*(0x10202204 as *mut Volatile<u32>)).write(0x00000000); //unset LCD fill
    (*(0x10202A04 as *mut Volatile<u32>)).write(0x00000000);
}

unsafe fn print_addr<T: PrimInt + UpperHex>(console: &mut Console, addr: usize, label: &'static str) {
    writeln!(console, "[0x{addr:08X}] = 0x{value:0width$X} {label}",
        addr = addr,
        value = RO::<T>::new(addr).read(),
        width = 2 * mem::size_of::<T>(),
        label = label,
    ).ok();
}

unsafe fn print_addr_bin<T: PrimInt + Binary>(console: &mut Console, addr: usize, label: &'static str) {
    writeln!(console, "[0x{addr:08X}] = 0b{value:0width$b} {label}",
        addr = addr,
        value = RO::<T>::new(addr).read(),
        width = 8 * mem::size_of::<T>(),
        label = label,
    ).ok();
}

fn busy_sleep(iterations: usize) {
    let n = 42;
    for _ in 0 .. 15 * iterations {
        unsafe {
            read_volatile(&n);
        }
    }
}

fn u32_to_rgb(n: u32) -> [u8; 3] {
    let c = n.to_be_bytes();
    [c[0], c[1], c[2]]
}
