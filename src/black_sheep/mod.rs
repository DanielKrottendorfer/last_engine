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

use std::time::Duration;

use gamestate::*;

use cgmath::{Deg, Vector3};
use rendering::geometry::MeshRepo;
use rendering::shader::ShaderRepo;
use sdl2::mouse::MouseButton;
use window::window_util::*;
use window::SDLWindow;

use sdl2::event::{Event, WindowEvent};

use crate::black_sheep::settings::MS_PER_UPDATE;
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
        let mut window = self.window;
        let mut mesh_repo = self.mesh_repo;
        let shader_repo = self.shader_repo;
        let mut cam = self.cam;
        let mut meshes = self.meshes;
        let mut gamestate: GameFlags = Default::default();
        let mut input: KeyboardInputFlags = Default::default();

        use constants::*;
        let triangle = mesh_repo.add_mesh(|mesh| {
            mesh.add_floatbuffer(&SIMPLE_TRIANGL, 0, 3);
            mesh.add_elementarraybuffer(&TRIANGLE_ELEMENTS);
        });

        let cube = mesh_repo.add_mesh(|mesh| {
            mesh.add_floatbuffer(&CUBE, 0, 3);
            mesh.add_floatbuffer(&CUBE_COLOR, 1, 3);
            mesh.add_elementarraybuffer(&CUBE_ELEMENTS);
        });

        let simple_shader = &shader_repo.simple;
        let color_shader = &shader_repo.color_3d;

        let mut color = Vector3::new(1.0, 0.0, 1.0);

        let mut window_size = (INIT_WINDOW_SIZE.0 as f32, INIT_WINDOW_SIZE.1 as f32);

        cam.move_cam(Vector3::new(0.0, 0.0, 5.0));

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
                            window_size = (w as f32, h as f32);
                        }
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
                        color = Vector3::new(1.0, 1.0, 0.0);
                    } 
                    
                    if let Some(v) = get_movement(&mut input){
                        cam.set_movement(v);
                    }else{
                        cam.reset_movement();
                    }
                }

                lag -= MS_PER_UPDATE;
            }

            //RENDER

            let i = lag.as_secs_f32() / MS_PER_UPDATE.as_secs_f32();

            let view = cam.get_i_view(i);
            let aspect = window_size.0 / window_size.1;
            let projection = cgmath::perspective(Deg(90.0), aspect, 0.001, 1000.0);

            clear_window();

            three_d_rendering_setup();

            color_shader.use_program();
            color_shader.set_MVP(projection * view);

            cube.bind_vertex_array();
            cube.draw_elements();

            simple_shader.use_program();
            simple_shader.set_color(color);

            triangle.bind_vertex_array();
            triangle.draw_elements();

            window.swap();
        }
    }
}
