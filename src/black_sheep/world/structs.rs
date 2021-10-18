use cgmath::{Matrix4, Quaternion, Vector3};

#[derive(Debug)]
pub struct Camera{
    pub position: Vector3<f32>,
    pub direction: Option<Vector3<f32>>,
    pub orientation: Quaternion<f32>,
    pub scale: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera{
            position: Vector3::new(0.0,0.0,0.0),
            direction: None,
            orientation: Quaternion::new(1.0,0.0,0.0,0.0),
            scale: 1.0,
        }
    }
    pub fn get_i_view(&self, i: f32) -> Matrix4<f32> {
        let t = if let Some(d) = self.direction {
            Matrix4::from_translation(self.position + (d * i))
        } else {
            Matrix4::from_translation(self.position)
        };
        let r = Matrix4::from(self.orientation);
        let s = Matrix4::from_scale(self.scale);

        t*r*s
    }
}

#[derive(Debug)]
pub struct Transform{
    pub position: Vector3<f32>,
    pub orientation: Quaternion<f32>
}

impl Transform {
    pub fn new() -> Self{
        Transform{
            position: Vector3::new(0.0,0.0,0.0),
            orientation: Quaternion::new(1.0,0.0,0.0,0.0)
        }
    }
}

impl Default for Transform{
    fn default() -> Self {
        Self::new()
    }
}
