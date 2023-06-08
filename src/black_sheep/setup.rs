use cgmath::{Vector2, Vector3, Zero};
use itertools::Itertools;
use rand::Rng;

use crate::black_sheep::{constants::*, generators::squares::*, generators::*};

use super::{
    generators,
    math::tetrahedral::Tetrahedral,
    rendering::geometry::{self},
};

pub fn init_mesh() -> Option<(Vector3<f32>, Vector3<f32>)> {
    let mut bb = None;
    let _vm = geometry::get_mesh_repo(|mesh_repo| {
        let _triangle = mesh_repo.add_mesh("triangle", |mesh| {
            mesh.add_floatbuffer(&SIMPLE_TRIANGL, 0, 2);
            mesh.add_elementarraybuffer(&TRIANGLE_ELEMENTS);
        });

        let _gizmo = mesh_repo.add_mesh("gizmo", |mesh| {
            mesh.add_floatbuffer(&GIZMO_VECS, 0, 3);
            mesh.add_elementarraybuffer(&GITMO_ELEMENTS);
        });

        let _cube = mesh_repo.add_mesh("cube", |mesh| {
            mesh.add_floatbuffer(&CUBE, 0, 3);
            mesh.add_floatbuffer(&CUBE_COLOR, 1, 3);
            mesh.add_elementarraybuffer(&CUBE_ELEMENTS);
        });

        let _cube_cloud = mesh_repo.add_mesh("cloud", |mesh| {
            let (v, c, e) = point_cloud::point_cube(5);
            mesh.add_floatbuffer(v.as_slice(), 0, 3);
            mesh.add_floatbuffer(c.as_slice(), 1, 4);
            mesh.add_elementarraybuffer(e.as_slice());
        });

        let _colored_triangles = mesh_repo.add_mesh("ctriangles", |mesh| {
            let mut ss = squares::SquareComposition::new();
            ss.add_square(Square::new(
                Vector2::new(100.0, 100.0),
                Vector2::new(500.0, 500.0),
                Vector3::new(1.0, 0.0, 0.0),
            ));
            let vc = ss.generate_colored_triangles();
            mesh.add_floatbuffer(&vc.0, 0, 2);
            mesh.add_floatbuffer(&vc.1, 1, 3);
            mesh.add_elementarraybuffer(&vc.2);
        });

        let _ape = mesh_repo.add_mesh("ape", |m| {
            let (gltf, buffers, _) = gltf::import("res/ape.glb").unwrap();
            let mesh = gltf.meshes().next().unwrap();

            for primitive in mesh.primitives() {
                let _b = primitive.bounding_box();
                bb = Some((Vector3::zero(), Vector3::zero()));
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                if let Some(iter) = reader.read_positions() {
                    let p = iter.collect_vec();
                    m.add_floatbuffer(p.as_slice(), 0, 3);
                }
                if let Some(iter) = reader.read_indices() {
                    let e = iter.into_u32().collect_vec();
                    m.add_elementarraybuffer(e.as_slice());
                }
            }
        });
        let _torus = mesh_repo.add_mesh("torus", |m| {
            let (v, e) = generators::point_circle::circel(20, 20.0);
            m.add_floatbuffer(v.as_slice(), 0, 3);
            m.add_elementarraybuffer(e.as_slice());
        });
        let _circles = mesh_repo.add_mesh("circles", |m| {
            let mut positions = Vec::new();
            let mut rads = Vec::new();
            let mut colors = Vec::new();
            let mut elements = Vec::new();

            let mut rng = rand::thread_rng();

            let mut e = 0;
            for _ in 0..4 {
                positions.push(Vector2::new(0.0, 0.0 as f32));
                rads.push(0.5 as f32);
                colors.push(Vector3::new(
                    rng.gen_range(0.0..1.0 as f32),
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                ));
                elements.push(e);
                e += 1;
            }

            m.add_dynamic_floatbuffer(positions.as_slice(), 0, 2);
            m.add_floatbuffer(colors.as_slice(), 1, 3);
            m.add_floatbuffer(rads.as_slice(), 2, 1);
            m.add_elementarraybuffer(elements.as_slice());
        });
        let _tetra = mesh_repo.add_mesh("tetra", |m| {
            let tet = Tetrahedral::new(4.0);

            let elements = vec![0, 1, 0, 2, 0, 3, 1, 2, 1, 3, 2, 3];

            m.add_dynamic_floatbuffer(&tet.0, 0, 3);
            m.add_elementarraybuffer(elements.as_slice());
        });
    });
    bb
}
