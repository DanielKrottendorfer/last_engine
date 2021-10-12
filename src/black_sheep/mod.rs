
pub mod rendering;
mod window;
mod constants;

use rendering::shader::ShaderRepo;
use rendering::geometry::MeshRepo;
use window::SDLWindow;

use sdl2::event::Event;

use crate::black_sheep::window::window_util::clear_window;

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
        println!("{:?}",simple_shader);
        let mut running = true;

        while running {
            
            while let Some(event) = self.window.poll_event() {

                match event {
                    Event::Quit {..} => {
                        running = false;
                    },
                    Event::KeyDown{keycode, ..} => {
    
                        if let Some(key) = keycode {
                            use sdl2::keyboard::Keycode::*;

                            match key {
                                Escape => {
                                    running = false;
                                }
                                _ => (),
                            }
                        } else {
                            println!("No Valid KeyCode")
                        }
                    },
                    _ => ()
                }

                //Render;

                //Swap;

            }
            
            clear_window();

            simple_shader.use_program();

            triangle.bind_vertex_array();
            triangle.draw_elements();

            self.window.swap();
        }

    }
}
