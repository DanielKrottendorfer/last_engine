extern crate cgmath;
extern crate gl;
extern crate sdl2;

pub mod rendering;
mod window;

#[allow(dead_code)]
mod constants;

//mod specs_world;

mod world;

pub mod settings;

use world::World;

use cgmath::{Deg, ElementWise, Rad, Vector3};
use rendering::geometry::MeshRepo;
use rendering::shader::ShaderRepo;
use window::SDLWindow;
use window::window_util::*;

use sdl2::event::{Event, WindowEvent};

use crate::black_sheep::{settings::INIT_WINDOW_SIZE, window::window_util::{clear_window, set_viewport}};

pub struct BlackSheep {
    window: SDLWindow,
    mesh_repo: MeshRepo,
    shader_repo: ShaderRepo,
    world: World
}

impl BlackSheep {
    pub fn new() -> Self {
        // KEEP THIS ORDER
        let window = SDLWindow::new();
        let shader_repo = ShaderRepo::new();
        let mesh_repo = MeshRepo::new();
        let world = World::new();
        
        Self {
            window,
            mesh_repo,
            shader_repo,
            world
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

        self.world.move_cam(Vector3::new(0.0,0.0,5.0));

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
                                W => self.world.move_cam(Vector3::new(0.0,0.0,-1.0)),
                                A => self.world.move_cam(Vector3::new(-1.0,0.0,0.0)),
                                D => self.world.move_cam(Vector3::new(1.0,0.0,0.0)),
                                S => self.world.move_cam(Vector3::new(0.0,0.0,1.0)),
                                _ => (),
                            }
                        } else {
                            println!("No Valid KeyCode")
                        }
                    }
                    Event::Window { win_event, .. } => match win_event {
                        WindowEvent::Resized(w, h) => {
                            set_viewport(w, h);
                            window_size = (w as u32,h as u32);
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }

            let view = self.world.get_view();
            let aspect = window_size.0 as f32/window_size.1 as f32;
            let projection = cgmath::perspective(Deg(120.0), aspect, 0.001, 1000.0);

            clear_window();

            three_d_rendering_setup();

            color_shader.use_program();
            color_shader.set_MVP(projection*view);

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
