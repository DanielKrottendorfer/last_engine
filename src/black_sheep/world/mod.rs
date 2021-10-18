use std::iter::repeat;

use self::structs::Transform;


mod structs;
use cgmath::{Matrix4, Point3, SquareMatrix, Vector3};
use structs::*;

use super::rendering::geometry::MeshToken;

#[derive(Default,Debug)]
pub struct World{
    cam: Transform,
    meshes: Vec<(Transform,MeshToken)>,
}


impl World {
    pub fn new() -> Self{
        World::default()
    }

    pub fn move_cam(&mut self, v: Vector3<f32>){
        self.cam.position -= v;
    }

    pub fn get_view(&self) -> Matrix4<f32> {
        let translate = Matrix4::from_translation(self.cam.position);
        let rotation = Matrix4::from(self.cam.orientation);

        translate * rotation
    }
}