extern crate cgmath;
extern crate gl;
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

mod imgui_system;
mod loader;

use std::time::Duration;

use cgmath::Vector4;
use gamestate::*;

use cgmath::{Deg, Vector3};
use imgui::Condition;
use imgui::Window;
use imgui::WindowFlags;
use imgui::im_str;
use rendering::geometry::MeshRepo;
use rendering::shader::ShaderRepo;
use sdl2::mouse::MouseButton;
use window::window_util::*;
use window::SDLWindow;

use sdl2::event::{Event, WindowEvent};

use crate::black_sheep::settings::INIT_WINDOW_SIZE_F32;
use crate::black_sheep::settings::INIT_WINDOW_SIZE_I32;
use crate::black_sheep::settings::MS_PER_UPDATE;
use crate::black_sheep::{
    settings::INIT_WINDOW_SIZE,
    window::window_util::{clear_window, set_viewport},
};

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

        use constants::*;
        let triangle = mesh_repo.add_mesh("triangle", |mesh| {
            mesh.add_floatbuffer(&SIMPLE_TRIANGL, 0, 2);
            mesh.add_elementarraybuffer(&TRIANGLE_ELEMENTS);
        });

        // let cube = mesh_repo.add_mesh("cube", |mesh| {
        //     mesh.add_floatbuffer(&CUBE, 0, 3);
        //     mesh.add_floatbuffer(&CUBE_COLOR, 1, 3);
        //     mesh.add_elementarraybuffer(&CUBE_ELEMENTS);
        // });

        let cube_cloud = mesh_repo.add_mesh("cloud", |mesh| {
            let (v, c, e) = point_cloud::point_cube(4);
            mesh.add_floatbuffer(v.as_slice(), 0, 3);
            mesh.add_floatbuffer(c.as_slice(), 1, 4);
            mesh.add_elementarraybuffer(e.as_slice());
        });

        let simple_shader = &shader_repo.simple;
        let color_shader = &shader_repo.color_3d;
        let cloud_shader = &shader_repo.point_cloud;



        let imgui_shader_program = &shader_repo.imgui;

        let mut imgui_system = imgui_system::init();
        imgui_system.imgui.io_mut().display_size = INIT_WINDOW_SIZE_F32;
        let font_texture = imgui_system.load_font_atlas_texture();

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, font_texture);
        }
        imgui_shader_program.set_tex(0);

        let mut ui_projection = ui_projection_mat([INIT_WINDOW_SIZE_I32[0],INIT_WINDOW_SIZE_I32[0]]);



        let mut color = Vector3::new(1.0, 0.0, 1.0);
        let mut window_size = INIT_WINDOW_SIZE_F32;

        cam.move_cam(Vector3::new(3.5, 3.5, 4.0));
        cam.rotate_h(Deg(35.0));
        cam.rotate_v(Deg(-35.0));

        let time = std::time::Instant::now();
        let mut previous = time.elapsed();
        let mut lag = Duration::from_secs(0);
        let mut last = Duration::from_secs(0);

        let mut fps = 0;

        'mainloop: loop {
            let current = time.elapsed();
            let elapsed = current - previous;
            previous = current;
            lag += elapsed;
            fps += 1;
            if current - last > Duration::from_secs(1) {
                println!("fps: {}", fps);
                last = current;
                fps = 0;
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

                                if let Some(cc) = mesh_repo.get_mesh_by_name("cloud"){
                                    let new_c:Vec<Vector4<f32>> = (0..64).map(|x| {
                                        Vector4::new(1.0,0.0,0.0,1.0)
                                    }).collect();
                                    cc.update_floatbuffer(new_c.as_slice(), 1);
                                    println!("update");
                                }
                            }
                        } else {
                            println!("No Valid KeyCode");
                        }
                    }
                    Event::KeyUp { keycode, .. } => {
                        if let Some(key) = keycode {
                            gamestate::key_up(key, &mut input);
                        } else {
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
                            ui_projection = ui_projection_mat([w,h]);
                            window_size = [w as f32, h as f32];
                        },
                        _ => (),
                    },
                    _ => (),
                }
            }

            while lag >= MS_PER_UPDATE {
                //UPDATE
                cam.update();

                //HANDLE INPUT
                {
                    use KeyboardInputFlags as kf;
                    if input.contains(kf::Q) {
                        color = Vector3::new(1.0, 0.0, 1.0);
                    } else if input.contains(kf::E) {
                        color = Vector3::new(0.4, 0.0, 0.4);
                    }

                    if let Some(v) = get_movement(&mut input) {
                        cam.set_movement(v);
                    } else {
                        cam.reset_movement();
                    }
                }

                lag -= MS_PER_UPDATE;
            }

            //RENDER

            let i = lag.as_secs_f32() / MS_PER_UPDATE.as_secs_f32();

            let view = cam.get_i_view(i);
            let aspect = window_size[0] / window_size[1];
            let projection = cgmath::perspective(Deg(90.0), aspect, 0.2, 1000.0);

            clear_window();

            three_d_rendering_setup();

            // color_shader.use_program();
            // color_shader.set_MVP(projection * view);
            // cube.bind_vertex_array();
            // cube.draw_triangle_elements();

            simple_shader.use_program();
            simple_shader.set_color(color);
            triangle.bind_vertex_array();
            triangle.draw_triangle_elements();

            cloud_shader.use_program();
            cloud_shader.set_mv(view);
            cloud_shader.set_projection(projection);
            cube_cloud.bind_vertex_array();
            cube_cloud.draw_point_elements();


            ui_rendering_setup();

            imgui_shader_program.use_program();
            imgui_shader_program.set_tex(0);
    
            imgui_shader_program.set_matrix(ui_projection);

            imgui_system.draw(&mut |ui| {
                Window::new(im_str!("Hello world"))
                    .size([300.0, 210.0], Condition::Once)
                    .position([0.0, 0.0], Condition::Once)
                    .flags(WindowFlags::NO_MOVE | WindowFlags::NO_RESIZE)
                    .build(&ui, || {
                        ui.text(im_str!("Hello world!"));
                        ui.text(im_str!("こんにちは世界！"));
                        ui.text(im_str!("This...is...imgui-rs!"));
                        ui.separator();
                    });
            });
            window.swap();
        }
    }
}
