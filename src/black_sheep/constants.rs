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
pub const ELEMENTS: [u32; 6] = [0, 1, 2, 3, 4, 5];

pub const SIMPLE_TRIANGL: [Vector3<f32>; 3] = [
    Vector3::new(-1.0, 1.0, -1.0),
    Vector3::new(-1.0, 0.5, -1.0),
    Vector3::new(-0.5, 1.0, -1.0),
];
pub const SIMPLE_TRIANGL_COLORS: [Vector3<f32>; 3] = [
    Vector3::new(1.0, 0.0, 0.0),
    Vector3::new(1.0, 0.0, 1.0),
    Vector3::new(1.0, 1.0, 1.0),
];
pub const TRIANGLE_ELEMENTS: [u32; 3] = [0, 1, 2];

pub const CUBE: [Vector3<f32>; 8] = [
    Vector3::new(0.0, 0.0, 0.0),
    Vector3::new(0.0, 0.0, 1.0),
    Vector3::new(1.0, 0.0, 1.0),
    Vector3::new(1.0, 0.0, 0.0),
    Vector3::new(0.0, 1.0, 0.0),
    Vector3::new(0.0, 1.0, 1.0),
    Vector3::new(1.0, 1.0, 1.0),
    Vector3::new(1.0, 1.0, 0.0),
];
pub const CUBE_COLOR: [Vector3<f32>; 8] = [
    Vector3::new(0.0, 0.0, 0.0),
    Vector3::new(0.0, 0.0, 1.0),
    Vector3::new(1.0, 0.0, 1.0),
    Vector3::new(1.0, 0.0, 0.0),
    Vector3::new(0.0, 1.0, 0.0),
    Vector3::new(0.0, 1.0, 1.0),
    Vector3::new(1.0, 1.0, 1.0),
    Vector3::new(1.0, 1.0, 0.0),
];
pub const CUBE_ELEMENTS: [u32; 36] = [
    0, 1, 2, 0, 2, 3, //BOT
    4, 5, 6, 4, 6, 7, //TOP
    1, 2, 6, 1, 6, 5, //FRONT
    3, 7, 4, 3, 4, 0, //BACK
    0, 4, 5, 0, 5, 1, //LEFT
    2, 3, 7, 2, 7, 6, //RIGHT
];
