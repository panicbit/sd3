use super::PadState;

pub struct GamePad {
    previous: PadState,
    current: PadState,
}

impl GamePad {
    pub fn new() -> GamePad {
        let state = PadState::read();

        GamePad {
            previous: state,
            current: state,
        }
    }

    pub fn poll(&mut self) {
        self.previous = self.current;
        self.current = PadState::read();
    }

    fn once(&self, state: PadState) -> bool {
        !self.previous.contains(state) &&
         self.current.contains(state)
    }

    fn continuous(&self, state: PadState) -> bool {
        self.current.contains(state)
    }

    pub fn a(&self) -> bool {
        self.continuous(PadState::A)
    }

    pub fn a_once(&self) -> bool {
        self.once(PadState::A)
    }

    pub fn b(&self) -> bool {
        self.continuous(PadState::B)
    }

    pub fn b_once(&self) -> bool {
        self.once(PadState::B)
    }

    pub fn select(&self) -> bool {
        self.continuous(PadState::SELECT)
    }

    pub fn select_once(&self) -> bool {
        self.once(PadState::SELECT)
    }

    pub fn start(&self) -> bool {
        self.continuous(PadState::START)
    }

    pub fn start_once(&self) -> bool {
        self.once(PadState::START)
    }

    pub fn right(&self) -> bool {
        self.continuous(PadState::RIGHT)
    }

    pub fn right_once(&self) -> bool {
        self.once(PadState::RIGHT)
    }

    pub fn left(&self) -> bool {
        self.continuous(PadState::LEFT)
    }

    pub fn left_once(&self) -> bool {
        self.once(PadState::LEFT)
    }

    pub fn up(&self) -> bool {
        self.continuous(PadState::UP)
    }

    pub fn up_once(&self) -> bool {
        self.once(PadState::UP)
    }

    pub fn down(&self) -> bool {
        self.continuous(PadState::DOWN)
    }

    pub fn down_once(&self) -> bool {
        self.once(PadState::DOWN)
    }

    pub fn r(&self) -> bool {
        self.continuous(PadState::R)
    }

    pub fn r_once(&self) -> bool {
        self.once(PadState::R)
    }

    pub fn l(&self) -> bool {
        self.continuous(PadState::L)
    }

    pub fn l_once(&self) -> bool {
        self.once(PadState::L)
    }

    pub fn x(&self) -> bool {
        self.continuous(PadState::X)
    }

    pub fn x_once(&self) -> bool {
        self.once(PadState::X)
    }

    pub fn y(&self) -> bool {
        self.continuous(PadState::Y)
    }

    pub fn y_once(&self) -> bool {
        self.once(PadState::Y)
    }
}
