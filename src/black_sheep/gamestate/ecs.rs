use cgmath::{Matrix4, Quaternion, Vector3};
use chained_component_system::chained_component_system;

use std::sync::*;

chained_component_system!(
    components{
        pos: Vector3<f32>,
        direction: Vector3<f32>,
        speed: f32,
        ori: Quaternion<f32>,
        target_ori: Quaternion<f32>,
        col: Vector3<f32>,

        mat: Matrix4<f32>,
    };

    entities{
        Ape(pos,ori,direction,target_ori,col,mat),
    };

    global_systems{
        update_pos_ori(mut pos,mut ori,direction,target_ori),
        circle(pos,ori,mut direction,mut target_ori, mut col,KEY),
        positions(pos,KEY),
        calculate_mat(pos,ori,direction,target_ori,mut mat),
        draw(mat,col)
    };
);
