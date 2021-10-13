use cgmath::{Vector2, Vector3};

pub const SQUARE: [Vector3<f32>; 6] = [
    Vector3::new(-1.0, -1.0, 0.0),
    Vector3::new(1.0, -1.0, 0.0),
    Vector3::new(1.0, 1.0, 0.0),
    Vector3::new(-1.0, -1.0, 0.0),
    Vector3::new(-1.0, 1.0, 0.0),
    Vector3::new(1.0, 1.0, 0.0),
];


pub const UVS: [Vector2<f32>; 6] = [
    Vector2::new(0.0, 0.0),
    Vector2::new(1.0, 0.0),
    Vector2::new(1.0, 1.0),
    Vector2::new(0.0, 0.0),
    Vector2::new(0.0, 1.0),
    Vector2::new(1.0, 1.0),
];

pub const SQUARE_NORM: [Vector3<f32>; 6] = [
    Vector3::new(0.0, 0.0, 1.0),
    Vector3::new(0.0, 0.0, 1.0),
    Vector3::new(0.0, 0.0, 1.0),
    Vector3::new(0.0, 0.0, 1.0),
    Vector3::new(0.0, 0.0, 1.0),
    Vector3::new(0.0, 0.0, 1.0),
];

pub const ELEMENTS: [u32; 6] = [0,1,2,3,4,5];


pub const SIMPLE_TRIANGL: [Vector3<f32>; 3] = [
    Vector3::new(0.5, -0.5, -1.0),
    Vector3::new(0.0, 0.5, -1.0),
    Vector3::new(-0.5, -0.5, -1.0),
];

pub const TRIANGLE_ELEMENTS: [u32; 3] = [0,1,2];