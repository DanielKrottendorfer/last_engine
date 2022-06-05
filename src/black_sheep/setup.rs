use crate::black_sheep::{constants::*, generators::*};

use super::rendering::geometry::{self, *};

pub fn init_mesh() -> Vec<MeshToken> {
    let vm = geometry::get_mesh_repo(|mesh_repo| {
        let gizmo = mesh_repo.add_mesh("gizmo", |mesh| {
            mesh.add_floatbuffer(&GIZMO_VECS, 0, 3);
            mesh.add_elementarraybuffer(&GITMO_ELEMENTS);
        });

        let cube = mesh_repo.add_mesh("cube", |mesh| {
            mesh.add_floatbuffer(&CUBE, 0, 3);
            mesh.add_floatbuffer(&CUBE_COLOR, 1, 3);
            mesh.add_elementarraybuffer(&CUBE_ELEMENTS);
        });

        let voxel_cloud = mesh_repo.add_mesh("voxel", |mesh| {
            let (voxels, e) = voxels::voxel_grid(202, 100, 202, 0.005);
            mesh.add_floatbuffer(voxels.as_slice(), 0, 3);
            mesh.add_elementarraybuffer(e.as_slice());
        });

        vec![gizmo, cube, voxel_cloud]
    });

    vm
}
