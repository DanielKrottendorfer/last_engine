
pub mod rendering;
mod window;

#[allow(dead_code)]
mod constants;

use cgmath::Vector3;
use rendering::shader::ShaderRepo;
use rendering::geometry::MeshRepo;
use window::SDLWindow;

use sdl2::event::Event;

use crate::black_sheep::window::window_util::{clear_window, set_viewport};

pub struct BlackSheep {
    window:      SDLWindow,
    mesh_repo:   MeshRepo,
    shader_repo: ShaderRepo
}

impl BlackSheep {
    pub fn new() -> Self {
        let window = SDLWindow::new();
        let shader_repo = ShaderRepo::new();
        let mesh_repo = MeshRepo::new();

        Self {
            window,
            mesh_repo,
            shader_repo
        }
    }

    pub fn run(mut self) {

        use constants::*;

        let triangle = self.mesh_repo.add_mesh(|mesh| {
            mesh.add_floatbuffer(&SIMPLE_TRIANGL,0, 3);
            mesh.add_elementarraybuffer(&TRIANGLE_ELEMENTS);
        });

        let simple_shader = &self.shader_repo.simple;

        let mut color = Vector3::new(1.0, 0.0, 1.0);

        'mainloop: loop {
            
            while let Some(event) = self.window.poll_event() {

                match event {
                    Event::Quit {..} => {
                        break 'mainloop;
                    },
                    Event::KeyDown{keycode, ..} => {
    
                        if let Some(key) = keycode {
                            use sdl2::keyboard::Keycode::*;

                            println!("{:?}",key);
                            
                            match key {
                                Escape => {
                                    break 'mainloop;
                                },
                                W => {
                                    color = Vector3::new(0.0,1.0,1.0);
                                },
                                Q => {
                                    color = Vector3::new(1.0, 0.0, 1.0);
                                }
                                _ => (),
                            }
                        } else {
                            println!("No Valid KeyCode")
                        }
                    }, 
                    Event::Window{win_event, ..} => {
                        match win_event {
                            sdl2::event::WindowEvent::Resized(w, h) => {
                                set_viewport(w, h);
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }
            }
            
            clear_window();

            simple_shader.use_program();
            simple_shader.set_color(color);
            triangle.bind_vertex_array();
            triangle.draw_elements();

            self.window.swap();
        }

    }
}
