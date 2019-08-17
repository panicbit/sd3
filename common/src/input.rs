use core::ptr::read_volatile;

bitflags! {
    pub struct PadState: u16 {
        const A = 1 << 0;
        const B = 1 << 1;
        const SELECT = 1 << 2;
        const START = 1 << 3;
        const RIGHT = 1 << 4;
        const LEFT = 1 << 5;
        const UP = 1 << 6;
        const DOWN = 1 << 7;
        const R = 1 << 8;
        const L = 1 << 9;
        const X = 1 << 10;
        const Y = 1 << 11;
    }
}

impl PadState {
    pub fn read() -> Self {
        let pad = 0x10146000 as *const u16;
        let state = unsafe { read_volatile(pad) };
        // When read: 0 = pressed, 1 = pressed
        // Hence the Not
        let state = !state;
        
        PadState::from_bits_truncate(state)
    }
}
