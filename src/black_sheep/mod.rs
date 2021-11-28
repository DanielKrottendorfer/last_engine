extern crate cgmath;
extern crate gl;
extern crate rand;
extern crate sdl2;

pub mod rendering;
mod window;

#[allow(dead_code)]
mod constants;

pub mod settings;

mod camera;
mod transform;

mod gamestate;
mod q_i_square_root;

mod point_cloud;
mod point_grid;

mod imgui_system;

mod setup;

mod algorithms;

use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;

use std::time::Duration;

use cgmath::Matrix4;
use cgmath::Vector2;
use cgmath::Vector4;
use gamestate::*;

use cgmath::{Deg, Vector3};
use imgui::{ColorPicker, Condition, Image, TextureId, Window};
use rendering::geometry::MeshRepo;
use rendering::shader::ShaderRepo;
use sdl2::mouse::MouseButton;
use window::window_util::*;
use window::SDLWindow;

use sdl2::event::{Event, WindowEvent};

use crate::black_sheep::rendering::geometry::MeshToken;
use crate::black_sheep::rendering::loader::load_texture_from_path;
use crate::black_sheep::rendering::rendertarget;
use crate::black_sheep::settings::*;
use crate::black_sheep::window::window_util::{clear_drawbuffer, set_viewport};

use camera::structs::FlyingEye;

pub struct BlackSheep {
    window: SDLWindow,
    mesh_repo: MeshRepo,
    shader_repo: ShaderRepo,
    cam: FlyingEye,
}

impl BlackSheep {
    pub fn new() -> Self {
        // KEEP THIS ORDER
        let window = SDLWindow::new();
        let shader_repo = ShaderRepo::new();
        let mesh_repo = MeshRepo::new();

        Self {
            window,
            mesh_repo,
            shader_repo,
            cam: FlyingEye::new(),
        }
    }

    pub fn run(self) {
        let mut window = self.window;
        let mut mesh_repo = self.mesh_repo;
        let shader_repo = self.shader_repo;
        let mut cam = self.cam;
        let mut gamestate: GameFlags = Default::default();
        let mut input: KeyboardInputFlags = Default::default();

        //init_rendering_setup();
        let mut window_size_f32 = INIT_WINDOW_SIZE_F32;
        let mut window_size_i32 = INIT_WINDOW_SIZE_I32;

        let mut ui_projection =
            ui_projection_mat([INIT_WINDOW_SIZE_I32[0], INIT_WINDOW_SIZE_I32[1]]);
        let mut orthogonal_projection =
            ui_projection_mat([INIT_WINDOW_SIZE_I32[0] - 300, INIT_WINDOW_SIZE_I32[1]]);

        init_rendersetup();

        setup::init_mesh(&mut mesh_repo);
        let gizmo = MeshToken::from(mesh_repo.get_mesh_by_name("gizmo").unwrap());
        let cube = MeshToken::from(mesh_repo.get_mesh_by_name("cube").unwrap());
        let cube_cloud = MeshToken::from(mesh_repo.get_mesh_by_name("cloud").unwrap());

        let (mut positions, e) = point_grid::new_point_grid(80, 80, 500);
        let mut dirs = Vec::new();
        let mut rads = Vec::new();
        let mut colors = Vec::new();

        positions.iter_mut().for_each(|v| {
            *v += Vector2::new(50.0, 50.0);
        });

        let mut rng = SmallRng::seed_from_u64(123);

        for _i in 0..positions.len() {
            let a: f32 = rng.gen_range(1.0..3.0);
            rads.push(a);

            let v: Vector3<f32> = Vector3::new(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            );
            colors.push(v);

            let t = rng.gen_range(0.0..7.0);
            let d: Vector2<f32> = Vector2::new(f32::sin(t) * 2.0, f32::cos(t) * 2.0);
            dirs.push(d);
        }

        let points = mesh_repo.add_mesh("points", |mesh| {
            mesh.add_dynamic_floatbuffer(positions.as_slice(), 0, 2);
            mesh.add_floatbuffer(rads.as_slice(), 1, 1);
            mesh.add_floatbuffer(colors.as_slice(), 2, 3);
            mesh.add_elementarraybuffer(e.as_slice());
        });
        let points = mesh_repo.get_mesh_by_name("points").unwrap();
        let color_shader = &shader_repo.color_3d;
        let cloud_shader = &shader_repo.point_cloud;
        let imgui_shader_program = &shader_repo.imgui;
        let gizmo_shader_program = &shader_repo.gizmo;
        let point_2d_shader = &shader_repo.point_2d;

        let mut imgui_system = imgui_system::init();

        let rt_gizmo = rendering::rendertarget::RenderTarget::new(300, 300);
        rendertarget::unbind_framebuffer();

        let font_texture = imgui_system.load_font_atlas_texture();
        let nice_image = load_texture_from_path("./res/aP3DgOB_460swp.png").unwrap();

        cam.move_cam(Vector3::new(1.35, 1.35, 2.0));
        cam.rotate_h(Deg(35.0));

        let time = std::time::Instant::now();
        let mut previous = time.elapsed();
        let mut lag = Duration::from_secs(0);
        let mut last = Duration::from_secs(0);

        let mut fps = 0;

        let mut t_color = [1.0, 0.0, 0.0, 1.0];

        let mut run_ui = false;
        let mut prune = false;

        'mainloop: loop {
            let current = time.elapsed();
            let elapsed = current - previous;
            previous = current;
            lag += elapsed;
            #[cfg(not(feature = "fps_off"))]
            {
                fps += 1;
                if current - last > Duration::from_secs(1) {
                    println!("fps: {}", fps);
                    last = current;
                    fps = 0;
                }
            }

            //PROCESS INPUT
            while let Some(event) = window.poll_event() {
                imgui_system.handle_event(&event);
                match event {
                    Event::Quit { .. } => {
                        break 'mainloop;
                    }
                    Event::KeyDown { keycode, .. } => {
                        if let Some(key) = keycode {
                            use sdl2::keyboard::Keycode::*;
                            if let Escape = key {
                                break 'mainloop;
                            } else {
                                gamestate::key_down(key, &mut input);
                            }
                            if let R = key {
                                if let Some(cc) = mesh_repo.get_mesh_by_name("cloud") {
                                    let new_c: Vec<Vector4<f32>> = (0..cc.vertex_count)
                                        .map(|_x| Vector4::from(t_color))
                                        .collect();
                                    cc.update_buffer(new_c.as_slice(), 1);
                                    #[cfg(not(feature = "debug_off"))]
                                    println!("update triangle colors");
                                }
                            }
                        } else {
                            #[cfg(not(feature = "debug_off"))]
                            println!("No Valid KeyCode");
                        }
                    }
                    Event::KeyUp { keycode, .. } => {
                        if let Some(key) = keycode {
                            gamestate::key_up(key, &mut input);
                        } else {
                            #[cfg(not(feature = "debug_off"))]
                            println!("No Valid KeyCode");
                        }
                    }
                    Event::MouseButtonDown { mouse_btn, .. } => {
                        if MouseButton::Right == mouse_btn {
                            gamestate.insert(GameFlags::CAPTURED_MOUSE);
                            window.capture_mouse();
                        }
                    }
                    Event::MouseButtonUp { mouse_btn, .. } => {
                        if MouseButton::Right == mouse_btn {
                            gamestate.remove(GameFlags::CAPTURED_MOUSE);
                            window.release_mouse();
                        }
                    }
                    Event::MouseMotion { xrel, yrel, .. } => {
                        if gamestate.contains(GameFlags::CAPTURED_MOUSE) {
                            if xrel != 0 {
                                cam.rotate_v(Deg(xrel as f32 / 10.0));
                            }
                            if yrel != 0 {
                                cam.rotate_h(Deg(yrel as f32 / 10.0));
                            }
                        }
                    }
                    Event::Window { win_event, .. } => match win_event {
                        WindowEvent::Resized(w, h) => {
                            set_viewport(w, h);
                            ui_projection = ui_projection_mat([w, h]);
                            orthogonal_projection = ui_projection_mat([w - 300, h]);
                            window_size_f32 = [w as f32, h as f32];
                            window_size_i32 = [w, h];
                            //rt_main.resize(window_size_i32[0] - 300, window_size_i32[1]);
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }

            while lag >= MS_PER_UPDATE {
                //UPDATE
                cam.update();
                imgui_system.update(&mut |ui| {
                    use imgui::WindowFlags;
                    Window::new("Image")
                        .size([300.0, window_size_f32[1]], Condition::Always)
                        .position([window_size_f32[0] - 300.0, 0.0], Condition::Always)
                        .flags(
                            WindowFlags::NO_MOVE
                                | WindowFlags::NO_RESIZE
                                | WindowFlags::NO_COLLAPSE
                                | WindowFlags::NO_TITLE_BAR,
                        )
                        .build(&ui, || {
                            ui.text("Hello world!");
                            ui.text("こんにちは世界！");

                            let label = if run_ui { "stop" } else { "start" };
                            if ui.button(label) {
                                run_ui = !run_ui;
                            }
                            let label = if prune { "np prune" } else { "prune" };
                            if ui.button(label) {
                                prune = !prune;
                            }
                            ui.text(format!("{:?}", -cam.position));
                            ui.text(format!("{:#?}", cam.orientation));
                            ColorPicker::new("color_picker", &mut t_color).build(ui);
                            Image::new(TextureId::new(2 as usize), [300.0, 300.0])
                                .uv0([0.0, 1.0])
                                .uv1([1.0, 0.0])
                                .build(ui);
                            Image::new(TextureId::new(1 as usize), [300.0, 300.0]).build(ui);
                        });
                });

                //HANDLE INPUT
                {
                    // if run_ui {
                    //     if !prune {
                    //         algorithms::brute_force_collison(&positions, &mut dirs, &rads);
                    //     } else {
                    //         algorithms::sweep_and_prune(&positions, &mut dirs, &rads);
                    //     }

                    //     algorithms::prevent_out_of_bound(
                    //         &positions,
                    //         &mut dirs,
                    //         &rads,
                    //         &window_size_f32,
                    //     );
                    //     positions.iter_mut().zip(dirs.iter()).for_each(|(p, d)| {
                    //         *p += *d;
                    //     });
                    //     points.update_buffer(positions.as_slice(), 0);
                    // }

                    if let Some(v) = get_movement(&mut input) {
                        cam.set_movement(v);
                    } else {
                        cam.reset_movement();
                    }
                }

                lag -= MS_PER_UPDATE;
            }

            //RENDER
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + 0);
                font_texture.bind();
                gl::ActiveTexture(gl::TEXTURE0 + 1);
                nice_image.bind();
                gl::ActiveTexture(gl::TEXTURE0 + 2);
                rt_gizmo.bind_texture();
            }

            let i = lag.as_secs_f32() / MS_PER_UPDATE.as_secs_f32();

            let view = cam.get_i_view(i);
            let aspect = (window_size_f32[0] - 300.0) / window_size_f32[1];
            let projection = cgmath::perspective(Deg(90.0), aspect, 0.1, 1000.0);

            let model = Matrix4::from_translation(Vector3::new(1.2, 0.0, 0.0));

            rt_gizmo.bind_framebuffer();
            three_d_rendering_setup();

            clear_color(0.1, 0.1, 0.1, 1.0);
            clear_drawbuffer();
            set_viewport(300, 300);

            gizmo_shader_program.use_program();
            gizmo_shader_program.set_view(view);
            gizmo.bind_vertex_array();
            gizmo.draw_point_elements();

            rendertarget::unbind_framebuffer();
            set_viewport(window_size_i32[0] - 300, window_size_i32[1]);

            clear_color(0.0, 0.3, 0.3, 1.0);
            clear_drawbuffer();

            color_shader.use_program();
            color_shader.set_MVP(projection * view * model);
            cube.bind_vertex_array();
            cube.draw_triangle_elements();

            cloud_shader.use_program();
            cloud_shader.set_mv(view);
            cloud_shader.set_projection(projection);
            cube_cloud.bind_vertex_array();
            cube_cloud.draw_point_elements();

            // unsafe {
            //     gl::Disable(gl::DEPTH_TEST);
            // }
            // point_2d_shader.use_program();
            // point_2d_shader.set_projection(orthogonal_projection);
            // points.bind_vertex_array();
            // points.draw_point_elements();

            ui_rendering_setup();

            set_viewport(window_size_i32[0], window_size_i32[1]);

            imgui_shader_program.use_program();
            imgui_shader_program.set_matrix(ui_projection);
            imgui_system.draw(|t| imgui_shader_program.set_tex(t));

            window.swap();
        }
    }
}
