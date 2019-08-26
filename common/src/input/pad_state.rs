
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

    pub fn a(&self) -> bool {
        self.contains(Self::A)
    }

    pub fn b(&self) -> bool {
        self.contains(Self::B)
    }

    pub fn select(&self) -> bool {
        self.contains(Self::SELECT)
    }

    pub fn start(&self) -> bool {
        self.contains(Self::START)
    }

    pub fn right(&self) -> bool {
        self.contains(Self::RIGHT)
    }

    pub fn left(&self) -> bool {
        self.contains(Self::LEFT)
    }

    pub fn up(&self) -> bool {
        self.contains(Self::UP)
    }

    pub fn down(&self) -> bool {
        self.contains(Self::DOWN)
    }

    pub fn r(&self) -> bool {
        self.contains(Self::R)
    }

    pub fn l(&self) -> bool {
        self.contains(Self::L)
    }

    pub fn x(&self) -> bool {
        self.contains(Self::X)
    }

    pub fn y(&self) -> bool {
        self.contains(Self::Y)
    }
}
