pub mod input_flags;

pub mod camera;

mod job;

use bitflags::Flags;
use cgmath::{Deg, Matrix4, Vector2, Vector3, Zero};

use self::{
    camera::structs::FlyingEye,
    input_flags::{InputFlags, Inputs},
};
use crate::black_sheep::q_i_square_root::q_normalize;

use super::settings::*;

pub struct GameState {
    pub inputs: Inputs,
    pub window_size_f32: [f32; 2],
    pub window_size_i32: [i32; 2],
    pub ui_projection: Matrix4<f32>,
    pub world_projection: Matrix4<f32>,
    pub cam: FlyingEye,
    pub up_pressed: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            inputs: Default::default(),
            window_size_f32: Default::default(),
            window_size_i32: Default::default(),
            ui_projection: Matrix4::zero(),
            world_projection: Matrix4::zero(),
            cam: Default::default(),
            up_pressed: Default::default(),
        }
    }
}

impl GameState {
    pub fn new() -> Self {
        let ui_projection = cgmath::ortho(
            0.0,
            INIT_WINDOW_SIZE_F32[0],
            INIT_WINDOW_SIZE_F32[1],
            0.0,
            -1.0,
            1.0,
        );
        let aspect = (INIT_WINDOW_SIZE_F32[0] - 300.0) / INIT_WINDOW_SIZE_F32[1];
        let world_projection = cgmath::perspective(Deg(120.0), aspect, 0.1, 1000.0);
        let mut cam = FlyingEye::new();
        cam.move_cam(Vector3::new(0.0, 00.0, 60.0));

        GameState {
            inputs: Inputs::default(),
            window_size_f32: INIT_WINDOW_SIZE_F32,
            window_size_i32: INIT_WINDOW_SIZE_I32,
            ui_projection,
            world_projection,
            cam,
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        self.cam.update();

        if let Some(v) = get_movement(&mut self.inputs.input_flags) {
            self.cam.set_movement(v);
        } else {
            self.cam.reset_movement();
        }
    }

    pub fn on_mouse_motion(&mut self, xrel: i32, yrel: i32, x: i32, y: i32) {
        let _v = Vector2::new(x as f32, y as f32);
        //self.structogram.update(v);

        if self.inputs.key_down(InputFlags::CAPTURED_MOUSE) {
            if xrel != 0 {
                self.cam.rotate_v(Deg(xrel as f32 / 10.0));
            }
            if yrel != 0 {
                self.cam.rotate_h(Deg(yrel as f32 / 10.0));
            }
        }
    }
}

const CAM_SPEED: f32 = 10.0;

pub fn get_movement(input: &mut InputFlags) -> Option<Vector3<f32>> {
    use InputFlags as kf;

    if input.bits() == kf::NONE.bits() {
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
        Some(q_normalize(v) * CAM_SPEED)
    }
}
