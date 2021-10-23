use cgmath::{Quaternion, Vector3};

#[derive(Debug)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub orientation: Quaternion<f32>,
    pub scale: f32,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            position: Vector3::new(0.0, 0.0, 0.0),
            orientation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            scale: 1.0,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}
