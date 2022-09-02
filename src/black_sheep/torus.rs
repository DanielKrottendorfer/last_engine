use cgmath::{num_traits::Pow, Vector3};

#[inline]
pub fn torus_r(v: Vector3<f32>, r_mj: f32) -> f32 {
    f32::sqrt((r_mj - f32::sqrt(v.x.pow(2) + v.z.pow(2))).pow(2) + v.y.pow(2))
}
