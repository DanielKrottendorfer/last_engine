use bitflags::bitflags;
use cgmath::{Vector3, Zero};
use sdl2::keyboard::Keycode;

use crate::black_sheep::q_i_square_root::q_normalize;

bitflags! {
    #[derive(Default)]
    pub struct GameFlags: u32 {
        const CAPTURED_MOUSE = 0b00000001;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct KeyboardInputFlags: u32 {
        const NONE =    0b0;
        const W =       0b1;
        const S =       0b10;
        const D =       0b100;
        const A =       0b1000;
        const X =       0b10000;
        const Y =       0b100000;
        const E =       0b1000000;
        const Q =       0b10000000;
        const RIGHT =   0b100000000;
        const LEFT =    0b1000000000;
        const UP =      0b10000000000;
        const DOWN  =   0b100000000000;
        const WS = Self::W.bits |  Self::S.bits;
    }
}

pub fn get_movement(input: &mut KeyboardInputFlags) -> Option<Vector3<f32>> {
    use KeyboardInputFlags as kf;

    if *input == kf::NONE {
        None
    } else {
        let mut v = Vector3::zero();
        if input.contains(kf::W) {
            v += Vector3::new(0.0, 0.0, -1.0)
        }
        if input.contains(kf::S) {
            v += Vector3::new(0.0, 0.0, 1.0)
        }
        if input.contains(kf::D) {
            v += Vector3::new(1.0, 0.0, 0.0)
        }
        if input.contains(kf::A) {
            v += Vector3::new(-1.0, 0.0, 0.0)
        }
        if input.contains(kf::X) {
            v += Vector3::new(0.0, 1.0, 0.0)
        }
        if input.contains(kf::Y) {
            v += Vector3::new(0.0, -1.0, 0.0)
        }
        Some(q_normalize(v))
    }
}

pub fn key_down(key_code: sdl2::keyboard::Keycode, input: &mut KeyboardInputFlags) {
    use KeyboardInputFlags as kf;
    match key_code {
        Keycode::E => input.insert(kf::E),
        Keycode::Q => input.insert(kf::Q),
        Keycode::W => input.insert(kf::W),
        Keycode::S => input.insert(kf::S),
        Keycode::D => input.insert(kf::D),
        Keycode::A => input.insert(kf::A),
        Keycode::X => input.insert(kf::X),
        Keycode::Y => input.insert(kf::Y),
        Keycode::Right => input.insert(kf::RIGHT),
        Keycode::Left => input.insert(kf::LEFT),
        Keycode::Up => input.insert(kf::UP),
        Keycode::Down => input.insert(kf::DOWN),
        _ => (),
    }
}

pub fn key_up(key_code: sdl2::keyboard::Keycode, input: &mut KeyboardInputFlags) {
    use KeyboardInputFlags as kf;
    match key_code {
        Keycode::E => input.remove(kf::E),
        Keycode::Q => input.remove(kf::Q),
        Keycode::W => input.remove(kf::W),
        Keycode::S => input.remove(kf::S),
        Keycode::D => input.remove(kf::D),
        Keycode::A => input.remove(kf::A),
        Keycode::X => input.remove(kf::X),
        Keycode::Y => input.remove(kf::Y),
        Keycode::Right => input.remove(kf::RIGHT),
        Keycode::Left => input.remove(kf::LEFT),
        Keycode::Up => input.remove(kf::UP),
        Keycode::Down => input.remove(kf::DOWN),
        _ => (),
    }
}
