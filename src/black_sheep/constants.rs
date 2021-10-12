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