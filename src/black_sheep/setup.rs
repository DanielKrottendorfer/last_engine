use cgmath::{Vector2, Vector3};
use itertools::Itertools;

use crate::black_sheep::{constants::*, generators::squares::*, generators::*};

use super::rendering::geometry::{self, mesh::MeshToken};

pub fn init_mesh() -> Vec<MeshToken> {
    let vm = geometry::get_mesh_repo(|mesh_repo| {
        let triangle = mesh_repo.add_mesh("triangle", |mesh| {
            mesh.add_floatbuffer(&SIMPLE_TRIANGL, 0, 2);
            mesh.add_elementarraybuffer(&TRIANGLE_ELEMENTS);
        });

        let gizmo = mesh_repo.add_mesh("gizmo", |mesh| {
            mesh.add_floatbuffer(&GIZMO_VECS, 0, 3);
            mesh.add_elementarraybuffer(&GITMO_ELEMENTS);
        });

        let cube = mesh_repo.add_mesh("cube", |mesh| {
            mesh.add_floatbuffer(&CUBE, 0, 3);
            mesh.add_floatbuffer(&CUBE_COLOR, 1, 3);
            mesh.add_elementarraybuffer(&CUBE_ELEMENTS);
        });

        let cube_cloud = mesh_repo.add_mesh("cloud", |mesh| {
            let (v, c, e) = point_cloud::point_cube(5);
            mesh.add_floatbuffer(v.as_slice(), 0, 3);
            mesh.add_floatbuffer(c.as_slice(), 1, 4);
            mesh.add_elementarraybuffer(e.as_slice());
        });

        let colored_triangles = mesh_repo.add_mesh("ctriangles", |mesh| {
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

        let ape = mesh_repo.add_mesh("ape", |m| {
            let (gltf, buffers, _) = gltf::import("res/ape.glb").unwrap();
            let mesh = gltf.meshes().next().unwrap();

            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                if let Some(iter) = reader.read_positions() {
                    let p = iter.collect_vec();
                    m.add_floatbuffer(&p, 0, 3);
                    m.add_floatbuffer(&p, 1, 3);
                }
                if let Some(iter) = reader.read_indices() {
                    let e = iter.into_u32().collect_vec();
                    m.add_elementarraybuffer(&e);
                }
            }
        });
        let torus = mesh_repo.add_mesh("torus", |m| {
            let (gltf, buffers, _) = gltf::import("res/torus.glb").unwrap();
            let mesh = gltf.meshes().next().unwrap();

            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                if let Some(iter) = reader.read_positions() {
                    let p = iter.collect_vec();
                    m.add_floatbuffer(&p, 0, 3);
                    m.add_floatbuffer(&p, 1, 3);
                }
                if let Some(iter) = reader.read_indices() {
                    let e = iter.into_u32().collect_vec();
                    m.add_elementarraybuffer(&e);
                }
            }
        });

        vec![
            triangle,
            gizmo,
            cube,
            cube_cloud,
            colored_triangles,
            ape,
            torus,
        ]
    });

    vm
}
