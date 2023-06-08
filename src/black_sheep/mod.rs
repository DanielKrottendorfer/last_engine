pub mod rendering;
mod window;

mod algorithms;
#[allow(dead_code)]
mod constants;
mod gameplay;
pub mod gamestate;
mod generators;
mod imgui_system;
mod loop_timing;
pub mod math;
mod q_i_square_root;
mod script;
pub mod settings;
mod setup;
mod torus;
mod transform;

mod softbody;

mod canvas;

use std::borrow::BorrowMut;





use cgmath::{Matrix4};


use imgui::{ColorPicker, Condition, Image, TextureId, Window};
use sdl2::mouse::MouseButton;

use window::window_util::*;
use window::SDLWindow;

use sdl2::event::{Event, WindowEvent};

use crate::black_sheep::rendering::loader::load_texture_from_path;


use crate::black_sheep::window::window_util::{clear_drawbuffer, set_viewport};

use gamestate::input_flags::InputFlags;
use imgui_system::ImguiSystem;
use rendering::geometry;

use rendering::shader;






pub struct BlackSheep {
    window: SDLWindow,
    pub input_flags: InputFlags,
    pub mouse_pos: [i32; 2],
    pub window_size_f32: [f32; 2],
    pub window_size_i32: [i32; 2],
    pub ui_projection: Matrix4<f32>,
    canvas: canvas::Canvas,
}

impl Drop for BlackSheep {
    fn drop(&mut self) {
        shader::cleanup();
        geometry::cleanup();
    }
}
pub fn run() {
    // KEEP THIS ORDER
    let window = SDLWindow::new();
    shader::init();
    geometry::init();
    // KEEP THIS ORDER
    unsafe {
        gl::LineWidth(0.5);
    }

    let _bb = setup::init_mesh().unwrap();

    let bs = BlackSheep {
        window,
        input_flags: InputFlags::default(),
        window_size_f32: settings::INIT_WINDOW_SIZE_F32,
        window_size_i32: settings::INIT_WINDOW_SIZE_I32,
        mouse_pos: [0, 0],
        ui_projection: cgmath::ortho(
            0.0,
            settings::INIT_WINDOW_SIZE_F32[0],
            settings::INIT_WINDOW_SIZE_F32[1],
            0.0,
            -1.0,
            1.0,
        ),
        canvas: canvas::Canvas::new(),
    };

    bs.run();
}

impl BlackSheep {
    fn update(&mut self) {
        let c = self.canvas.borrow_mut();
        c.build();
    }

    fn draw(&mut self) {
        self.canvas.borrow_mut().draw();
    }

    pub fn handle_events(&mut self, imgui_system: &mut ImguiSystem) {
        while let Some(event) = self.window.poll_event() {
            imgui_system.handle_event(&event);
            self.canvas.update(&event);
            match event {
                Event::Quit { .. } => {
                    self.input_flags.insert(InputFlags::CLOSE);
                }
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keycode {
                        use sdl2::keyboard::Keycode::*;
                        if let Escape = key {
                            self.input_flags.insert(InputFlags::CLOSE);
                        } else {
                            self.input_flags.key_down(key);
                        }
                    } else {
                        #[cfg(not(feature = "debug_off"))]
                        println!("No Valid KeyCode");
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = keycode {
                        self.input_flags.key_up(key);
                    } else {
                        #[cfg(not(feature = "debug_off"))]
                        println!("No Valid KeyCode");
                    }
                }
                Event::MouseButtonDown {
                    mouse_btn,   ..
                } => {
                    if MouseButton::Left == mouse_btn {
                        self.input_flags.left_mouse_down(true);
                    }
                    if MouseButton::Right == mouse_btn {
                        self.input_flags.right_mouse_down(true);
                        self.input_flags.insert(InputFlags::CAPTURED_MOUSE);
                        self.window.capture_mouse();
                    }
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    if MouseButton::Left == mouse_btn {
                        self.input_flags.left_mouse_down(false);
                    }
                    if MouseButton::Right == mouse_btn {
                        self.input_flags.right_mouse_down(false);
                        self.input_flags.remove(InputFlags::CAPTURED_MOUSE);
                        self.window.release_mouse();
                    }
                }
                Event::MouseMotion {
                      x, y, ..
                } => {
                    self.mouse_pos = [x, y];
                }
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(w, h) => {
                        set_viewport(w, h);
                        self.window_size_i32 = [w, h];
                        let wh = [w as f32, h as f32];
                        self.window_size_f32 = wh;

                        self.ui_projection = cgmath::ortho(0.0, wh[0], wh[1], 0.0, -1.0, 1.0);
                        let _aspect = (wh[0] - 300.0) / wh[1];
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }

    pub fn run(mut self) {
        let (imgui_shader_program, _gizmo_shader) = {
            let shader_repo = rendering::shader::get_shader_repo();
            (shader_repo.imgui, shader_repo.gizmo)
        };

        init_rendersetup();

        let mut imgui_system = imgui_system::init();

        let font_texture = imgui_system.load_font_atlas_texture();
        let nice_image = load_texture_from_path("./res/aP3DgOB_460swp.png").unwrap();

        let mut loop_timer = loop_timing::CatchupTimer::new();

        let _fps = 0;

        let mut t_color = [1.0, 0.0, 0.0, 1.0];

        let mut wiregrid = false;

        'mainloop: loop {
            //PROCESS INPUT
            self.handle_events(&mut imgui_system);
            if self.input_flags.contains(InputFlags::CLOSE) {
                break 'mainloop;
            }

            while loop_timer.should_update() {
                //UPDATE

                imgui_system.update(&mut |ui| {
                    use imgui::WindowFlags;

                    Window::new("Image")
                        .size([300.0, self.window_size_f32[1]], Condition::Always)
                        .position([self.window_size_f32[0] - 300.0, 0.0], Condition::Always)
                        .flags(
                            WindowFlags::NO_MOVE
                                | WindowFlags::NO_RESIZE
                                | WindowFlags::NO_COLLAPSE
                                | WindowFlags::NO_TITLE_BAR,
                        )
                        .build(&ui, || {
                            ui.text("Hello world!");
                            ui.text("こんにちは世界！");

                            let label = if wiregrid { "no wwiregrid" } else { "iregrid" };
                            if ui.button(label) {
                                wiregrid = !wiregrid;
                                gl_wiregrid(wiregrid);
                            }

                            ColorPicker::new("color_picker", &mut t_color).build(ui);
                            Image::new(TextureId::new(1 as usize), [300.0, 300.0]).build(ui);
                        });
                });
                //HANDLE INPUT
                self.update();

            }

            //RENDER

            three_d_rendering_setup();
            clear_color(0.0, 0.3, 0.3, 1.0);
            clear_drawbuffer();
            set_viewport(self.window_size_i32[0]-300, self.window_size_i32[1]);
            self.draw();

            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + 0);
                font_texture.bind();
                gl::ActiveTexture(gl::TEXTURE0 + 1);
                nice_image.bind();
            }

            set_viewport(self.window_size_i32[0], self.window_size_i32[1]);

            imgui_rendering_setup();

            imgui_shader_program.use_program();
            imgui_shader_program.set_matrix(self.ui_projection);
            imgui_system.draw();

            self.window.swap();
        }
    }
}
