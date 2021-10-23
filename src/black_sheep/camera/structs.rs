use cgmath::{BaseFloat, Matrix4, Quaternion, Rad, Rotation3, Vector3};

#[derive(Debug)]
pub struct FlyingEye {
    pub position: Vector3<f32>,
    pub movement: Option<Vector3<f32>>,
    pub orientation: Quaternion<f32>,
    pub target_orientation: Option<Quaternion<f32>>,
}

impl FlyingEye {
    pub fn new() -> Self {
        FlyingEye {
            position: Vector3::new(0.0, 0.0, 0.0),
            movement: None,
            orientation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            target_orientation: None,
        }
    }
    pub fn move_cam(&mut self, v: Vector3<f32>) {
        self.position -= (Matrix4::from(self.orientation.conjugate()) * v.extend(1.0)).truncate();
    }
    pub fn rotate_h<A>(&mut self, theta: A) where A: Into<Rad<f32>> {
        self.orientation =  Quaternion::from_angle_x(theta) * self.orientation;
    }
    pub fn rotate_v<A>(&mut self, theta: A) where A: Into<Rad<f32>> {
        self.orientation =  self.orientation * Quaternion::from_angle_y(theta);
    }
    pub fn get_i_view(&self, i: f32) -> Matrix4<f32> {
        let t = if let Some(d) = self.movement {
            Matrix4::from_translation(self.position + (d * i))
        } else {
            Matrix4::from_translation(self.position)
        };
        let r = if let Some(r) = self.target_orientation {
            Matrix4::from(self.orientation.slerp(r, i))
        } else {
            Matrix4::from(self.orientation)
        };
        r * t
    }
}

impl Default for FlyingEye {
    fn default() -> Self {
        Self::new()
    }
}
