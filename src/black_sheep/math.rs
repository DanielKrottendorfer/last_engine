pub mod tetrahedral {
    use cgmath::*;

    #[derive(Debug, Clone)]
    pub struct Tetrahedral(pub [Vector3<f32>; 4], pub f32);

    impl Tetrahedral {
        pub fn new(r: f32) -> Self {
            let x = 3_f32.sqrt() * r;
            let h = 3_f32.sqrt() * x;

            let v1 = Vector3::new(-x, 0.0, -r);
            let v2 = Vector3::new(x, 0.0, -r);
            let v3 = Vector3::new(0.0, 0.0, 2.0 * r);
            let v4 = Vector3::new(0.0, h, 0.0);
            Tetrahedral([v1, v2, v3, v4], 2.0 * x)
        }
        pub fn zero() -> Self {
            let v1 = Vector3::zero();
            let v2 = Vector3::zero();
            let v3 = Vector3::zero();
            let v4 = Vector3::zero();
            Tetrahedral([v1, v2, v3, v4], 0.0)
        }
        pub fn get_constraints(&self) -> [Vector3<f32>; 4] {
            let v1 = self.0[0];
            let v2 = self.0[1];
            let v3 = self.0[2];
            let v4 = self.0[3];

            let c1 = (v4 - v2).cross(v3 - v2);
            let c2 = (v3 - v1).cross(v4 - v1);
            let c3 = (v4 - v1).cross(v2 - v1);
            let c4 = (v2 - v1).cross(v3 - v1);
            [c1, c2, c3, c4]
        }
        pub fn get_volume(&self) -> f32 {
            let v1 = self.0[0];
            let v2 = self.0[1];
            let v3 = self.0[2];
            let v4 = self.0[3];

            (1.0 / 6.0) * ((v2 - v1).cross(v3 - v1).dot(v4 - v1))
        }
    }

    impl From<Tetrahedral> for [Vector3<f32>; 4] {
        fn from(t: Tetrahedral) -> Self {
            t.0
        }
    }
}
