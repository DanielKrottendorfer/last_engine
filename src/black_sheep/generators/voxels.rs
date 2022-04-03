use cgmath::Vector3;



pub fn voxel_grid(width: usize, height: usize, debth: usize, space: f32) -> (Vec<Vector3<f32>>,Vec<u32>) {

    let mut voxels = Vec::new();
    let mut elements = Vec::new();
    let mut i = 0;
    for w in 0..width {
        let w_f = w as f32;
        for h in 0..height{
            let h_f = h as f32;
            for d in 0..debth{
                let d_f = d as f32;
                voxels.push(Vector3::new(space * w_f,space * h_f,space * d_f));
                elements.push(i);
                i+=1;
            }
        }
    }

    (voxels,elements)
}
