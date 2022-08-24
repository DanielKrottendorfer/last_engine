pub mod imgui_mesh;
mod mesh_util;
mod unique_index;
pub mod mesh;


use std::{borrow::Borrow, collections::HashMap, sync::Mutex};

use mesh_util::*;
use unique_index::*;

use mesh::Mesh;

use self::mesh::MeshToken;


lazy_static! {
    static ref MESH_REPO: Mutex<Option<MeshRepo>> = Mutex::new(None);
}

pub fn init() {
    let sr = MESH_REPO.lock();
    if sr.is_err() {
        panic!("shader_repo locked failed");
    }

    let mut sr = sr.unwrap();

    if sr.is_some() {
        panic!("shader_repo already initialized")
    }

    *sr = Some(MeshRepo::new());
}
pub fn cleanup() {
    let sr = MESH_REPO.lock();
    if sr.is_err() {
        panic!("shader_repo locked failed");
    }

    let sr = sr.unwrap();

    if let Some(sr) = &*sr {
        for mesh in sr.mesh_i_data.iter() {
            mesh.cleanup();
        }
    }else{
        panic!("shader_repo not initialized")
    }

}

pub fn get_mesh_repo<T: FnMut(&mut MeshRepo) -> S, S>(mut f: T) -> S {
    let sr = MESH_REPO.lock();
    if sr.is_err() {
        panic!("shader_repo locked failed");
    }

    let mut sr = sr.unwrap();
    if sr.is_none() {
        panic!("shader_repo already initialized");
    }

    if let Some(mr) = sr.as_mut() {
        f(mr)
    } else {
        panic!("something went horribly wrong");
    }
}

pub struct MeshRepo {
    unique_indexer: UniqueIndexer,
    mesh_i_data: Vec<Mesh>,
    mesh_map: HashMap<String, usize>,
}

impl MeshRepo {
    fn new() -> Self {
        MeshRepo {
            unique_indexer: UniqueIndexer::new(),
            mesh_i_data: Vec::new(),
            mesh_map: HashMap::new(),
        }
    }

    pub fn remove_mesh(&mut self, name: &str) {
        if let Some(uid) = self.mesh_map.remove(name) {
            if let Some(i) = self
                .mesh_i_data
                .binary_search_by_key(&uid, |x| x.uid)
                .ok()
                .map(|i| i)
            {
                self.mesh_i_data.remove(i);
            }
        }
    }

    pub fn add_mesh<T: Fn(&mut Mesh)>(&mut self, name: &str, init_mesh: T) -> MeshToken {
        let unique_index = self.unique_indexer.get_next();

        let mut mesh = Mesh::new();
        mesh.uid = unique_index;

        init_mesh(&mut mesh);

        let mesh_token = MeshToken::from(mesh.borrow());

        self.mesh_i_data.push(mesh);

        if self
            .mesh_map
            .insert(String::from(name), unique_index)
            .is_some()
        {
            panic!("name already taken!");
        }

        mesh_token
    }

    pub fn get_mesh_by_uid(&self, uid: &usize) -> Option<&Mesh> {
        self.mesh_i_data
            .binary_search_by_key(uid, |x| x.uid)
            .ok()
            .map(|i| &self.mesh_i_data[i])
    }

    pub fn get_mesh_by_name(&self, name: &str) -> Option<&Mesh> {
        let uid = self.mesh_map.get(name)?;
        self.get_mesh_by_uid(uid)
    }
}
