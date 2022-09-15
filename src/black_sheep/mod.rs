pub mod rendering;
mod window;

mod algorithms;
#[allow(dead_code)]
mod constants;
mod gameplay;
pub mod gamestate;
mod generators;
mod imgui_system;
mod loop_timing;
pub mod math;
mod q_i_square_root;
mod script;
pub mod settings;
mod setup;
mod torus;
mod transform;

use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};

use cgmath::{Deg, InnerSpace, Matrix3, Quaternion, Rad, Rotation3, Vector2, Vector4, Zero};
use cgmath::{Matrix4, Vector3};
use gamestate::*;

use imgui::{ColorPicker, Condition, Image, TextureId, Window};
use sdl2::mouse::MouseButton;
use sdl2::render;
use window::window_util::*;
use window::SDLWindow;

use sdl2::event::{Event, WindowEvent};

use crate::black_sheep::rendering::loader::load_texture_from_path;
use crate::black_sheep::rendering::rendertarget;

use crate::black_sheep::window::window_util::{clear_drawbuffer, set_viewport};

use gamestate::input_flags::InputFlags;
use imgui_system::ImguiSystem;
use rendering::geometry;
use rendering::geometry::mesh::MeshToken;
use rendering::shader;

use camera::structs::FlyingEye;

use self::math::tetrahedral::Tetrahedral;
use self::settings::{DT, UPS_F32};

pub struct BlackSheep<U, D>
where
    U: FnMut(InputFlags),
    D: FnMut(f32, &FlyingEye, &Matrix4<f32>),
{
    window: SDLWindow,
    game_state: GameState<U, D>,
    rel_mouse_pos: Vector2<f32>,
}

impl<U: FnMut(InputFlags), D: FnMut(f32, &FlyingEye, &Matrix4<f32>)> Drop for BlackSheep<U, D> {
    fn drop(&mut self) {
        shader::cleanup();
        geometry::cleanup();
    }
}
pub fn run() {
    // KEEP THIS ORDER
    let window = SDLWindow::new();
    shader::init();
    geometry::init();

    let bb = setup::init_mesh().unwrap();

    let (ape, tetra, circles) = geometry::get_mesh_repo(|mr| {
        let ape = MeshToken::from(mr.get_mesh_by_name("ape").unwrap());
        let tetra = MeshToken::from(mr.get_mesh_by_name("tetra").unwrap());
        let circles = MeshToken::from(mr.get_mesh_by_name("circles").unwrap());
        (ape, tetra, circles)
    });

    let rendering = rendering::shader::get_shader_repo();

    let three_d = rendering.color_3d;
    let circles_2d = rendering.point_2d;
    let circle_3d = rendering.circle_point_cloud;

    let mut t = Tetrahedral::new(4.0);
    // let m = Matrix3::from_angle_z(Deg(20.0));
    t.0.iter_mut().for_each(|v| {
        *v += Vector3::unit_y() * 5.0;
        //*v = m * *v;
    });
    let c = t.get_constraints();

    let mut tet1 = Arc::new(Mutex::new(t));
    let mut tet_v1 = Arc::new(Mutex::new(Tetrahedral::zero()));
    let mut tet2 = tet1.clone();

    let game_state = GameState::new(
        |ecs| {
            gameplay::gen_apes(ecs);

            let rng = rand::thread_rng();

            ecs.add_ball_soa(
                Vector2::new(3.0, 0.0),
                Vector2::new(3.0, 0.0),
                Vector2::zero(),
            );
            ecs.add_ball_soa(
                Vector2::new(5.0, 0.0),
                Vector2::new(5.0, 0.0),
                Vector2::zero(),
            );
            ecs.add_ball_soa(
                Vector2::new(7.0, 0.0),
                Vector2::new(7.0, 0.0),
                Vector2::zero(),
            );
            ecs.add_ball_soa(
                Vector2::new(9.0, 0.0),
                Vector2::new(9.0, 0.0),
                Vector2::zero(),
            );

            let mut circle = ecs.get_circle_accessor();
            let positions = ecs.get_positions_accessor();
            let mut pos_update = ecs.get_update_pos_ori_accessor();

            let mut simulate = ecs.get_simulate_accessor();

            let g = Vector3::unit_y() * -10.0;

            let r = 3.0;

            move |_input| {
                // {
                //     let mut update = pos_update.lock();
                //     for (pos, ori, direction, target_ori) in update.iter() {
                //         *pos = *pos + *direction;
                //         *ori = *target_ori;
                //     }
                // }
                // gameplay::run_ape_ai(&mut circle, &positions);
                gameplay::run_pendulum(&mut simulate);

                let t = &mut *tet1.lock().unwrap();
                let v = &mut *tet_v1.lock().unwrap();

                let p = t.clone();
                for (x, v) in t.0.iter_mut().zip(v.0.iter_mut()) {
                    *v += g * DT;
                    *x += *v * DT;
                }

                gameplay::tetra_dist(t);
                gameplay::vol_c(t);
                gameplay::harddeck(t);

                for i in 0..4 {
                    let v = v.0.get_mut(i).unwrap();
                    let t = t.0.get_mut(i).unwrap();
                    let p = p.0.get(i).unwrap();

                    *v = (*t - *p) / DT;
                    if v.magnitude2() < 0.1 {
                        *v = Vector3::zero();
                    }
                }
            }
        },
        |ecs| {
            let draw_m = ecs.get_draw_accessor();

            let mut calc_mat = ecs.get_calculate_mat_accessor();

            let mut c_vec = Vec::new();
            let mut simulate = ecs.get_simulate_accessor();

            let mut tet = ecs.get_tets();

            move |i: f32, cam: &FlyingEye, prj: &Matrix4<f32>| {
                let view = cam.get_i_view(i);

                for (p, o, direction, to, model) in calc_mat.lock().iter() {
                    let q = o.slerp(*to, i);
                    let v = p + (direction * i);

                    let mut m = Matrix4::from(q);
                    m.w = v.extend(1.0);
                    *model = m;
                }

                let d_lock = draw_m.lock();

                clear_color(0.0, 0.3, 0.3, 1.0);
                clear_drawbuffer();

                
                circle_3d.use_program();
                circle_3d.set_mv(view);
                circle_3d.set_projection(*prj);
                
                tetra.bind_vertex_array();
                tetra.draw_point_array();

                three_d.use_program();
                three_d.set_MVP(prj * view);
                three_d.set_col(Vector3::unit_x());

                let t = tet2.lock().unwrap().clone();
                let te: [Vector3<f32>; 4] = t.into();
                geometry::get_mesh_repo(|mr| {
                    mr.get_mesh_by_uid(&tetra.uid)
                        .unwrap()
                        .update_buffer(&te, 0);
                });
                tetra.bind_vertex_array();
                tetra.draw_line_elements();


                // for (m, c) in d_lock.iter() {
                //     three_d.set_MVP(prj * view * m);
                //     three_d.set_col(*c);
                //     ape.draw_triangle_elements();
                // }

                // three_d.set_MVP(prj * view);
                // three_d.set_col(Vector3::new(1.0, 0.0, 1.0));

                // torus.bind_vertex_array();
                // torus.draw_line_elements();

                for c in simulate.lock().iter() {
                    c_vec.push(*c.0 + ((*c.2 * DT) * i));
                }
                geometry::get_mesh_repo(|mr| {
                    mr.get_mesh_by_uid(&circles.uid)
                        .unwrap()
                        .update_buffer(c_vec.as_slice(), 0);
                });
                c_vec.clear();

                circles_2d.use_program();
                let ortho = cgmath::ortho(-8.0, 8.0, -10.0, 6.0, -1.0, 1.0);
                circles_2d.set_projection(ortho);

                unsafe {
                    gl::Disable(gl::DEPTH_TEST);
                }

                circles.bind_vertex_array();
                circles.draw_point_elements();
            }
        },
    );

    let bs = BlackSheep {
        window,
        game_state,
        rel_mouse_pos: Vector2::new(0.0, 0.0),
    };

    bs.run();
}

impl<U, D> BlackSheep<U, D>
where
    U: FnMut(InputFlags),
    D: FnMut(f32, &FlyingEye, &Matrix4<f32>),
{
    pub fn handle_events(&mut self, imgui_system: &mut ImguiSystem) {
        while let Some(event) = self.window.poll_event() {
            imgui_system.handle_event(&event);
            let game_state = &mut self.game_state;
            match event {
                Event::Quit { .. } => {
                    game_state.input_flags.insert(InputFlags::CLOSE);
                }
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keycode {
                        use sdl2::keyboard::Keycode::*;
                        if let Escape = key {
                            game_state.input_flags.insert(InputFlags::CLOSE);
                        } else {
                            game_state.input_flags.key_down(key);
                        }
                    } else {
                        #[cfg(not(feature = "debug_off"))]
                        println!("No Valid KeyCode");
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = keycode {
                        game_state.input_flags.key_up(key);
                    } else {
                        #[cfg(not(feature = "debug_off"))]
                        println!("No Valid KeyCode");
                    }
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    if MouseButton::Right == mouse_btn {
                        game_state.input_flags.insert(InputFlags::CAPTURED_MOUSE);
                        self.window.capture_mouse();
                    }
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    if MouseButton::Right == mouse_btn {
                        game_state.input_flags.remove(InputFlags::CAPTURED_MOUSE);
                        self.window.release_mouse();
                    }
                }
                Event::MouseMotion {
                    xrel, yrel, x, y, ..
                } => {
                    self.rel_mouse_pos = Vector2::new(x as f32, y as f32);
                    self.game_state.on_mouse_motion(xrel, yrel, x, y);
                }
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(w, h) => {
                        set_viewport(w, h);
                        game_state.window_size_i32 = [w, h];
                        let wh = [w as f32, h as f32];
                        game_state.window_size_f32 = wh;

                        game_state.ui_projection = cgmath::ortho(0.0, wh[0], wh[1], 0.0, -1.0, 1.0);
                        let aspect = (wh[0] - 300.0) / wh[1];
                        game_state.world_projection =
                            cgmath::perspective(Deg(60.0), aspect, 0.1, 1000.0);
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }

    pub fn run(mut self) {
        let (imgui_shader_program, gizmo_shader) = {
            let shader_repo = rendering::shader::get_shader_repo();
            (shader_repo.imgui, shader_repo.gizmo)
        };

        init_rendersetup();

        let mut imgui_system = imgui_system::init();

        let rt_gizmo = rendering::rendertarget::RenderTarget::new(300, 300);
        rendertarget::unbind_framebuffer();

        let font_texture = imgui_system.load_font_atlas_texture();
        let nice_image = load_texture_from_path("./res/aP3DgOB_460swp.png").unwrap();

        let gizmo =
            geometry::get_mesh_repo(|mr| MeshToken::from(mr.get_mesh_by_name("gizmo").unwrap()));

        let mut loop_timer = loop_timing::CatchupTimer::new();

        let _fps = 0;

        let mut t_color = [1.0, 0.0, 0.0, 1.0];

        let mut wiregrid = false;

        'mainloop: loop {
            //PROCESS INPUT
            self.handle_events(&mut imgui_system);
            if self.game_state.input_flags.contains(InputFlags::CLOSE) {
                break 'mainloop;
            }

            let game_state = &mut self.game_state;

            while loop_timer.should_update() {
                //UPDATE

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

                            let label = if wiregrid { "no wwiregrid" } else { "iregrid" };
                            if ui.button(label) {
                                wiregrid = !wiregrid;
                                gl_wiregrid(wiregrid);
                            }

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
                //HANDLE INPUT

                game_state.update();
            }

            //RENDER
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + 0);
                font_texture.bind();
                gl::ActiveTexture(gl::TEXTURE0 + 1);
                //font_texture.bind();
                nice_image.bind();
                gl::ActiveTexture(gl::TEXTURE0 + 2);
                rt_gizmo.bind_texture();
            }

            let i = loop_timer.get_iv();

            let view = game_state.cam.get_i_view(i);

            rt_gizmo.bind_framebuffer();
            three_d_rendering_setup();

            clear_color(0.1, 0.1, 0.1, 1.0);
            clear_drawbuffer();
            set_viewport(300, 300);

            gizmo_shader.use_program();
            gizmo_shader.set_view(view);
            gizmo.bind_vertex_array();
            gizmo.draw_point_elements();

            rendertarget::unbind_framebuffer();
            set_viewport(
                game_state.window_size_i32[0] - 300,
                game_state.window_size_i32[1],
            );

            clear_color(0.0, 0.3, 0.3, 1.0);
            clear_drawbuffer();

            game_state.draw(i);

            set_viewport(game_state.window_size_i32[0], game_state.window_size_i32[1]);

            imgui_rendering_setup();

            imgui_shader_program.use_program();
            imgui_shader_program.set_matrix(game_state.ui_projection);
            imgui_system.draw();

            self.window.swap();
        }
    }
}
