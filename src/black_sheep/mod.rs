pub mod rendering;
mod window;

mod algorithms;
#[allow(dead_code)]
mod constants;
mod gamestate;
mod generators;
mod imgui_system;
mod q_i_square_root;
mod script;
pub mod settings;
mod setup;
mod transform;

use std::time::Duration;

use cgmath::Deg;
use gamestate::*;

use imgui::{ColorPicker, Condition, Image, TextureId, Window};
use imgui::{ImColor32, StyleVar};
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

use self::imgui_system::ImguiSystem;
use self::rendering::geometry;
use self::rendering::shader;
use self::script::{imgui_structogram, structogram};

pub struct BlackSheep {
    window: SDLWindow,
    game_state: GameState,
}

impl Drop for BlackSheep {
    fn drop(&mut self) {
        shader::cleanup();
        geometry::cleanup();
    }
}

impl BlackSheep {
    pub fn new() -> Self {
        // KEEP THIS ORDER
        let window = SDLWindow::new();
        geometry::init();
        rendering::shader::init();
        let game_state = GameState::new();
        Self { window, game_state }
    }

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
                            cgmath::perspective(Deg(90.0), aspect, 0.1, 1000.0);
                        //rt_main.resize(window_size_i32[0] - 300, window_size_i32[1]);
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

        //init_rendering_setup();

        init_rendersetup();

        let mut imgui_system = imgui_system::init();

        let rt_gizmo = rendering::rendertarget::RenderTarget::new(300, 300);
        rendertarget::unbind_framebuffer();

        let font_texture = imgui_system.load_font_atlas_texture();
        let nice_image = load_texture_from_path("./res/aP3DgOB_460swp.png").unwrap();

        let gizmo =
            geometry::get_mesh_repo(|mr| MeshToken::from(mr.get_mesh_by_name("gizmo").unwrap()));

        let time = std::time::Instant::now();
        let mut previous = time.elapsed();
        let mut lag = Duration::from_secs(0);
        let mut last = Duration::from_secs(0);

        let mut fps = 0;

        let mut t_color = [1.0, 0.0, 0.0, 1.0];

        let mut run_ui = false;
        let mut prune = false;

        let structogram = imgui_structogram::Structogram::new(script::another_script());

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
            self.handle_events(&mut imgui_system);
            if self.game_state.input_flags.contains(InputFlags::CLOSE) {
                break 'mainloop;
            }

            let game_state = &mut self.game_state;

            while lag >= MS_PER_UPDATE {
                //UPDATE

                imgui_system.update(&mut |ui| {
                    use imgui::WindowFlags;

                    let a = ui.push_style_var(StyleVar::FramePadding([0.0, 0.0]));
                    let w = Window::new("Test");
                    w.build(ui, || {

                        structogram.build(ui);

                        // let mut vmin = ui.window_content_region_min();
                        // vmin[0] += ui.window_pos()[0];
                        // vmin[1] += ui.window_pos()[1];
                        // let mut vmax = ui.window_content_region_max();
                        // vmax[0] += ui.window_pos()[0];
                        // vmax[1] += ui.window_pos()[1];
                        // let dl = ui.get_window_draw_list();
                        // dl.add_rect(
                        //     vmin, // [p[0]+10.0, p[1]+20.0],
                        //     vmax, //[p[0]+20.0, p[1]+30.0],
                        //     ImColor32::from_rgba(255, 0, 255, 255),
                        // )
                        // .filled(true)
                        // .build();

                        // // let mut c = [0.0,0.0];
                        // // c[0] += 20.0;
                        // // c[1] += 20.0;
                        // // ui.set_cursor_pos(c);
                        // println!("{:?}", ui.cursor_pos());
                        // println!("{:?}", ui.text_line_height_with_spacing());
                        // ui.text("text");

                        // println!("{:?}\n", ui.cursor_pos());
                    });
                    a.end();

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

                            let label = if run_ui { "stop" } else { "start" };
                            if ui.button(label) {
                                run_ui = !run_ui;
                            }
                            let label = if prune { "np prune" } else { "prune" };
                            if ui.button(label) {
                                prune = !prune;
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

                lag -= MS_PER_UPDATE;
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

            let i = lag.as_secs_f32() / MS_PER_UPDATE.as_secs_f32();

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

            game_state.draw_3d(i);

            set_viewport(game_state.window_size_i32[0], game_state.window_size_i32[1]);
            game_state.draw_ui(i);

            imgui_rendering_setup();

            imgui_shader_program.use_program();
            imgui_shader_program.set_matrix(game_state.ui_projection);
            imgui_system.draw();

            self.window.swap();
        }
    }
}
