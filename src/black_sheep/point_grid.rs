use cgmath::Vector2;

pub fn new_point_grid(lines: u32, columns: u32, scale: f32) -> (Vec<Vector2<f32>>, Vec<u32>) {
    let lines_f32 = lines as f32;
    let columns_f32 = columns as f32;
    let mut v = Vec::new();
    let mut e = Vec::new();
    let mut i = 0;

    for c in 0..columns {
        for l in 0..lines {
            v.push(Vector2::new(c as f32 / columns_f32, l as f32 / lines_f32) * scale);
            e.push(i);
            i += 1;
        }
    }

    (v, e)
}
