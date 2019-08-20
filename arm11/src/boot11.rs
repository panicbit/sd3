use common::util::reg::*;
use crate::mpcore::interrupt_distributor::*;

const CPU_ENTRYPOINT: RW<unsafe extern fn() -> !> = RW::new(0x1FFFFFDC);

pub unsafe fn start_cpu(cpu: u8, entrypoint: unsafe extern fn() -> !) {
    CPU_ENTRYPOINT.write(entrypoint);

    let cpu = match cpu {
        0 => CPUSet::CPU0,
        1 => CPUSet::CPU1,
        2 => CPUSet::CPU2,
        3 => CPUSet::CPU3,
        _ => return,
    };

    InterruptDistributor::software_interrupt(SoftwareInterruptRequest {
        id: 1,
        target: InterruptTarget::Set(cpu),
    });
}
