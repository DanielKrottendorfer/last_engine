use cgmath::*;
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

        pp: Vector2<f32>,
        p: Vector2<f32>,
        v: Vector2<f32>,

        mat: Matrix4<f32>,
    };

    entities{
        Ape(pos,ori,direction,target_ori,col,mat),
        Ball(p,pp,v)
    };

    global_systems{
        UpdatePosOri(mut pos,mut ori,direction,target_ori),
        Circle(pos,ori,mut direction,mut target_ori, mut col,KEY),
        Positions(pos,KEY),
        CalculateMat(pos,ori,direction,target_ori,mut mat),
        Draw(mat,col),
        Simulate(mut p,mut pp,mut v),
        Poss(p),
    };
);
