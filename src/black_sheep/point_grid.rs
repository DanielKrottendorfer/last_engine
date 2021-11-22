use cgmath::Vector2;

pub fn new_point_grid(lines: i32, columns: i32, scale: i32) -> (Vec<Vector2<i32>>, Vec<u32>) {
    let mut v = Vec::new();
    let mut e = Vec::new();
    let mut i = 0;

    for c in 0..columns {
        for l in 0..lines {
            v.push(Vector2::new((c * scale) / columns, (l * scale) / lines));
            e.push(i);
            i += 1;
        }
    }

    (v, e)
}
