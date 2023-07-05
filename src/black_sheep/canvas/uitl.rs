use cgmath::{Vector2, Vector3};

use super::{Annotation, Canvas};

impl Canvas {
    fn add_line(&mut self, start: Vector2<f32>, end: Vector2<f32>) {
        self.lines.push(start);
        self.lines.push(end);
    }

    fn add_dot(&mut self, pos: Vector2<f32>) {
        self.dots.push(pos);
    }

    pub fn add_cross(&mut self) {
        self.add_line(
            Vector2 {
                x: 0.0,
                y: self.mouse_pos.y,
            },
            Vector2 {
                x: self.canvas_size[0] as f32,
                y: self.mouse_pos.y,
            },
        );
        self.add_line(
            Vector2 {
                x: self.mouse_pos.x,
                y: 0.0,
            },
            Vector2 {
                x: self.mouse_pos.x,
                y: self.canvas_size[1] as f32,
            },
        );
        self.l_colors.extend(
            [Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }; 4],
        );
    }

    pub fn add_square(&mut self, start: Vector2<f32>, end: Vector2<f32>) {
        self.add_line(
            start,
            Vector2 {
                x: start.x,
                y: end.y,
            },
        );
        self.add_line(
            start,
            Vector2 {
                x: end.x,
                y: start.y,
            },
        );
        self.add_line(
            end,
            Vector2 {
                x: start.x,
                y: end.y,
            },
        );
        self.add_line(
            end,
            Vector2 {
                x: end.x,
                y: start.y,
            },
        );
    }
}

pub fn rotate_anno90(anno: &mut Annotation) {
    let m = cgmath::Matrix2::<f32>::from_angle(cgmath::Deg(90.0));
    anno.bbox.0 = Vector2::new(640.0, 0.0) + m * anno.bbox.0;
    anno.bbox.1 = Vector2::new(640.0, 0.0) + m * anno.bbox.1;

    println!("{:?}", m * anno.bbox.0);
    println!("{:?}", m * anno.bbox.1);

    anno.keyp.iter_mut().for_each(|kp| {
        *kp = Vector2::new(640.0, 0.0) + m * *kp;
    });
}
