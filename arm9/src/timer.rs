use volatile::Volatile;

#[repr(C)]
struct Timer {
    value: Volatile<u16>,
    control: Volatile<u16>,
}

const TICKS_PER_SEC: u64 = 67_027_964;

unsafe fn timers() -> &'static mut [Timer; 4] {
    &mut *(0x10003000 as *mut [Timer; 4])
}

fn start() -> u64 {
    static mut timer_needs_init: bool = true;

    let timers = unsafe { timers() };

    if unsafe { timer_needs_init } {
        for timer in &mut *timers {
            timer.control.write(0);
            timer.value.write(0);
        }

        unsafe {
            timer_needs_init = false;
        }
    }

    timers[0].control.write(0b1_0_000_0_00);

    for timer in &mut timers[1..] {
        timer.control.write(0b1_0_000_1_00);
    }

    elapsed_since(0)
}

fn elapsed_since(start_time: u64) -> u64 {
    let timers = unsafe { timers() };
    let mut ticks: u64 = 0;
    ticks |= (timers[0].value.read() as u64) << 0;
    ticks |= (timers[1].value.read() as u64) << 16;
    ticks |= (timers[2].value.read() as u64) << 32;
    ticks |= (timers[3].value.read() as u64) << 48;
    ticks - start_time
}

fn msecs_since(start_time: u64) -> u64 {
    elapsed_since(start_time) / (TICKS_PER_SEC / 1000)
}

pub fn sleep_msecs(msecs: u64) {
    let start = start();
    while msecs_since(start) < msecs {}
}
