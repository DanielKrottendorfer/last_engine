use cgmath::Vector3;

pub fn circel(pont_c: u32, rad: f32) -> (Vec<Vector3<f32>>, Vec<u32>) {
    let segment = (std::f32::consts::TAU) / pont_c as f32;

    let mut points = Vec::new();
    let mut elements = Vec::new();

    for i in 0..pont_c {
        points.push(Vector3::new(
            f32::sin(segment * i as f32) * rad,
            0.0,
            f32::cos(segment * i as f32) * rad,
        ));
        elements.push(i);
        elements.push((i + 1) % pont_c);
    }

    (points, elements)
}
