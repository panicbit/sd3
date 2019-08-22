use common::util::reg::*;
use common::mem::arm11::*;

pub unsafe fn install_handlers() {
    let base = AXI_WRAM.end - 0x60;
    let mut vectors = RW::new(base);
    vectors.write(exception_vectors_template);
}

#[no_mangle]
#[naked]
pub unsafe extern fn irq_handler() {
    set_stack_pointer(0x24000000);
    panic!("irq");
}

#[no_mangle]
#[naked]
pub unsafe extern fn fiq_handler() {
    set_stack_pointer(0x24000000);
    panic!("fiq");
}

#[no_mangle]
#[naked]
pub unsafe extern fn svc_handler() {
    set_stack_pointer(0x24000000);

    let spsr: u32;
    asm!("msr spsr, $0" : "=r"(spsr));

    // TODO: check for thumb state first
    let mut svc: u32;
    asm!("ldr $0, [lr, #-4]" : "=r"(svc));

    svc &= (1<<24)-1; // extract low 24 bits (service number)

    panic!("\
        svc: {svc}\n\
        spsr: 0b{spsr:032b}\n\
        ",
        svc = svc,
        spsr = spsr,
    );
}

#[no_mangle]
#[naked]
pub unsafe extern fn undefined_instruction_handler() {
    set_stack_pointer(0x24000000);
    panic!("undefined instruction");
}

#[no_mangle]
#[naked]
pub unsafe extern fn prefetch_abort_handler() {
    set_stack_pointer(0x24000000);
    panic!("prefetch abort");
}

#[no_mangle]
#[naked]
pub unsafe extern fn data_abort_handler() {
    set_stack_pointer(0x24000000);
    panic!("data abort");
}

#[inline(always)]
pub unsafe fn set_stack_pointer(sp: usize) {
    asm!("mov sp, $0" :: "r"(sp))
}

extern {
    static mut exception_vectors_template: [[u32; 2]; 6];
}

global_asm!(r#"
.section .text
.align 4
.arm

exception_vectors_template:
    @ irq
    ldr pc, [pc, #-4]
    .word irq_handler

    @ fiq
    ldr pc, [pc, #-4]
    .word fiq_handler

    @ svc
    ldr pc, [pc, #-4]
    .word svc_handler

    @ undefined instruction
    ldr pc, [pc, #-4]
    .word undefined_instruction_handler

    @ prefetch abort
    ldr pc, [pc, #-4]
    .word prefetch_abort_handler

    @ data abort
    ldr pc, [pc, #-4]
    .word data_abort_handler
"#);
