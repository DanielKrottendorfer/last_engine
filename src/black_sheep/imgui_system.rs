use imgui::{Context, FontConfig, FontGlyphRanges, FontSource, Key, TextureId, Ui};
use sdl2::event::Event;

use crate::black_sheep::settings::INIT_WINDOW_SIZE_F32;

use super::rendering::{
    geometry::imgui_mesh::{imguimesh_from_drawdata, ImguiMesh},
    loader, Texture,
};

pub struct ImguiSystem {
    pub imgui: Context,
    frame_update_counter: u8,
    mesh_vec: Vec<ImguiMesh>,
}

impl ImguiSystem {
    pub fn load_font_atlas_texture(&mut self) -> Texture {
        let mut fonts = self.imgui.fonts();
        let atlas = fonts.build_rgba32_texture();
        loader::load_texture_fontatlas(&atlas)
    }

    #[allow(unused_variables)]
    pub fn handle_event(&mut self, event: &Event) {
        let io = self.imgui.io_mut();

        match event {
            Event::Window {
                timestamp,
                window_id,
                win_event,
            } => match win_event {
                sdl2::event::WindowEvent::Resized(x, y) => {
                    io.display_size = [*x as f32, *y as f32];
                }
                _ => (),
            },
            Event::KeyDown {
                timestamp,
                window_id,
                keycode,
                scancode,
                keymod,
                repeat,
            } => {
                if let Some(k) = keycode {
                    io.add_input_character(*k as u8 as char);
                    io.keys_down.get_mut(*k as i32 as usize).map(|d| *d = true);
                }
            }
            Event::KeyUp {
                timestamp,
                window_id,
                keycode,
                scancode,
                keymod,
                repeat,
            } => {
                if let Some(k) = keycode {
                    io.keys_down.get_mut(*k as i32 as usize).map(|d| *d = false);
                }
            }
            Event::MouseMotion {
                timestamp,
                window_id,
                which,
                mousestate,
                x,
                y,
                xrel,
                yrel,
            } => {
                io.mouse_pos = [*x as f32, *y as f32];
            }
            Event::MouseButtonDown {
                timestamp,
                window_id,
                which,
                mouse_btn,
                clicks,
                x,
                y,
            } => match mouse_btn {
                sdl2::mouse::MouseButton::Unknown => {}
                sdl2::mouse::MouseButton::Left => io.mouse_down[0] = true,
                sdl2::mouse::MouseButton::Right => io.mouse_down[1] = true,
                sdl2::mouse::MouseButton::Middle => io.mouse_down[2] = true,
                sdl2::mouse::MouseButton::X1 => io.mouse_down[3] = true,
                sdl2::mouse::MouseButton::X2 => io.mouse_down[4] = true,
            },
            Event::MouseButtonUp {
                timestamp,
                window_id,
                which,
                mouse_btn,
                clicks,
                x,
                y,
            } => match mouse_btn {
                sdl2::mouse::MouseButton::Unknown => {}
                sdl2::mouse::MouseButton::Left => io.mouse_down[0] = false,
                sdl2::mouse::MouseButton::Right => io.mouse_down[1] = false,
                sdl2::mouse::MouseButton::Middle => io.mouse_down[2] = false,
                sdl2::mouse::MouseButton::X1 => io.mouse_down[3] = false,
                sdl2::mouse::MouseButton::X2 => io.mouse_down[4] = false,
            },
            Event::MouseWheel {
                timestamp,
                window_id,
                which,
                x,
                y,
                direction,
            } => {
                io.mouse_wheel += *y as f32;
            }
            _ => (),
        }

        if io.want_capture_keyboard || io.want_capture_mouse {
            self.reset_update_frame_counter();
        }
    }

    pub fn reset_update_frame_counter(&mut self) {
        self.frame_update_counter = 3;
    }

    pub fn update<F: FnMut(&Ui)>(&mut self, run_ui: &mut F) {
        let ui = self.imgui.frame();
        run_ui(&ui);

        let draw_data = ui.render();

        if draw_data.draw_lists_count() != self.mesh_vec.len() {
            self.mesh_vec = imguimesh_from_drawdata(draw_data);
        } else {
            let draw_list = draw_data.draw_lists();
            for (mesh, drawdata) in self.mesh_vec.iter_mut().zip(draw_list) {
                mesh.update_vertex_buffer(drawdata);
            }
        }
    }

    pub fn draw(&self)
    {
        self.mesh_vec.iter().for_each(|mesh| {
            mesh.bind_vertex_array();
            mesh.draw(self.imgui.io().display_size);
        });
    }
}

pub fn init() -> ImguiSystem {
    let mut imgui = Context::create();

    imgui.io_mut()[Key::Backspace] = sdl2::keyboard::Keycode::Backspace as u32;

    let hidpi_factor = 1.0;
    let font_size = 13.0 * hidpi_factor;
    imgui.fonts().add_font(&[
        FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: font_size,
                ..FontConfig::default()
            }),
        },
        FontSource::TtfData {
            data: include_bytes!("../../res/mplus-1p-regular.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                rasterizer_multiply: 1.75,
                glyph_ranges: FontGlyphRanges::japanese(),
                ..FontConfig::default()
            }),
        },
    ]);

    imgui.fonts().tex_id = TextureId::new(1);

    imgui.io_mut().font_global_scale = 1.0 / hidpi_factor;

    imgui.io_mut().display_size = INIT_WINDOW_SIZE_F32;
    ImguiSystem {
        imgui,
        frame_update_counter: 2,
        mesh_vec: Vec::new(),
    }
}
