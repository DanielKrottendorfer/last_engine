use std::borrow::BorrowMut;

use cgmath::*;

use crate::black_sheep::rendering::geometry;

use super::{
    gamestate::ecs::{CircleAccessor, PositionsAccessor, SimulateAccessor, CHAINED_ECS},
    math::tetrahedral::Tetrahedral,
    settings::DT,
    torus::torus_r,
};

pub fn run_pendulum(circels: &mut SimulateAccessor) {
    let mut simulate = circels.lock();

    let steps = 50;
    let s = steps as f32;
    let dt_var = DT / s;

    let g = Vector2::new(0.0, -10.0);
    let r = 3.0;

    for _ in 0..steps {
        let mut it = simulate.iter();
        while let Some((x, p, v)) = it.next() {
            *v += g * dt_var;
            *p = *x;
            *x += *v * dt_var;
        }

        let mut it = simulate.iter();
        if let Some(mut v1) = it.next() {
            *v1.0 = v1.0.normalize() * r;
            while let Some(v2) = it.next() {
                let cs = *v2.0 - *v1.0;
                let c = cs.normalize() * 2.0;
                let k = (cs - c) / 2.0;

                *v1.0 += k;
                *v2.0 -= k;

                v1 = v2;
            }
        }

        let mut it = simulate.iter();
        while let Some((pos, pp, v)) = it.next() {
            *v = (*pos - *pp) / dt_var;
        }
    }
}

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

pub fn gen_apes(ecs: &mut CHAINED_ECS) {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();

    let mut center = Vec::new();
    let mut ups = Vec::new();

    for i in 0..3 {
        for y in 0..3 {
            let c = [i as f32, 0.0, y as f32].into();
            let u: Vector3<f32> = Vector3::unit_y();

            ecs.add_ape_soa(
                c,
                Quaternion::from_angle_x(Rad(0.0)),
                [i as f32, rng.gen_range(-0.5..0.5), y as f32].into(),
                Quaternion::from_angle_x(Rad(0.0)),
                Vector3::new(1.0, 1.0, 1.0),
                cgmath::SquareMatrix::identity(),
            );
            center.push(c);
            ups.push(u);
        }
    }
}

pub fn harddeck(t: &mut Tetrahedral) {
    for v in t.0.iter_mut() {
        if v.y < 0.0 {
            v.y = 0.0;
        }
    }
}

pub fn vol_c(t: &mut Tetrahedral) {
    let c = t.get_constraints();
    let lamb = (-6.0 * (t.get_volume() - t.1))
        / (c.iter().map(|x| x.magnitude2()).sum::<f32>() + (1.0 / DT.powf(2.0)));

    for i in 0..4 {
        t.0[i] += lamb * c[i];
    }
}

pub fn tetra_dist(t: &mut Tetrahedral) {
    let dist = t.2;

    let mut rest = t.0.as_mut_slice();
    while let Some((v1, rest_)) = rest.split_first_mut() {
        for v2 in rest_.borrow_mut() {
            let w = 2.0;

            let g = *v1 - *v2;
            let len = g.magnitude();
            let g = g / len;

            let c = len - dist;

            let lamb = -c / (w + (1.0 / DT.powf(2.0)));

            *v1 += g * lamb;
            *v2 -= g * lamb;
        }
        rest = rest_;
    }
}
