use std::{borrow::BorrowMut, io::Write};

use cgmath::{Vector2, Vector3, Zero};
use imgui::Ui;

use super::{
    rendering::{
        self,
        geometry::mesh::Mesh,
        loader::{load_texture_from_path},
        shader::shader_structs::{CanvasImageShader, SimpleShaderProgram},
        Texture,
    },
    settings,
};

const KEY_P: usize = 2;

mod uitl;

#[derive(Clone, Copy, Debug)]
struct Annotation {
    class: i32,
    bbox: (Vector2<f32>, Vector2<f32>),
    keyp: [Vector2<f32>; KEY_P],
}

#[derive(Clone, Debug)]
pub enum AnnoState {
    BoxStart,
    BoxEnd,
    KeyPoint(usize),
}

pub struct Canvas {
    lines: Vec<Vector2<f32>>,
    l_colors: Vec<Vector3<f32>>,
    dots: Vec<Vector2<f32>>,
    d_colors: Vec<Vector3<f32>>,

    line_mesh: Option<Mesh>,
    dot_mesh: Option<Mesh>,

    image_mesh: Mesh,

    canvas_shader: SimpleShaderProgram,
    canvas_image_shader: CanvasImageShader,
    texture: Texture,

    canvas_size: [i32; 2],

    mouse_pos: Vector2<f32>,
    annos: Vec<Annotation>,
    pub anno_state: AnnoState,

    pub current_class: i32,
    pub current_file_string: String,
}

const SQUARE: [f32; 8] = [
    0.0,
    settings::INIT_WINDOW_SIZE_F32[1],
    settings::INIT_WINDOW_SIZE_F32[0] - 300.0,
    settings::INIT_WINDOW_SIZE_F32[1],
    0.0,
    0.0,
    settings::INIT_WINDOW_SIZE_F32[0] - 300.0,
    0.0,
];

const SQUARE2: [f32; 8] = [0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0];

const S_ELEM: [u32; 6] = [0, 1, 2, 1, 2, 3];

impl Canvas {
    pub fn new() -> Self {
        let mut image_mesh = Mesh::new();
        image_mesh.add_floatbuffer(&SQUARE, 0, 2);
        image_mesh.add_floatbuffer(&SQUARE2, 1, 2);
        image_mesh.add_elementarraybuffer(&S_ELEM);

        Self {
            lines: Vec::new(),
            l_colors: Vec::new(),
            dots: Vec::new(),
            d_colors: Vec::new(),
            line_mesh: None,
            dot_mesh: None,
            canvas_shader: rendering::shader::get_shader_repo().simple,
            canvas_image_shader: rendering::shader::get_shader_repo().canvas_image,
            canvas_size: [
                settings::INIT_WINDOW_SIZE_I32[0] - 300,
                settings::INIT_WINDOW_SIZE_I32[1],
            ],
            mouse_pos: Vector2::zero(),
            annos: Vec::new(),
            anno_state: AnnoState::BoxStart,
            image_mesh,
            texture: load_texture_from_path("./res/aP3DgOB_460swp.png").unwrap(),
            current_class: 0,
            current_file_string: String::new(),
        }
    }

    pub fn handle_event(&mut self, event: &sdl2::event::Event) {
        match event {
            sdl2::event::Event::DropFile {
                timestamp: _,
                window_id: _,
                filename,
            } => {
                if filename.ends_with(".png") {
                    self.texture = load_texture_from_path(&filename).unwrap();
                    self.current_file_string = filename.clone();
                    self.reset();
                }
            }
            sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                if *x > self.canvas_size[0] {
                    return;
                }

                match &mut self.anno_state {
                    AnnoState::BoxStart => {
                        let anno = Annotation {
                            bbox: (
                                Vector2 {
                                    x: *x as f32,
                                    y: *y as f32,
                                },
                                Vector2 {
                                    x: *x as f32,
                                    y: *y as f32,
                                },
                            ),
                            keyp: [Vector2::<f32>::unit_x(); KEY_P],
                            class: self.current_class,
                        };

                        self.annos.push(anno);
                        self.anno_state = AnnoState::BoxEnd;
                    }
                    AnnoState::KeyPoint(i) => {
                        self.annos.last_mut().map(|a| {
                            a.keyp[*i] = Vector2 {
                                x: *x as f32,
                                y: *y as f32,
                            };
                        });

                        *i += 1;
                        if *i >= KEY_P {
                            self.anno_state = AnnoState::BoxStart;
                        }
                    }
                    _ => println!("UNDIFINED DOWN"),
                }
            }
            sdl2::event::Event::MouseMotion { x, y, .. } => {
                if *x > self.canvas_size[0] {
                    return;
                }

                let mouse_pos = Vector2 {
                    x: *x as f32,
                    y: *y as f32,
                };

                match self.anno_state {
                    AnnoState::BoxEnd => {
                        self.annos.last_mut().map(|a| {
                            a.bbox.1 = mouse_pos;
                        });
                    }
                    _ => (),
                }

                self.mouse_pos = mouse_pos;
            }
            sdl2::event::Event::MouseButtonUp { x, .. } => {
                if *x > self.canvas_size[0] {
                    return;
                }
                match &mut self.anno_state {
                    AnnoState::BoxEnd => {
                        self.anno_state = AnnoState::KeyPoint(0);
                    }
                    _ => println!("UNDIFINED UP"),
                }
            }
            _ => (),
        }
    }

    pub fn build(&mut self) {
        self.add_cross();

        for i in 0..self.annos.len() {
            let a = &self.annos[i];

            let color = match a.class {
                0 => Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 0.0,
                },
                1 => Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 1.0,
                },
                2 => Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 1.0,
                },
                _ => Vector3::zero(),
            };

            self.dots.extend(a.keyp);
            self.add_square(a.bbox.0, a.bbox.1);
            self.l_colors.extend([color; 8]);
            self.d_colors.extend([Vector3::unit_x(), Vector3::unit_z()]);
        }

        if self.lines.len() > 0 {
            if let Some(l) = self.line_mesh.borrow_mut() {
                l.update_buffer(&self.lines, 0);
                l.update_buffer(&self.l_colors, 1);
            } else {
                let mut m = Mesh::new();
                m.add_floatbuffer(&self.lines, 0, 2);
                m.add_floatbuffer(&self.l_colors, 1, 3);
                self.line_mesh = Some(m);
            }
            self.lines.clear();
            self.l_colors.clear();
        }

        if self.dots.len() > 0 {
            if let Some(d) = self.dot_mesh.borrow_mut() {
                d.update_buffer(&self.dots, 0);
                d.update_buffer(&self.d_colors, 1);
            } else {
                let mut m = Mesh::new();
                m.add_floatbuffer(&self.dots, 0, 2);
                m.add_floatbuffer(&self.d_colors, 1, 3);
                self.dot_mesh = Some(m);
            }
            self.dots.clear();
            self.d_colors.clear();
        }
    }

    pub fn build_ui(&mut self, ui: &Ui) {
        ui.separator();
        if ui.button("reset") {
            self.reset();
        }
        ui.indent();
        let mut delete = None;
        for (i, anno) in self.annos.iter_mut().enumerate() {
            let _id = ui.push_id(i as i32);
            if ui.button("delete") {
                delete = Some(i);
            }

            let mut v: [f32; 2] = anno.bbox.0.into();
            ui.input_float2("bbox1", v.borrow_mut()).build();
            anno.bbox.0 = v.into();

            let mut v: [f32; 2] = anno.bbox.1.into();
            ui.input_float2("bbox2", v.borrow_mut()).build();
            anno.bbox.1 = v.into();

            for (ii, kp) in anno.keyp.iter_mut().enumerate() {
                let _iid = ui.push_id(ii as i32);
                let mut v: [f32; 2] = (*kp).into();
                ui.input_float2("keyp", &mut v).build();
                *kp = v.into();
            }
            ui.separator();
        }

        if let Some(d) = delete {
            println!("remove {}", d);
            self.annos.remove(d);
        }

        ui.unindent();
    }

    pub fn draw(&mut self) {
        let proj = cgmath::ortho(
            0.0,
            self.canvas_size[0] as f32,
            self.canvas_size[1] as f32,
            0.0,
            0.0,
            10.0,
        );

        self.canvas_shader.use_program();
        self.canvas_shader.set_proj(proj);

        let anno_len = self.annos.len();
        self.line_mesh.as_mut().map(|l| {
            l.bind_vertex_array();
            l.vertex_count = 4 + (anno_len as i32 * 8);
            l.draw_line_array();
        });

        self.dot_mesh.as_mut().map(|d| {
            d.bind_vertex_array();
            d.vertex_count = (KEY_P * anno_len) as i32;
            d.draw_point_array();
        });

        self.canvas_image_shader.use_program();
        self.canvas_image_shader.set_proj(proj);
        self.image_mesh.bind_vertex_array();

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + 0);
            self.texture.bind();
        }

        self.canvas_image_shader.set_image1(0);
        self.image_mesh.draw_triangle_elements();
    }

    pub fn reset(&mut self) {
        self.lines.clear();
        self.dots.clear();
        self.l_colors.clear();
        self.d_colors.clear();
        self.annos.clear();
        self.anno_state = AnnoState::BoxStart;
        self.current_class = 0;
    }

    pub fn export(&self) {
        let c_dim = Vector2 {
            x: self.canvas_size[0] as f32,
            y: self.canvas_size[1] as f32,
        };
        let out_str = self
            .annos
            .iter()
            .map(|anno| {
                let box_center = (anno.bbox.0 + anno.bbox.1) / 2.0;
                let box_center = Vector2 {
                    x: box_center.x / c_dim.x,
                    y: box_center.y / c_dim.y,
                };

                let box_dim = anno.bbox.0 - anno.bbox.1;
                let box_dim = Vector2 {
                    x: box_dim.x.abs() / c_dim.x,
                    y: box_dim.y.abs() / c_dim.y,
                };

                let kp = anno
                    .keyp
                    .iter()
                    .map(|kp| format!("{} {} 2.0 ", kp.x / c_dim.x, kp.y / c_dim.y))
                    .collect::<String>();

                format!(
                    "{} {} {} {} {} {}\n",
                    anno.class, box_center.x, box_center.y, box_dim.x, box_dim.y, kp
                )
            })
            .collect::<String>();

        let txt_path = self.current_file_string.replace(".png", ".txt");
        let file = std::fs::File::create(&txt_path);

        match file {
            Ok(mut f) => {
                f.write_all(out_str.as_bytes()).unwrap();
            }
            Err(e) => println!("{}", e),
        }
    }
}
