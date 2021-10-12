
mod gamestate;
pub mod rendering;
mod window;

mod constants;

use std::borrow::BorrowMut;

use gamestate::GameState;
use rendering::shader::ShaderRepo;
use rendering::geometry::MeshRepo;
use window::SDLWindow;

use sdl2::event::Event;

pub struct BlackSheep {
    window:      SDLWindow,
    mesh_repo:   MeshRepo,
    shader_repo: ShaderRepo,
    gamestate:   GameState,
}

impl BlackSheep {
    pub fn new() -> Self {
        let window = SDLWindow::new();
        let shader_repo = ShaderRepo::new();
        let mesh_repo = MeshRepo::new();

        Self {
            window,
            mesh_repo,
            shader_repo,
            gamestate: GameState
        }
    }

    pub fn run(mut self,init: Option<fn(&mut GameState)>) {

        if let Some(init) = init{
            init(self.gamestate.borrow_mut());
        }else{
            let square_id = self.mesh_repo.add_mesh(|mesh| {
                use constants::*;
                mesh.add_floatbuffer(&SQUARE,      0, 3);
                mesh.add_floatbuffer(&SQUARE_NORM, 1, 3);
                mesh.add_floatbuffer(&UVS,         2, 2);
                mesh.add_elementarraybuffer(&ELEMENTS)
            });
        }
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
        }

    }
}
