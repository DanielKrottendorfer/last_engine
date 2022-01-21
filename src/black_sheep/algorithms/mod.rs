use cgmath::{InnerSpace, Vector2};

pub fn sweep_and_prune(
    positions: &Vec<Vector2<f32>>,
    dirs: &mut Vec<Vector2<f32>>,
    rads: &Vec<f32>,
) {
    let mut xx: Vec<(i32, usize)> = Vec::new();

    for i in 0..positions.len() {
        let p = positions[i];
        let d = dirs[i];
        let r = rads[i];
        xx.push(((p.x + d.x + r) as i32, i));
        xx.push(((p.x + d.x - r) as i32, i));
    }
    xx.sort_by(|a, b| a.0.cmp(&b.0));

    let mut collision_candidates: Vec<usize> = Vec::new();

    for x_i in xx.drain(0..) {
        if let Some(last) = collision_candidates.last() {
            if x_i.1 == *last {
                collision_candidates.pop();
            } else {
                collision_candidates.push(x_i.1);
            }
        } else {
            collision_candidates.push(x_i.1);
        }
    }

    collision_candidates.sort();
    collision_candidates.dedup();

    let mut yy = Vec::new();

    for i in collision_candidates.drain(0..) {
        yy.push((positions[i].y + rads[i], i));
        yy.push((positions[i].y - rads[i], i));
    }
    yy.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    for y_i in yy.drain(0..) {
        if let Some(last) = collision_candidates.last() {
            if y_i.1 == *last {
                collision_candidates.pop();
            } else {
                collision_candidates.push(y_i.1);
            }
        } else {
            collision_candidates.push(y_i.1);
        }
    }

    collision_candidates.sort();
    collision_candidates.dedup();

    let mut colliders = Vec::new();
    for i in collision_candidates {
        let d = dirs.get(i).unwrap();
        let p = positions[i] + d.clone();
        let r = rads[i];

        for y in i + 1..positions.len() {
            let pi = &positions[y] + &dirs[y];
            let ri = rads[y];

            if (p - pi).magnitude() < ri + r {
                colliders.push((i, y));
            }
        }
    }
    for (i, y) in colliders.drain(0..) {
        let pi = positions[i] + dirs[i];
        let py = positions[y] + dirs[y];
        let ri = rads[i];
        let ry = rads[y];

        let v = (pi - py).normalize();

        if (py - pi).magnitude() < ri + ry {
            let di = &mut dirs[i];
            let ki = di.dot(v);
            *di = *di - (2.0 * ki * v);

            let dy = &mut dirs[y];
            let ky = dy.dot(v);
            *dy = *dy - (2.0 * ky * v);
        }
    }
}

pub fn brute_force_collison(
    positions: &Vec<Vector2<f32>>,
    dirs: &mut Vec<Vector2<f32>>,
    rads: &Vec<f32>,
) {
    let mut colliders = Vec::new();
    for i in 0..positions.len() {
        let p = positions[i] + dirs[i];
        let r = rads[i];

        for y in i + 1..positions.len() {
            let pi = &positions[y] + dirs[y];
            let ri = rads[y];

            if (p - pi).magnitude() < ri + r {
                colliders.push((i, y));
            }
        }
    }
    for (i, y) in colliders.drain(0..) {
        let pi = positions[i] + dirs[i];
        let py = positions[y] + dirs[y];
        let ri = rads[i];
        let ry = rads[y];

        let v = (pi - py).normalize();

        if (py - pi).magnitude() < ri + ry {
            let di = &mut dirs[i];
            let ki = di.dot(v);
            *di = *di - (2.0 * ki * v);

            let dy = &mut dirs[y];
            let ky = dy.dot(v);
            *dy = *dy - (2.0 * ky * v);
        }
    }
}

pub fn prevent_out_of_bound(
    positions: &Vec<Vector2<f32>>,
    dirs: &mut Vec<Vector2<f32>>,
    rads: &Vec<f32>,
    window_size_f32: &[f32; 2],
) {
    positions
        .iter()
        .zip(rads.iter().zip(dirs.iter_mut()))
        .for_each(|(p, (r, d))| {
            let tp = *p + *d;

            if tp.x - r < 0.0 || tp.x + r > window_size_f32[0] - 300.0 {
                d.x = -d.x;
            }
            if tp.y - r < 0.0 || tp.y + r > window_size_f32[1] {
                d.y = -d.y;
            }
        });
}
