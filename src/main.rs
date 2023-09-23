#![feature(trait_alias)]

use crate::black_sheep::{
    gamestate::camera::structs::FlyingEye,
    rendering::geometry::{mesh::MeshToken},
    settings::DT,
};
use black_sheep::{DrawFunction, UpdateFunction};
use cgmath::{InnerSpace, Matrix4, Vector2, Vector3, Zero, vec3};

mod black_sheep;
mod gameplay;

extern crate cgmath;
extern crate gl;
extern crate rand;
extern crate sdl2;

#[macro_use]
extern crate lazy_static;

fn main() {
    #[cfg(not(feature = "debug_off"))]
    println!("Hello, world!");

    black_sheep::run(|ecs| {
        gameplay::gen_apes(ecs);

        ecs.add_ball_soa(Vector2::new(5.0, -5.0), Vector2::zero());

        let mut circle = ecs.get_circle_accessor();
        let positions = ecs.get_positions_accessor();
        let mut pos_update = ecs.get_update_pos_ori_accessor();

        let mut simulate = ecs.get_simulate_accessor();

        let g = Vector2::new(0.0, -10.0);

        let r = 2.0;

        let update = move |_input| {
            {
                let mut update = pos_update.lock();
                for (pos, ori, direction, target_ori) in update.iter() {
                    *pos = *pos + *direction;
                    *ori = *target_ori;
                }
            }
            gameplay::run_ape_ai(&mut circle, &positions);

            let mut simulate = simulate.lock();

            for (pos, v) in simulate.iter() {
                *v += g * DT;
                let p = *pos;
                *pos += *v * DT;
                *pos = pos.normalize() * r;
                *v = (*pos - p) / DT;
            }
        };

        let draw_m = ecs.get_draw_accessor();

        let mut calc_mat = ecs.get_calculate_mat_accessor();

        //let mut c_vec = Vec::new();
        //let mut simulate = ecs.get_simulate_accessor();

        let _bb = black_sheep::setup::init_mesh().unwrap();

        let (ape, torus) = black_sheep::rendering::geometry::get_mesh_repo(|mr| {
            let ape = MeshToken::from(mr.get_mesh_by_name("ape").unwrap());
            let torus = MeshToken::from(mr.get_mesh_by_name("torus").unwrap());
            //let circles = MeshToken::from(mr.get_mesh_by_name("circles").unwrap());
            (ape, torus)
        });

        let rendering = black_sheep::rendering::shader::get_shader_repo();

        let three_d = rendering.color_3d;
        let three_dl = rendering.color_3d_light;
        let _circles_2d = rendering.point_2d;

        let draw = move |i: f32, cam: &FlyingEye, prj: &Matrix4<f32>| {
            let view = cam.get_i_view(i);

            for (p, o, direction, to, model) in calc_mat.lock().iter() {
                let q = o.slerp(*to, i);
                let v = p + (direction * i);
                //println!("{:?}",v);
                let mut m = Matrix4::from(q);
                m.w = v.extend(1.0);
                *model = m;
            }

            let d_lock = draw_m.lock();

            ape.bind_vertex_array();
            three_dl.use_program();
            three_dl.set_light_position(vec3(30.0, 30.0, 10.0));
            three_dl.set_light_power(1000.0);

            for (m, c) in d_lock.iter() {
                three_dl.set_MVP(prj * view * m);
                three_dl.set_M(*m);
                three_dl.set_col(*c);
                ape.draw_triangle_elements();
            }

            three_d.use_program();
            three_d.set_MVP(prj * view);
            three_d.set_col(Vector3::new(1.0, 0.0, 1.0));

            torus.bind_vertex_array();
            torus.draw_line_elements();

            // for c in simulate.lock().iter() {
            //     c_vec.push(*c.0 + (*c.1 * DT * i));
            // }
            // geometry::get_mesh_repo(|mr| {
            //     mr.get_mesh_by_uid(&circles.uid)
            //         .unwrap()
            //         .update_buffer(c_vec.as_slice(), 0);
            // });
            // c_vec.clear();

            // circles_2d.use_program();
            // let ortho = cgmath::ortho(-8.0, 8.0, -8.0, 8.0, -1.0, 1.0);
            // circles_2d.set_projection(ortho);

            // unsafe {
            //     gl::Disable(gl::DEPTH_TEST);
            // }

            // circles.bind_vertex_array();
            // circles.draw_point_elements();
        };
        black_sheep::Logic { update, draw }
    });
}
