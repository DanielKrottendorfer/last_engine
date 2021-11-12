use cgmath::{Vector3, Vector4};

use super::q_i_square_root::q_normalize;

pub fn point_cube(size: i32) -> (Vec<Vector3<f32>>, Vec<Vector4<f32>>, Vec<u32>) {
    let mut v = Vec::new();
    let mut c = Vec::new();
    let mut e = Vec::new();

    let s = size as f32;

    let a = (1.0 / 3.0 as f32).sqrt();
    let b = (3.0 / 4.0 as f32).sqrt();
    let h = (2.0 / 3.0 as f32).sqrt();

    let mut i = 0;

    for x in 0..size {
        for y in 0..size {
            for z in 0..size {
                let x_offset = if z % 2 == 0 { 0.5 } else { 0.0 };

                let z_offset = if y % 2 == 0 { a } else { 0.0 };

                let t = Vector3::new(x as f32 + x_offset, y as f32 * h, (z as f32 * b) + z_offset);

                v.push(t / s);
                c.push(q_normalize(t).extend(1.0));
                e.push(i);
                i += 1;
            }
        }
    }
    (v, c, e)
}
