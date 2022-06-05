use cgmath::Vector3;

pub fn voxel_grid(
    width: usize,
    height: usize,
    debth: usize,
    voxel_size: f32,
) -> (Vec<Vector3<f32>>, Vec<u32>) {
    let offset = -Vector3::new(
        width as f32 * voxel_size / 2.0,
        height as f32 * voxel_size / 2.0,
        debth as f32 * voxel_size / 2.0,
    );

    let mut voxels = Vec::new();
    let mut elements = Vec::new();
    let mut i = 0;
    for w in 0..width {
        let w_f = w as f32;
        for h in 0..height {
            let h_f = h as f32;
            for d in 0..debth {
                let d_f = d as f32;
                voxels.push(
                    Vector3::new(voxel_size * w_f, voxel_size * h_f, voxel_size * d_f) + offset,
                );
                elements.push(i);
                i += 1;
            }
        }
    }

    (voxels, elements)
}
