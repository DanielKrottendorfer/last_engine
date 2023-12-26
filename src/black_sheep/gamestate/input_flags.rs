use bitflags::bitflags;
use sdl2::keyboard::Keycode;

#[derive(Default)]
pub struct Inputs {
    pub input_flags: InputFlags,
    last_input_flags: InputFlags,
}

impl Inputs {
    pub fn key_pressed(&self, input: InputFlags) -> bool {
        println!("{} {}",self.input_flags.bits(), self.last_input_flags.bits());
        (!self.last_input_flags.contains(input)) && self.input_flags.contains(input)
    }
    pub fn key_released(&self, input: InputFlags) -> bool {
        self.last_input_flags.contains(input) && (!self.input_flags.contains(input)) 
    }
    pub fn key_down(&self, input: InputFlags) -> bool {
        self.input_flags.contains(input)
    }
    pub fn close(&self) -> bool {
        self.input_flags.bits() & InputFlags::CLOSE.bits() == InputFlags::CLOSE.bits()
    }
    pub fn add_inputs<F:FnMut(&mut InputFlags)>(&mut self, mut f: F) {
        self.last_input_flags.0 = self.input_flags.0;
        f(&mut self.input_flags);
    }
}

bitflags! {
    #[derive(Default,Clone, Copy)]
    pub struct InputFlags: u32 {
        const NONE =            0b0;
        const CLOSE =           1 << 1;
        const W =               1 << 2;
        const S =               1 << 3;
        const D =               1 << 4;
        const A =               1 << 5;
        const X =               1 << 6;
        const Y =               1 << 7;
        const E =               1 << 8;
        const Q =               1 << 9;
        const RIGHT =           1 << 10;
        const LEFT =            1 << 11;
        const UP =              1 << 12;
        const DOWN  =           1 << 13;
        const CAPTURED_MOUSE =  1 << 14;
        const WS = Self::W.bits() | Self::A.bits();
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