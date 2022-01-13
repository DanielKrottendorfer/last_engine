

pub mod rendering;
mod window;

#[allow(dead_code)]
mod constants;

pub mod settings;

mod transform;

mod gamestate;
mod q_i_square_root;

mod imgui_system;

mod generators;

mod setup;

mod algorithms;

use std::time::Duration;

use cgmath::Matrix4;
use cgmath::Vector4;
use gamestate::*;

use cgmath::{Deg, Vector3};
use imgui::{ColorPicker, Condition, Image, TextureId, Window};
use rendering::geometry::MeshRepo;
use sdl2::mouse::MouseButton;
use window::window_util::*;
use window::SDLWindow;

use sdl2::event::{Event, WindowEvent};

use crate::black_sheep::rendering::geometry::MeshToken;
use crate::black_sheep::rendering::loader::load_texture_from_path;
use crate::black_sheep::rendering::rendertarget;
use crate::black_sheep::settings::*;
use crate::black_sheep::window::window_util::{clear_drawbuffer, set_viewport};


use self::gamestate::input_flags::InputFlags;
use self::generators::point_grid;
use self::rendering::geometry;
use self::rendering::shader;

pub struct BlackSheep {
    window: SDLWindow,
    mesh_repo: MeshRepo,
    game_state: GameState,
}

impl Drop for BlackSheep{
    fn drop(&mut self) {
        shader::cleanup();
        geometry::cleanup();
    }
}

impl BlackSheep {
    pub fn new() -> Self {
        // KEEP THIS ORDER
        let window = SDLWindow::new();
        let mesh_repo = MeshRepo::new();
        geometry::init();
        rendering::shader::init();
        let game_state = GameState::new();
        Self {
            window,
            mesh_repo,
            game_state,
        }
    }

    pub fn start(mut self) {
    }

    pub fn run(mut self) {
        let mut mesh_repo = &mut self.mesh_repo;
        let shader_repo = rendering::shader::get_shader_repo();

        //init_rendering_setup();
        let mut window_size_f32 = INIT_WINDOW_SIZE_F32;
        let mut window_size_i32 = INIT_WINDOW_SIZE_I32;

        let mut ui_projection =
            ui_projection_mat([INIT_WINDOW_SIZE_I32[0], INIT_WINDOW_SIZE_I32[1]]);

        init_rendersetup();

        let imgui_shader_program = &shader_repo.imgui;
        
        let mut imgui_system = imgui_system::init();

        let rt_gizmo = rendering::rendertarget::RenderTarget::new(300, 300);
        rendertarget::unbind_framebuffer();

        let font_texture = imgui_system.load_font_atlas_texture();
        let nice_image = load_texture_from_path("./res/aP3DgOB_460swp.png").unwrap();


        let gizmo_shader = shader_repo.gizmo;

        let gizmo = geometry::get_mesh_repo(|mr|{
            MeshToken::from(mr.get_mesh_by_name("gizmo").unwrap())
        });

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
            while let Some(event) = self.window.poll_event() {
                imgui_system.handle_event(&event);
                let input = &mut self.game_state.input_flags;
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
                                input.key_down(key);
                            }
                        } else {
                            #[cfg(not(feature = "debug_off"))]
                            println!("No Valid KeyCode");
                        }
                    }
                    Event::KeyUp { keycode, .. } => {
                        if let Some(key) = keycode {
                            input.key_up(key);
                        } else {
                            #[cfg(not(feature = "debug_off"))]
                            println!("No Valid KeyCode");
                        }
                    }
                    Event::MouseButtonDown { mouse_btn, .. } => {
                        if MouseButton::Right == mouse_btn {
                            input.insert(InputFlags::CAPTURED_MOUSE);
                            self.window.capture_mouse();
                        }
                    }
                    Event::MouseButtonUp { mouse_btn, .. } => {
                        if MouseButton::Right == mouse_btn {
                            input.remove(InputFlags::CAPTURED_MOUSE);
                            self.window.release_mouse();
                        }
                    }
                    Event::MouseMotion { xrel, yrel, .. } => {
                        self.game_state.on_mouse_motion(xrel, yrel);
                    }
                    Event::Window { win_event, .. } => match win_event {
                        WindowEvent::Resized(w, h) => {
                            set_viewport(w, h);
                            ui_projection = ui_projection_mat([w, h]);
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
                            ui.text(format!("{:?}", -self.game_state.cam.position));
                            ui.text(format!("{:#?}", self.game_state.cam.orientation));
                            ColorPicker::new("color_picker", &mut t_color).build(ui);
                            Image::new(TextureId::new(2 as usize), [300.0, 300.0])
                                .uv0([0.0, 1.0])
                                .uv1([1.0, 0.0])
                                .build(ui);
                            Image::new(TextureId::new(1 as usize), [300.0, 300.0]).build(ui);
                        });
                });
                //HANDLE INPUT
                
                self.game_state.update();

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


            let view = self.game_state.cam.get_i_view(i);


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
            set_viewport(window_size_i32[0] - 300, window_size_i32[1]);


            clear_color(0.0, 0.3, 0.3, 1.0);
            clear_drawbuffer();
            
            self.game_state.draw(i);

            ui_rendering_setup();

            set_viewport(window_size_i32[0], window_size_i32[1]);

            imgui_shader_program.use_program();
            imgui_shader_program.set_matrix(ui_projection);
            imgui_system.draw();

            self.window.swap();
        }
    }
}
