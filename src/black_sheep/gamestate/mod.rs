
pub mod input_flags;

pub mod camera;

use cgmath::{Matrix4, Vector3, Zero, Deg};


use crate::black_sheep::q_i_square_root::q_normalize;
use self::{input_flags::InputFlags, camera::structs::FlyingEye};

use super::{rendering::{shader::shader_structs::*, self, geometry::{MeshToken}}, settings::*, window::window_util::*, setup};


pub struct GameState {
    pub input_flags: InputFlags,
    pub window_size_f32: [f32; 2],
    pub window_size_i32: [i32; 2],
    pub ui_projection: Matrix4<f32>,
    pub cam: FlyingEye,

    color_shader: Color3D,
    cloud_shader: CloudGeometryShaderProgram,
    
    mesh_ts: Vec<MeshToken>
}


impl GameState {
    pub fn new() -> Self {
        let ui_projection = ui_projection_mat([INIT_WINDOW_SIZE_I32[0], INIT_WINDOW_SIZE_I32[1]]);
        let mut cam = FlyingEye::new();
        cam.move_cam(Vector3::new(1.35, 1.35, 2.0));
        cam.rotate_h(Deg(35.0));

        let shader_repo = rendering::shader::get_shader_repo();
        let color_shader = shader_repo.color_3d;
        let cloud_shader = shader_repo.point_cloud;

        
        let mesh_ts = setup::init_mesh();

        GameState {
            input_flags: InputFlags::NONE,
            window_size_f32: INIT_WINDOW_SIZE_F32,
            window_size_i32: INIT_WINDOW_SIZE_I32,
            ui_projection,
            cam,
            color_shader,
            cloud_shader,

            mesh_ts
        }
    }

    pub fn update(&mut self) {
        
        self.cam.update();
        
        if let Some(v) = get_movement(&mut self.input_flags) {
            self.cam.set_movement(v);
        } else {
            self.cam.reset_movement();
        }
    }

    pub fn draw(&mut self, i:f32) {

        let view = self.cam.get_i_view(i);
        let aspect = (self.window_size_f32[0] - 300.0) / self.window_size_f32[1];
        let projection = cgmath::perspective(Deg(90.0), aspect, 0.1, 1000.0);

        let model = Matrix4::from_translation(Vector3::new(1.2, 0.0, 0.0));

        clear_color(0.0, 0.3, 0.3, 1.0);
        clear_drawbuffer();

        let cube = &self.mesh_ts[2];
        self.color_shader.use_program();
        self.color_shader.set_MVP(projection * view * model);
        cube.bind_vertex_array();
        cube.draw_triangle_elements();

        let cube_cloud = &self.mesh_ts[3];
        self.cloud_shader.use_program();
        self.cloud_shader.set_mv(view);
        self.cloud_shader.set_projection(projection);
        cube_cloud.bind_vertex_array();
        cube_cloud.draw_point_elements();       
    }

    pub fn on_mouse_motion(&mut self,xrel: i32, yrel: i32){

        if self.input_flags.contains(InputFlags::CAPTURED_MOUSE) {
            if xrel != 0 {
                self.cam.rotate_v(Deg(xrel as f32 / 10.0));
            }
            if yrel != 0 {
                self.cam.rotate_h(Deg(yrel as f32 / 10.0));
            }
        }
    }
}


pub fn get_movement(input: &mut InputFlags) -> Option<Vector3<f32>> {
    use InputFlags as kf;

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
