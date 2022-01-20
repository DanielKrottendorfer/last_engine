use std::mem;

use cgmath::{InnerSpace, Vector3};

#[inline]
pub fn q_i_square_root(f: f32) -> f32 {
    // Magic number based on Chris Lomont work:
    // const MAGIC_U32: u32 = 0x5f375a86;
    // The Original Magic Number:
    // const MAGIC_32: u32 = 0x5f3759df;
    const THREEHALFS: f32 = 1.5f32;
    let x2 = f * 0.5;
    let mut i: u32 = unsafe { mem::transmute(f) }; // evil floating point bit level hacking
    i = 0x5f375a86 - (i >> 1); // what the fuck?
    let y: f32 = unsafe { mem::transmute(i) };
    let y = y * (THREEHALFS - (x2 * y * y)); // 1st iteration
                                             //	y  = y * ( threehalfs - ( x2 * y * y ) );       // 2nd iteration, this can be removed

    return y;
}

#[inline]
pub fn q_normalize(v: Vector3<f32>) -> Vector3<f32> {
    v * q_i_square_root(v.magnitude2())
}
