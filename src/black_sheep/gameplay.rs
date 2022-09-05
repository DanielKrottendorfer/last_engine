use cgmath::*;

use super::{
    gamestate::ecs::{CircleAccessor, PositionsAccessor},
    torus::torus_r,
};

pub fn run_ape_ai(circle: &mut CircleAccessor, positions: &PositionsAccessor) {
    let mut c_l = circle.lock();
    let pos_s = positions.lock();

    let speed = 0.5;
    for (pos, ori, direction, target_ori, col, key) in c_l.iter() {
        let r_x = Quaternion::from_angle_x(Deg(20.0));
        let r_y = Quaternion::from_angle_y(Deg(20.0));

        let q1 = *ori * r_x;
        let q2 = *ori * r_x.invert();
        let q3 = *ori * r_y;
        let q4 = *ori * r_y.invert();

        let v1 = *pos + q1.rotate_vector(Vector3::unit_z() * speed);
        let v2 = *pos + q2.rotate_vector(Vector3::unit_z() * speed);
        let v3 = *pos + q3.rotate_vector(Vector3::unit_z() * speed);
        let v4 = *pos + q4.rotate_vector(Vector3::unit_z() * speed);

        let id = if torus_r(*pos, 20.0) > 4.0 {
            let t1 = torus_r(v1, 20.0);
            let t2 = torus_r(v2, 20.0);
            let t3 = torus_r(v3, 20.0);
            let t4 = torus_r(v4, 20.0);

            let mut min = 0;
            let ts = [t1, t2, t3, t4];
            for i in 1..ts.len() {
                if ts[i] < ts[min] {
                    min = i;
                }
            }

            *col = Vector3::unit_x();
            min
        } else {
            let mut min_dist = f32::MAX;
            let mut min_key = key.clone();
            for (p, k) in pos_s.iter() {
                if key == k {
                    continue;
                }
                let dist = (pos - p).magnitude();
                if dist < min_dist {
                    min_dist = dist;
                    min_key = k;
                }
            }

            let p = pos_s.get(min_key).unwrap();

            let t1 = (v1 - *p).magnitude();
            let t2 = (v2 - *p).magnitude();
            let t3 = (v3 - *p).magnitude();
            let t4 = (v4 - *p).magnitude();

            let id = if min_dist < 5.0 {
                let mut max = 0;
                let ts = [t1, t2, t3, t4];
                for i in 1..ts.len() {
                    if ts[i] > ts[max] {
                        max = i;
                    }
                }

                *col = Vector3::unit_y();
                max
            } else if min_dist > 10.0 {
                let mut min = 0;
                let ts = [t1, t2, t3, t4];
                for i in 1..ts.len() {
                    if ts[i] < ts[min] {
                        min = i;
                    }
                }

                *col = Vector3::unit_z();
                min
            } else {
                continue;
            };
            id
        };

        match id {
            0 => {
                *target_ori = q1;
                *direction = (v1 - *pos) * speed;
            }
            1 => {
                *target_ori = q2;
                *direction = (v2 - *pos) * speed;
            }
            2 => {
                *target_ori = q3;
                *direction = (v3 - *pos) * speed;
            }
            3 => {
                *target_ori = q4;
                *direction = (v4 - *pos) * speed;
            }

            _ => (),
        }
    }
}
