pub mod rendering;
pub mod window;

mod algorithms;
#[allow(dead_code)]
mod constants;
pub mod ecs;
pub mod gamestate;
mod generators;
pub mod imgui_system;
mod loop_timing;
mod q_i_square_root;
mod script;
pub mod settings;
pub mod setup;
mod torus;
mod transform;

mod gl_debug;

use cgmath::Matrix4;
use cgmath::{Deg, Vector2};
use gamestate::*;

use imgui::{ColorPicker, Condition, Image, TextureId, Window};
use sdl2::mouse::MouseButton;

use window::window_util::*;
use window::SDLWindow;

use sdl2::event::{Event, WindowEvent};

use crate::black_sheep::rendering::loader::load_texture_from_path;
use crate::black_sheep::rendering::rendertarget;

use crate::black_sheep::window::window_util::{clear_drawbuffer, set_viewport};

use gamestate::input_flags::InputFlags;
use imgui_system::ImguiSystem;
use rendering::geometry;
use rendering::geometry::mesh::MeshToken;
use rendering::shader;

use camera::structs::FlyingEye;

pub trait UpdateFunction = FnMut(&mut GameState, &mut ImguiSystem);
pub trait DrawFunction = FnMut(f32, &GameState);

pub struct Logic<U: UpdateFunction, D: DrawFunction> {
    pub update: U,
    pub draw: D,
}

pub struct BlackSheep<U, D>
where
    U: UpdateFunction,
    D: DrawFunction,
{
    window: SDLWindow,
    game_state: GameState,
    rel_mouse_pos: Vector2<f32>,
    logic: Logic<U, D>,
    pub ecs: ecs::CHAINED_ECS,
}

impl<U: UpdateFunction, D: DrawFunction> Drop for BlackSheep<U, D> {
    fn drop(&mut self) {
        shader::cleanup();
        geometry::cleanup();
    }
}

pub fn run<U, D, FL>(mut f_logic: FL)
where
    U: UpdateFunction,
    D: DrawFunction,
    FL: FnMut(&mut ecs::CHAINED_ECS) -> Logic<U, D>,
{
    // KEEP THIS ORDER
    let window = SDLWindow::new();
    shader::init();
    geometry::init();

    #[cfg(feature = "gl_debug")]
    gl_debug::setup_debug();

    let game_state = GameState::new();
    let mut ecs = ecs::CHAINED_ECS::new();
    let logic = (f_logic)(&mut ecs);

    let bs = BlackSheep {
        window,
        game_state,
        rel_mouse_pos: Vector2::new(0.0, 0.0),
        logic,
        ecs,
    };

    bs.run();
}

impl<U, D> BlackSheep<U, D>
where
    U: UpdateFunction,
    D: DrawFunction,
{
    pub fn handle_events(&mut self, imgui_system: &mut ImguiSystem) {
        while let Some(event) = self.window.poll_event() {
            imgui_system.handle_event(&event);
            let game_state = &mut self.game_state;
            let window = &mut self.window;
            game_state.inputs.add_inputs(|input_flags| match event {
                Event::Quit { .. } => {
                    input_flags.insert(InputFlags::CLOSE);
                }
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keycode {
                        use sdl2::keyboard::Keycode::*;
                        if let Escape = key {
                            input_flags.insert(InputFlags::CLOSE);
                        } else {
                            input_flags.insert(InputFlags::from(key));
                        }
                    } else {
                        #[cfg(not(feature = "debug_off"))]
                        println!("No Valid KeyCode");
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = keycode {
                        input_flags.remove(InputFlags::from(key));
                    } else {
                        #[cfg(not(feature = "debug_off"))]
                        println!("No Valid KeyCode");
                    }
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    if MouseButton::Right == mouse_btn {
                        input_flags.remove(InputFlags::CAPTURED_MOUSE);
                        window.release_mouse();
                    }
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    if MouseButton::Right == mouse_btn {
                        input_flags.insert(InputFlags::CAPTURED_MOUSE);
                        window.capture_mouse();
                    }
                }
                _ => (),
            });

            match event {
                Event::MouseMotion {
                    xrel, yrel, x, y, ..
                } => {
                    self.rel_mouse_pos = Vector2::new(x as f32, y as f32);
                    game_state.on_mouse_motion(xrel, yrel, x, y);
                }
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(w, h) => {
                        game_state.window_size_i32 = [w, h];
                        let wh = [w as f32, h as f32];
                        game_state.window_size_f32 = wh;

                        game_state.ui_projection = cgmath::ortho(0.0, wh[0], wh[1], 0.0, -1.0, 1.0);
                        let aspect = (wh[0] - 300.0) / wh[1];
                        game_state.world_projection =
                            cgmath::perspective(Deg(80.0), aspect, 0.1, 1000.0);
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }

    pub fn run(mut self) {
        let (imgui_shader_program, gizmo_shader, _three_d) = {
            let shader_repo = rendering::shader::get_shader_repo();
            (shader_repo.imgui, shader_repo.gizmo, shader_repo.color_3d)
        };

        init_rendersetup();

        let mut imgui_system = imgui_system::init();

        let font_texture = imgui_system.load_font_atlas_texture();
        let nice_image = load_texture_from_path("./res/1322615842122.jpg").unwrap();

        let mut loop_timer = loop_timing::CatchupTimer::new();

        let _fps = 0;

        'mainloop: loop {
            //PROCESS INPUT
            self.handle_events(&mut imgui_system);
            if self.game_state.inputs.key_down(InputFlags::CLOSE) {
                break 'mainloop;
            }

            let mut game_state = &mut self.game_state;

            while loop_timer.should_update() {
                //UPDATE

                //HANDLE INPUT

                game_state.update();

                (self.logic.update)(&mut game_state, &mut imgui_system);
            }

            //RENDER
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + 0);
                font_texture.bind();
                gl::ActiveTexture(gl::TEXTURE0 + 1);
                nice_image.bind();
            }

            let i = loop_timer.get_iv();

            let view = game_state.cam.get_i_view(i);

            set_viewport(
                game_state.window_size_i32[0] - 300,
                game_state.window_size_i32[1],
            );

            three_d_rendering_setup();

            clear_color(0.0, 0.3, 0.3, 1.0);
            clear_drawbuffer();

            (self.logic.draw)(i, &game_state);

            set_viewport(game_state.window_size_i32[0], game_state.window_size_i32[1]);
            imgui_rendering_setup();

            imgui_shader_program.use_program();
            imgui_shader_program.set_matrix(game_state.ui_projection);
            imgui_system.draw();

            self.window.swap();
        }
    }
}
