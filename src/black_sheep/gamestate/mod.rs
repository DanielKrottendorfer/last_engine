pub mod input_flags;

pub mod camera;

use cgmath::{Deg, Matrix4, Rad, Vector2, Vector3, Zero};
use imgui::{Selectable, Ui};

use self::{camera::structs::FlyingEye, input_flags::InputFlags};
use crate::black_sheep::q_i_square_root::q_normalize;

use super::{
    constants::TRI_TABLE,
    rendering::{
        self, geometry::MeshToken, loader::gen_isampler_texture, shader::shader_structs::*, Texture,
    },
    settings::*,
    setup,
    window::window_util::*,
};

pub struct GameState {
    pub input_flags: InputFlags,
    pub window_size_f32: [f32; 2],
    pub window_size_i32: [i32; 2],
    pub ui_projection: Matrix4<f32>,
    pub world_projection: Matrix4<f32>,
    pub cam: FlyingEye,

    voxel: VoexelProgram,
    voxel_norm: VoexelNormProgram,
    mesh_ts: Vec<MeshToken>,
    textures: Vec<Texture>,
    rot: f32, //structogram: Structogram,
    g: f32,
    r: f32,
    normals: bool,
    rotate: bool,
    wiregird: bool,
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
        let world_projection = cgmath::perspective(Deg(90.0), aspect, 0.1, 1000.0);
        let mut cam = FlyingEye::new();
        cam.move_cam(Vector3::new(-0.6, 0.5, 0.25));
        cam.rotate_h(Deg(45.0));
        cam.rotate_v(Deg(65.0));

        let shader_repo = rendering::shader::get_shader_repo();
        let voxel = shader_repo.voxel;
        let voxel_norm = shader_repo.voxel_norm;

        let mesh_ts = setup::init_mesh();
        let t = Texture::new(gen_isampler_texture(16, 256, TRI_TABLE.as_slice()));
        let textures = vec![t];

        GameState {
            input_flags: InputFlags::NONE,
            window_size_f32: INIT_WINDOW_SIZE_F32,
            window_size_i32: INIT_WINDOW_SIZE_I32,
            ui_projection,
            world_projection,
            cam,
            voxel,
            voxel_norm,
            mesh_ts,
            textures,
            rot: 0.0, //structogram,
            g: 0.15,
            r: 0.3,
            normals: false,
            rotate: false,
            wiregird: false,
        }
    }

    pub fn update(&mut self) {
        if self.rotate {
            self.rot += 0.01;
        }
        self.cam.update();

        if let Some(v) = get_movement(&mut self.input_flags) {
            self.cam.set_movement(v);
        } else {
            self.cam.reset_movement();
        }
    }

    pub fn draw_3d(&mut self, i: f32) {
        let view = self.cam.get_i_view(i);

        clear_color(0.0, 0.3, 0.3, 1.0);
        clear_drawbuffer();

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + 0);
            self.textures[0].bind();
        }

        let voxel_grid = &self.mesh_ts[2];
        voxel_grid.bind_vertex_array();

        let m = Matrix4::from_angle_y(Rad(self.rot));

        if self.normals {
            self.voxel_norm.use_program();
            self.voxel_norm.set_v(view);
            self.voxel_norm.set_m(m);
            self.voxel_norm.set_projection(self.world_projection);
            self.voxel_norm.set_triTableTex(0);
            self.voxel_norm.set_voxel_size(0.01);
            self.voxel.set_G(self.g);
            self.voxel.set_R(self.r);

            voxel_grid.draw_point_elements();
        }
        self.voxel.use_program();
        self.voxel.set_v(view);
        self.voxel.set_m(m);
        self.voxel.set_projection(self.world_projection);
        self.voxel.set_triTableTex(0);
        self.voxel.set_voxel_size(0.01);
        self.voxel.set_gEyeWorldPos(self.cam.position);
        self.voxel.set_G(self.g);
        self.voxel.set_R(self.r);

        voxel_grid.draw_point_elements();
    }

    pub fn draw_ui(&mut self, ui: &Ui) {
        use imgui::Slider;

        let sr = Slider::new("R", 0.0, 1.0);
        sr.build(ui, &mut self.r);
        let sg = Slider::new("G", 0.0, 1.0);
        sg.build(ui, &mut self.g);

        if Selectable::new(if self.normals {
            "hide normals"
        } else {
            "show normals"
        })
        .build(ui)
        {
            self.normals = !self.normals;
        }

        if Selectable::new(if self.rotate {
            "stop rotationg"
        } else {
            "rotate"
        })
        .build(ui)
        {
            self.rotate = !self.rotate;
        }

        if Selectable::new(if self.wiregird {
            "turn off wiregrid"
        } else {
            "turn on wiregrid"
        })
        .build(ui)
        {
            self.wiregird = !self.wiregird;
            toggle_wiregrid(self.wiregird);
        }
    }

    pub fn on_mouse_motion(&mut self, xrel: i32, yrel: i32, x: i32, y: i32) {
        let _v = Vector2::new(x as f32, y as f32);
        //self.structogram.update(v);

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
