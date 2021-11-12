use super::{constants::*, point_cloud, rendering::geometry::MeshRepo};



pub fn init_mesh(mesh_repo: &mut MeshRepo){
    
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
}