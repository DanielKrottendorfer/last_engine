pub mod rendering;
mod window;

mod algorithms;
#[allow(dead_code)]
mod constants;
pub mod gamestate;
mod generators;
mod imgui_system;
mod loop_timing;
mod q_i_square_root;
mod script;
pub mod settings;
mod setup;
mod torus;
mod transform;

use cgmath::{
    Deg, InnerSpace, Quaternion, Rad, Rotation, Rotation3, Vector2,
};
use cgmath::{Matrix4, Vector3};
use gamestate::*;

use imgui::{ColorPicker, Condition, Image, TextureId, Window};
use sdl2::mouse::MouseButton;
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

use self::torus::torus_r;

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

    let meshes = setup::init_mesh();

    let ape = meshes[5].clone();
    let torus = meshes[6].clone();

    let rendering = rendering::shader::get_shader_repo();

    let three_d = rendering.color_3d;

    let game_state = GameState::new(
        |ecs| {
            use rand::{thread_rng, Rng};

            let mut rng = thread_rng();

            for i in 0..3 {
                for y in 0..3 {
                    ecs.add_ape_soa(
                        [i as f32, 0.0, y as f32].into(),
                        Quaternion::from_angle_x(Rad(0.0)),
                        [i as f32, rng.gen_range(-0.5..0.5), y as f32].into(),
                        Quaternion::from_angle_x(Rad(0.0)),
                        Vector3::new(1.0, 1.0, 1.0),
                        cgmath::SquareMatrix::identity(),
                    );
                }
            }

            let mut circle = ecs.get_circle_accessor();
            let positions = ecs.get_positions_accessor();

            let mut pos_update = ecs.get_update_pos_ori_accessor();

            move |_input| {
                {
                    let mut update = pos_update.lock();
                    for (pos, ori, direction, target_ori) in update.iter() {
                        *pos = *pos + *direction;
                        *ori = *target_ori;
                    }
                }
                let mut c_l = circle.lock();
                let pos_s = positions.lock();

                let speed = 0.5;
                for (pos, ori, direction, target_ori, col, key) in c_l.iter() {
                    let r_x = Quaternion::from_angle_x(Deg(20.0));
                    let r_y = Quaternion::from_angle_y(Deg(20.0));

                    let q1 = *ori * r_x;
                    let q2 = *ori * r_x.invert();
                    let q3 = *ori * r_y;
                    let q4 = *ori * r_y.invert();

                    let v1 = *pos + q1.rotate_vector(Vector3::unit_z() * speed);
                    let v2 = *pos + q2.rotate_vector(Vector3::unit_z() * speed);
                    let v3 = *pos + q3.rotate_vector(Vector3::unit_z() * speed);
                    let v4 = *pos + q4.rotate_vector(Vector3::unit_z() * speed);

                    let id = if torus_r(*pos, 20.0) > 4.0 {
                        let t1 = torus_r(v1, 20.0);
                        let t2 = torus_r(v2, 20.0);
                        let t3 = torus_r(v3, 20.0);
                        let t4 = torus_r(v4, 20.0);

                        let mut min = 0;
                        let ts = [t1, t2, t3, t4];
                        for i in 1..ts.len() {
                            if ts[i] < ts[min] {
                                min = i;
                            }
                        }

                        *col = Vector3::unit_x();
                        min
                    } else {
                        let mut min_dist = f32::MAX;
                        let mut min_key = key.clone();
                        for (p, k) in pos_s.iter() {
                            if key == k {
                                continue;
                            }
                            let dist = (pos - p).magnitude();
                            if dist < min_dist {
                                min_dist = dist;
                                min_key = k;
                            }
                        }

                        let p = pos_s.get(min_key).unwrap();

                        let t1 = (v1 - *p).magnitude();
                        let t2 = (v2 - *p).magnitude();
                        let t3 = (v3 - *p).magnitude();
                        let t4 = (v4 - *p).magnitude();

                        let id = if min_dist < 5.0 {
                            let mut max = 0;
                            let ts = [t1, t2, t3, t4];
                            for i in 1..ts.len() {
                                if ts[i] > ts[max] {
                                    max = i;
                                }
                            }

                            *col = Vector3::unit_y();
                            max
                        } else if min_dist > 10.0 {
                            let mut min = 0;
                            let ts = [t1, t2, t3, t4];
                            for i in 1..ts.len() {
                                if ts[i] < ts[min] {
                                    min = i;
                                }
                            }

                            *col = Vector3::unit_z();
                            min
                        } else {
                            continue;
                        };
                        id
                    };

                    match id {
                        0 => {
                            *target_ori = q1;
                            *direction = (v1 - *pos) * speed;
                        }
                        1 => {
                            *target_ori = q2;
                            *direction = (v2 - *pos) * speed;
                        }
                        2 => {
                            *target_ori = q3;
                            *direction = (v3 - *pos) * speed;
                        }
                        3 => {
                            *target_ori = q4;
                            *direction = (v4 - *pos) * speed;
                        }

                        _ => (),
                    }
                }
            }
        },
        |ecs| {
            let draw_m = ecs.get_draw_accessor();

            let mut calc_mat = ecs.get_calculate_mat_accessor();

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

                ape.bind_vertex_array();
                three_d.use_program();

                for (m, c) in d_lock.iter() {
                    three_d.set_MVP(prj * view * m);
                    three_d.set_col(*c);
                    ape.draw_triangle_elements();
                }

                three_d.set_MVP(prj * view);
                three_d.set_col(Vector3::new(1.0, 0.0, 1.0));

                torus.bind_vertex_array();
                torus.draw_line_elements();
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
                            cgmath::perspective(Deg(120.0), aspect, 0.1, 1000.0);
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
