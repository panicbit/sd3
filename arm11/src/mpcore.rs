use common::util::reg::*;
use common::mem::arm11::*;

pub mod interrupt_distributor;

#[inline(always)]
pub unsafe fn enable_scu() {
    asm!("
        ldr $0, =0x17E00000
        ldr $1, [$0]
        orr $1, 1
        str $1, [$0]
    " ::: "r0", "r1", "memory");
}

#[inline(always)]
pub unsafe fn disable_interrupts() {
    asm!("cpsid aif");
}


#[inline(always)]
pub unsafe fn clean_and_invalidate_data_cache() {
    asm!("
        ldr $0, =0
        mcr p15, 0, $0, c7, c14, 0
    " ::: "r0");
}

#[inline(always)]
pub unsafe fn enable_smp_mode() {
    asm!("
        mrc p15, 0, $0, c1, c0, 1
        orr r0, 0b10000
        mcr p15, 0, $0, c1, c0, 1
    " ::: "r0")
}

#[inline(always)]
pub unsafe fn control_register() -> u32 {
    let ctrl: u32;

    asm!("
        mrc p15, 0, $0, c1, c0, 0"
        : "=r"(ctrl)
    );

    ctrl
}

#[inline(always)]
pub fn cpu_id() -> u32 {
    let mut id: u32;

    unsafe {
        asm!(
            "mrc p15, 0, $0, c0, c0, 5"
            : "=r"(id)
        );
    }

    id
}

// #[inline(always)]
// pub unsafe fn enable_cpu1() {
//     asm!("
//         ldr $0, =0x17E00008
//         ldr $1, [$0]
//         orr $1, 0b00
//         str $1, [$0]
//     " ::: "r0", "r1", "memory");
// }


#[inline(always)]
pub unsafe fn cpu_status() -> u32 {
    let mut status: u32;

    asm!("
        ldr $1, =0x17E00008
        ldr $0, [$1]
    " : "=r"(status) :: "r1");

    status
}

#[inline(always)]
pub unsafe fn cpu_status_reg() -> u32 {
    let mut status: u32;

    asm!(
        "mrs $0, cpsr"
        : "=r"(status)
    );

    status
}

pub unsafe fn set_software_interrupt_handler(fun: unsafe extern fn() -> !) {
    RW::<usize>::new(AXI_WRAM.end - 0x60 + 0x10).write(fun as usize);
}

#[inline(always)]
pub unsafe fn software_interrupt() {
    asm!("swi 0")
}