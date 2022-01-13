
use bitflags::bitflags;
use sdl2::keyboard::Keycode;

bitflags! {
    #[derive(Default)]
    pub struct InputFlags: u32 {
        const NONE =            0b0;
        const CLOSE =           0b1;
        const W =               0b10;
        const S =               0b100;
        const D =               0b1000;
        const A =               0b10000;
        const X =               0b100000;
        const Y =               0b1000000;
        const E =               0b10000000;
        const Q =               0b100000000;
        const RIGHT =           0b1000000000;
        const LEFT =            0b10000000000;
        const UP =              0b100000000000;
        const DOWN  =           0b1000000000000;
        const CAPTURED_MOUSE =  0b10000000000000;
        const WS = Self::W.bits |  Self::S.bits;
    }
}

impl From<Keycode> for InputFlags {
    fn from(key_code: Keycode) -> Self {
        use InputFlags as kf;
        match key_code {
            Keycode::E => kf::E,
            Keycode::Q => kf::Q,
            Keycode::W => kf::W,
            Keycode::S => kf::S,
            Keycode::D => kf::D,
            Keycode::A => kf::A,
            Keycode::X => kf::X,
            Keycode::Y => kf::Y,
            Keycode::Right => kf::RIGHT,
            Keycode::Left => kf::LEFT,
            Keycode::Up => kf::UP,
            Keycode::Down => kf::DOWN,
            _ => kf::NONE,
        }
    }
}

impl InputFlags {
    pub fn key_down(&mut self,key_code: sdl2::keyboard::Keycode) {
        use InputFlags as kf;
        self.insert(kf::from(key_code));
    }
    pub fn key_up(&mut self,key_code: sdl2::keyboard::Keycode) {
        use InputFlags as kf;
        self.remove(kf::from(key_code));
    }
    pub fn close(&self) -> bool {
        self.bits() & InputFlags::CLOSE.bits() == InputFlags::CLOSE.bits()
    }
}
