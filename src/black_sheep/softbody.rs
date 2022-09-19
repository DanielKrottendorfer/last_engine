use cgmath::{num_traits::Float, InnerSpace, Vector3, Zero};

pub struct Softbody {
    pub particles: Vec<Vector3<f32>>,
    pub colors: Vec<Vector3<f32>>,
    pub temp_particles: Vec<Vector3<f32>>,
    pub vel_particles: Vec<Vector3<f32>>,
    pub edges: Vec<u32>,
    a: f32,
    tetras: Vec<[usize; 4]>,
}

impl Softbody {
    pub fn build_tetragrid(a: f32, l: usize) -> Self {
        let mut particles = Vec::new();
        let mut colors = Vec::new();

        let root_32 = 3.0.sqrt() / 2.0;
        let root_23 = (2.0 / 3.0).sqrt();

        let ha = root_32 * a;
        let h = root_23 * a;

        for y in 0..l {
            let z_off = if y % 2 == 0 { ha * 2.0 / 3.0 } else { 0.0 };
            for z in 0..l {
                let x_off = if z % 2 == 0 { a / 2.0 } else { 0.0 };
                for x in 0..l {
                    let vx = (x as f32 * a) + x_off;
                    let vy = y as f32 * h;
                    let vz = (z as f32 * ha) + z_off;
                    let v = Vector3::new(vx, vy, vz);
                    particles.push(v);
                    colors.push(Vector3::unit_x());
                }
            }
        }
        let l = particles.len();
        let mut s = Softbody {
            particles,
            edges: Vec::new(),
            tetras: Vec::new(),
            temp_particles: vec![Vector3::zero(); l],
            vel_particles: vec![Vector3::zero(); l],
            a,
            colors,
        };
        s.build_edges(a);
        s
    }
    pub fn get_particle_slice(&self) -> &[Vector3<f32>] {
        self.particles.as_slice()
    }

    pub fn simulate(&mut self, dt: f32) {
        self.pre_solve(dt);
        self.edge(dt);
        self.harddeck();
        self.post_solve(dt);
    }

    fn pre_solve(&mut self, dt: f32) {
        let g = Vector3::new(0.0, -1.0, 0.0);
        for i in 0..self.particles.len() {
            let x = &mut self.particles[i];
            let p = &mut self.temp_particles[i];
            let v = &mut self.vel_particles[i];
            *v += g * dt;
            *p = *x;
            *x += *v * dt;
        }
    }

    fn post_solve(&mut self, dt: f32) {
        for i in 0..self.particles.len() {
            let x = &mut self.particles[i];
            let p = &mut self.temp_particles[i];
            let v = &mut self.vel_particles[i];
            *v = (*x - *p) / dt;
        }
    }

    fn edge(&mut self, dt: f32) {
        for i in (0..self.edges.len()).step_by(2) {
            let e1 = self.edges[i] as usize;
            let e2 = self.edges[i + 1] as usize;

            let mut v1 = self.particles.get(e1).unwrap().clone();
            let mut v2 = self.particles.get(e2).unwrap().clone();

            let w = 2.0;

            let g = v1 - v2;
            let len = g.magnitude();
            let g = g / len;

            let c = len - self.a;

            let lamb = -c / (w + (0.01 / dt.powf(2.0)));

            v1 += g * lamb;
            v2 -= g * lamb;

            *self.particles.get_mut(e1).unwrap() = v1;
            *self.particles.get_mut(e2).unwrap() = v2;
        }
    }

    fn harddeck(&mut self) {
        for v in self.particles.iter_mut() {
            if v.y < 0.0 {
                v.y = 0.0;
            }
        }
    }

    fn build_edges(&mut self, a: f32) {
        self.edges.clear();

        let mut i = 0;

        let mut _rest = self.particles.as_slice();
        while let Some((v, rest)) = _rest.split_first() {
            let mut y = i + 1;

            for u in rest.iter() {
                let mag = (v - u).magnitude();
                if mag < a + 0.01 {
                    self.edges.push(i);
                    self.edges.push(y);
                };
                y += 1;
            }

            i += 1;
            _rest = rest;
        }
    }
}
