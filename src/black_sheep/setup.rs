use cgmath::{Vector2, Vector3, InnerSpace};

use crate::black_sheep::{constants::*, generators::squares::*, generators::*};

use super::rendering::geometry::{self, *};

pub fn get_cube_normals() -> Vec<Vector3<f32>> {

    let center = Vector3::new(0.5,0.5,0.5);

    CUBE.iter().map(|c| {
        (c - center).normalize()
    }).collect()
}

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
            mesh.add_floatbuffer(&get_cube_normals(), 2, 3);
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

        vec![triangle, gizmo, cube, cube_cloud, colored_triangles]
    });

    vm
}
