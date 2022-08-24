use cgmath::{Vector3, Quaternion, Matrix4};
use chained_component_system::chained_component_system;

use std::sync::*;



chained_component_system!(
    components{
        pos: Vector3<f32>,
        ori: Quaternion<f32>,

        mat: Matrix4<f32>,
        col: Vector3<f32>,
    };

    entities{
        Bird(pos,ori,mat,col),
    };

    global_systems{
        calculate_mat(pos,ori,mut mat),
        draw(mat,col),
    };
);