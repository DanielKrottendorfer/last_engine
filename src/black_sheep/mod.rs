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

use cgmath::{Deg, Vector3};
use rendering::geometry::MeshRepo;
use rendering::shader::ShaderRepo;
use sdl2::mouse::MouseButton;
use window::window_util::*;
use window::SDLWindow;

use sdl2::event::{Event, WindowEvent};

use crate::black_sheep::{
    settings::INIT_WINDOW_SIZE,
    window::window_util::{clear_window, set_viewport},
};

use camera::structs::FlyingEye;

use rendering::geometry::MeshToken;

pub struct BlackSheep {
    window: SDLWindow,
    mesh_repo: MeshRepo,
    shader_repo: ShaderRepo,
    cam: FlyingEye,
    meshes: Vec<(transform::Transform, MeshToken)>,
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
            meshes: Vec::new(),
        }
    }

    pub fn run(mut self) {
        use constants::*;
        let triangle = self.mesh_repo.add_mesh(|mesh| {
            mesh.add_floatbuffer(&SIMPLE_TRIANGL, 0, 3);
            mesh.add_elementarraybuffer(&TRIANGLE_ELEMENTS);
        });

        let cube = self.mesh_repo.add_mesh(|mesh| {
            mesh.add_floatbuffer(&CUBE, 0, 3);
            mesh.add_floatbuffer(&CUBE_COLOR, 1, 3);
            mesh.add_elementarraybuffer(&CUBE_ELEMENTS);
        });

        let simple_shader = &self.shader_repo.simple;
        let color_shader = &self.shader_repo.color_3d;

        let mut color = Vector3::new(1.0, 0.0, 1.0);

        let mut window_size = INIT_WINDOW_SIZE;

        self.cam.move_cam(Vector3::new(0.0, 0.0, 5.0));

        let mut mouse_captured = false;

        'mainloop: loop {
            while let Some(event) = self.window.poll_event() {
                match event {
                    Event::Quit { .. } => {
                        break 'mainloop;
                    }
                    Event::KeyDown { keycode, .. } => {
                        if let Some(key) = keycode {
                            use sdl2::keyboard::Keycode::*;
                            match key {
                                Escape => break 'mainloop,
                                E => color = Vector3::new(0.0, 1.0, 1.0),
                                Q => color = Vector3::new(1.0, 0.0, 1.0),
                                W => self.cam.move_cam(Vector3::new(0.0, 0.0, -1.0)),
                                S => self.cam.move_cam(Vector3::new(0.0, 0.0, 1.0)),
                                D => self.cam.move_cam(Vector3::new(1.0, 0.0, 0.0)),
                                A => self.cam.move_cam(Vector3::new(-1.0, 0.0, 0.0)),
                                X => self.cam.move_cam(Vector3::new(0.0, 1.0, 0.0)),
                                Y => self.cam.move_cam(Vector3::new(0.0, -1.0, 0.0)),
                                Right => self.cam.rotate_v(Deg(1.0)),
                                Left => self.cam.rotate_v(Deg(-1.0)),
                                Up => self.cam.rotate_h(Deg(1.0)),
                                Down => self.cam.rotate_h(Deg(-1.0)),
                                _ => (),
                            }
                        } else {
                            println!("No Valid KeyCode");
                        }
                    },
                    Event::MouseButtonDown{mouse_btn, ..} => {
                        if MouseButton::Right == mouse_btn {
                            mouse_captured = true;
                            self.window.capture_mouse();
                        }
                    },
                    Event::MouseButtonUp{mouse_btn, ..} => {
                        if MouseButton::Right == mouse_btn {
                            mouse_captured = false;
                            self.window.release_mouse();
                        }
                    },
                    Event::MouseMotion{ xrel, yrel, ..} => {
                        if mouse_captured {
                            self.cam.rotate_v(Deg(xrel as f32 / 10.0));
                            self.cam.rotate_h(Deg(yrel as f32 / 10.0));
                        }
                    },
                    Event::Window { win_event, .. } => match win_event {
                        WindowEvent::Resized(w, h) => {
                            set_viewport(w, h);
                            window_size = (w as u32, h as u32);
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }

            let view = self.cam.get_i_view(0.0);
            let aspect = window_size.0 as f32 / window_size.1 as f32;
            let projection = cgmath::perspective(Deg(120.0), aspect, 0.001, 1000.0);
            

            clear_window();

            three_d_rendering_setup();

            color_shader.use_program();
            color_shader.set_MVP(projection * view );

            cube.bind_vertex_array();
            cube.draw_elements();

            simple_shader.use_program();
            simple_shader.set_color(color);

            triangle.bind_vertex_array();
            triangle.draw_elements();

            self.window.swap();
        }
    }
}
