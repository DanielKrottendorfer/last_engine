use cgmath::{Matrix4, Quaternion, Rad, Rotation3, Vector3};

use crate::black_sheep::settings::UPS_F32;

#[derive(Debug)]
pub struct FlyingEye {
    pub position: Vector3<f32>,
    pub movement: Option<Vector3<f32>>,
    pub orientation: Quaternion<f32>,
}

impl FlyingEye {
    pub fn new() -> Self {
        FlyingEye {
            position: Vector3::new(0.0, 0.0, 0.0),
            movement: None,
            orientation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
        }
    }
    pub fn move_cam(&mut self, v: Vector3<f32>) {
        self.position -= (Matrix4::from(self.orientation.conjugate()) * v.extend(1.0)).truncate();
    }
    pub fn set_movement(&mut self, v: Vector3<f32>) {
        self.movement =
            Some((Matrix4::from(self.orientation.conjugate()) * (-v).extend(1.0)).truncate());
    }
    pub fn reset_movement(&mut self) {
        self.movement = None;
    }
    pub fn rotate_h<A>(&mut self, theta: A)
    where
        A: Into<Rad<f32>>,
    {
        self.orientation = Quaternion::from_angle_x(theta) * self.orientation;
    }
    pub fn rotate_v<A>(&mut self, theta: A)
    where
        A: Into<Rad<f32>>,
    {
        self.orientation = self.orientation * Quaternion::from_angle_y(theta);
    }
    pub fn update(&mut self) {
        if let Some(v) = self.movement {
            self.position += v / UPS_F32;
        }
    }
    pub fn get_i_view(&self, i: f32) -> Matrix4<f32> {
        let t = if let Some(v) = self.movement {
            Matrix4::from_translation(self.position + ((v / UPS_F32) * i))
        } else {
            Matrix4::from_translation(self.position)
        };
        let r = Matrix4::from(self.orientation);

        r * t
    }
}

impl Default for FlyingEye {
    fn default() -> Self {
        Self::new()
    }
}
