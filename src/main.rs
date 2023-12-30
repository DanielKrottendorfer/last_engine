#![feature(trait_alias)]

use std::borrow::Borrow;

use crate::black_sheep::{
    gamestate::{self, camera::structs::FlyingEye, GameState, input_flags::{self, InputFlags}},
    imgui_system::ImguiSystem,
    rendering::{self, geometry::mesh::MeshToken, loader::load_texture_from_path, rendertarget},
    settings::DT,
    window::window_util::{clear_color, clear_drawbuffer, set_viewport, three_d_rendering_setup},
};
use black_sheep::{DrawFunction, UpdateFunction};
use cgmath::{vec2, vec3, Deg, InnerSpace, Matrix4, Vector2, Vector3, Zero};
use imgui::{ColorPicker, Condition, Image, TextureId, Window};

mod black_sheep;
mod gameplay;

extern crate cgmath;
extern crate gl;
extern crate rand;
extern crate sdl2;

#[macro_use]
extern crate lazy_static;

fn main() {
    #[cfg(not(feature = "debug_off"))]
    println!("Hello, world!");

    black_sheep::run(|ecs| {
        gameplay::gen_apes(ecs);

        let rt_gizmo = rendering::rendertarget::RenderTarget::new(300, 300);
        rendertarget::unbind_framebuffer();

        ecs.add_ball_soa(Vector2::new(5.0, -5.0), Vector2::zero());

        let mut circle = ecs.get_circle_accessor();
        let positions = ecs.get_positions_accessor();
        let mut pos_update = ecs.get_update_pos_ori_accessor();


        let mut t_color = [0.0, 0.0, 0.0, 0.0];
        let update = move |game_state: &mut GameState, imgui_system: &mut ImguiSystem| {
            {
                let mut update = pos_update.lock();
                for (pos, ori, direction, target_ori) in update.iter() {
                    *pos = *pos + *direction;
                    *ori = *target_ori;
                }
            }
            gameplay::run_ape_ai(&mut circle, &positions);

            if game_state.inputs.key_pressed(InputFlags::UP) {
                game_state.up_pressed = !game_state.up_pressed;
            }

            imgui_system.update(&mut |ui| {
                use imgui::WindowFlags;


                Window::new("Image")
                    .size([300.0, game_state.window_size_f32[1]], Condition::Always)
                    .position(
                        [game_state.window_size_f32[0] - 300.0, 0.0],
                        Condition::Always,
                    )
                    .flags(
                        WindowFlags::NO_MOVE
                            | WindowFlags::NO_RESIZE
                            | WindowFlags::NO_COLLAPSE
                            | WindowFlags::NO_TITLE_BAR,
                    )
                    .build(&ui, || {
                        ui.text("Hello world!");
                        ui.text("こんにちは世界！");
                        ui.input_float("tt", &mut game_state.tt).build();
                        ui.input_float("aa", &mut game_state.aa).build();
                        ui.text(format!("{:?}", -game_state.cam.position));
                        ui.text(format!("{:#?}", game_state.cam.orientation));
                        ColorPicker::new("color_picker", &mut t_color).build(ui);
                        Image::new(TextureId::new(2 as usize), [300.0, 300.0])
                            .uv0([0.0, 1.0])
                            .uv1([1.0, 0.0])
                            .build(ui);
                        Image::new(TextureId::new(1 as usize), [300.0, 300.0]).build(ui);
                    });
            });
        };

        let draw_m = ecs.get_draw_accessor();

        let mut calc_mat = ecs.get_calculate_mat_accessor();

        //let mut c_vec = Vec::new();
        //let mut simulate = ecs.get_simulate_accessor();

        let _bb = black_sheep::setup::init_mesh().unwrap();

        let (ape, torus, sprite) = black_sheep::rendering::geometry::get_mesh_repo(|mr| {
            let ape = MeshToken::from(mr.get_mesh_by_name("ape").unwrap());
            let torus = MeshToken::from(mr.get_mesh_by_name("torus").unwrap());
            let sprite = MeshToken::from(mr.get_mesh_by_name("sprite").unwrap());
            //let circles = MeshToken::from(mr.get_mesh_by_name("circles").unwrap());
            (ape, torus, sprite)
        });

        let rendering = black_sheep::rendering::shader::get_shader_repo();

        let three_d = rendering.color_3d;
        let three_dl = rendering.color_3d_light;
        let _circles_2d = rendering.point_2d;
        let sprite_shader = rendering.sprite;
        let doubl_sphere = rendering.double_sphere;

        let nice_image = load_texture_from_path("./res/1322615842122.jpg").unwrap();
        let mut last = None;

        let mut f = true;

        let draw = move |i: f32, game_state: &GameState| {

            let cam = game_state.cam.borrow();
            let prj = game_state.world_projection;

            let view = cam.get_i_view(i);
            let vp = prj * view;
            for (p, o, direction, to, model) in calc_mat.lock().iter() {
                let q = o.slerp(*to, i);
                let v = p + (direction * i);
                last = Some(v.clone());
                let mut m = Matrix4::from(q);
                m.w = v.extend(1.0);
                *model = m;
            }

            let d_lock = draw_m.lock();


            ape.bind_vertex_array();

            let three_d = || {

                doubl_sphere.use_program();
                doubl_sphere.set_light_position(vec3(30.0, 30.0, 10.0));
                doubl_sphere.set_light_power(3000.0);
                doubl_sphere.set_aa(game_state.aa);
                doubl_sphere.set_tt(game_state.tt);

                for (m, c) in d_lock.iter() {
                    doubl_sphere.set_M(view * m);
                    doubl_sphere.set_col(*c);
                    ape.draw_triangle_elements();
                }
            };
            let doubele_l = || {
                three_dl.use_program();
                three_dl.set_light_position(vec3(30.0, 30.0, 10.0));
                three_dl.set_light_power(3000.0);

                let vp = cgmath::perspective(Deg(80.0), 1.0, 0.1, 1000.0) * view;
                for (m, c) in d_lock.iter() {
                    three_dl.set_MVP(vp * m);
                    three_dl.set_M(*m);
                    three_dl.set_col(*c);
                    ape.draw_triangle_elements();
                }
            };

            


            if game_state.up_pressed {
                three_d();
            }else{
                doubele_l();
            }

            rt_gizmo.bind_framebuffer();
            three_d_rendering_setup();
            clear_color(0.0, 0.3, 0.3, 1.0);
            clear_drawbuffer();
            set_viewport(300, 300);

            if !f{
                three_d();
            }else{
                doubele_l();
            }
            rendertarget::unbind_framebuffer();

            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + 2);
                rt_gizmo.bind_texture();
            }

            // rendertarget::unbind_framebuffer();

            // three_d.use_program();
            // three_d.set_MVP(vp);
            // three_d.set_col(Vector3::new(1.0, 0.0, 1.0));

            // torus.bind_vertex_array();
            // torus.draw_line_elements();

            // sprite_shader.use_program();
            // unsafe { gl::ActiveTexture(gl::TEXTURE0 + 1) };
            // nice_image.bind();
            // sprite_shader.set_CameraRight_worldspace(vec3(view.x.x, view.y.x, view.z.x));
            // sprite_shader.set_CameraUp_worldspace(vec3(view.x.y, view.y.y, view.z.y));
            // sprite_shader.set_VP(vp);
            // sprite_shader.set_BillboardPos(last.unwrap() + vec3(2.0,2.0,2.0));
            // sprite_shader.set_BillboardSize(vec2(2.0, 2.0));
            // sprite_shader.set_myTextureSampler(1);
            // sprite.bind_vertex_array();
            // sprite.draw_triangle_elements();
        };
        black_sheep::Logic { update, draw }
    });
}
