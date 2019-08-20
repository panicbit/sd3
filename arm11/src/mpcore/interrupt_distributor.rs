use common::mem::arm11::PRIVATE_MEM;
use common::util::reg::*;

pub struct InterruptDistributor;

impl InterruptDistributor {
    const ADDR: usize = PRIVATE_MEM.start + 0x1000;
    const SOFTWARE_INTERRUPT: WO<u32> = WO::new(Self::ADDR + 0xF00);

    pub unsafe fn software_interrupt(request: SoftwareInterruptRequest) {
        let id = (request.id as u32) & 0b1_1111_1111;
        let mut cpu_target_list = 0;
        let target_filter = match request.target {
            InterruptTarget::Set(cpus) => {
                cpu_target_list = cpus.bits() as u32;
                0b00
            },
            InterruptTarget::AllExceptSender => 0b01,
            InterruptTarget::Sender => 0b10,
        };

        let request =
              id << 0
            | cpu_target_list << 16
            | target_filter << 24;

        Self::SOFTWARE_INTERRUPT.write(request);
    }
}

#[derive(Copy, Clone)]
pub struct SoftwareInterruptRequest {
    pub id: u16,
    pub target: InterruptTarget,
}

#[derive(Copy, Clone)]
pub enum InterruptTarget {
    Sender,
    AllExceptSender,
    Set(CPUSet),
}

bitflags! {
    pub struct CPUSet: u8 {
        const CPU0 = 0b0001;
        const CPU1 = 0b0010;
        const CPU2 = 0b0100;
        const CPU3 = 0b1000;
    }
}
