use std::borrow::{BorrowMut};

use cgmath::{Vector2, Vector3, Zero};

use super::{
    rendering::{self, geometry::mesh::Mesh, shader::shader_structs::SimpleShaderProgram},
    settings,
};

const KEY_P: usize = 2;

mod uitl;

#[derive(Clone,Copy, Debug)]
struct Annotation {
    bbox: (Vector2<f32>, Vector2<f32>),
    keyp: [Vector2<f32>; KEY_P],
}

#[derive(Clone, Debug)]
enum AnnoState {
    BoxStart,
    BoxEnd,
    KeyPoint(usize),
}

pub struct Canvas {
    lines: Vec<Vector2<f32>>,
    dots: Vec<Vector2<f32>>,

    line_mesh: Option<Mesh>,
    dot_mesh: Option<Mesh>,

    canvas_shader: SimpleShaderProgram,

    canvas_size: [i32; 2],

    mouse_pos: Vector2<f32>,
    annos: Vec<Annotation>,
    anno_state: AnnoState,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            dots: Vec::new(),
            line_mesh: None,
            dot_mesh: None,
            canvas_shader: rendering::shader::get_shader_repo().simple,
            canvas_size: [
                settings::INIT_WINDOW_SIZE_I32[0] - 300,
                settings::INIT_WINDOW_SIZE_I32[1],
            ],
            mouse_pos: Vector2::zero(),
            annos: Vec::new(),
            anno_state: AnnoState::BoxStart,
        }
    }

    pub fn update(&mut self, event: &sdl2::event::Event) {
        match event {
            sdl2::event::Event::MouseButtonDown {
                x,
                y,
                ..
            } => match &mut self.anno_state {
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
            },
            sdl2::event::Event::MouseMotion {
                x,
                y,
                ..
            } => {

                let mouse_pos = Vector2{x: *x as f32, y: *y as f32};

                match self.anno_state {
                    AnnoState::BoxEnd => {
                        self.annos.last_mut().map(|a| {
                            a.bbox.1 = mouse_pos;
                        });
                    },
                    _ => ()
                }

                self.mouse_pos = mouse_pos;
            }
            sdl2::event::Event::MouseButtonUp {
                ..
            } => match &mut self.anno_state {
                AnnoState::BoxEnd => {
                    self.anno_state = AnnoState::KeyPoint(0);
                }
                _ => println!("UNDIFINED UP"),
            },
            _ => (),
        }
    }



    pub fn build(&mut self) {
        
        self.add_cross();
        
        for i in 0..self.annos.len() {
            let a = &self.annos[i];
            self.dots.extend(a.keyp);
            self.add_square(a.bbox.0, a.bbox.1);
        }

        if self.lines.len() > 0 {
            if let Some(l) = self.line_mesh.borrow_mut() {
                l.update_buffer(&self.lines, 0)
            } else {
                let mut m = Mesh::new();
                m.add_floatbuffer(&self.lines, 0, 2);
                self.line_mesh = Some(m);
            }
            self.lines.clear();
        }

        if self.dots.len() > 0 {
            if let Some(d) = self.dot_mesh.borrow_mut() {
                d.update_buffer(&self.dots, 0);
            } else {
                let mut m = Mesh::new();
                m.add_floatbuffer(&self.dots, 0, 2);
                self.dot_mesh = Some(m);
            }
            self.dots.clear();
        }
    }

    pub fn draw(&mut self) {
        self.canvas_shader.use_program();
        self.canvas_shader.set_color(Vector3::new(0.0, 1.0, 1.0));
        self.canvas_shader.set_proj(cgmath::ortho(
            0.0,
            self.canvas_size[0] as f32,
            self.canvas_size[1] as f32,
            0.0,
            0.0,
            10.0,
        ));

        let anno_len = self.annos.len();
        self.line_mesh.as_mut().map(|l| {
            l.bind_vertex_array();
            l.vertex_count = 4 + (anno_len as i32 * 8 );
            l.draw_line_array();
        });

        self.dot_mesh.as_mut().map(|d| {
            d.bind_vertex_array();
            d.vertex_count = (KEY_P * anno_len) as i32;
            d.draw_point_array();

        });
    }
}
