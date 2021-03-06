pub mod imgui_mesh;
mod mesh_util;
mod unique_index;

use std::{borrow::Borrow, collections::HashMap, sync::Mutex};

use mesh_util::*;
use unique_index::*;

pub struct Mesh {
    pub uid: usize,
    buffer_ids: Vec<u32>,
    array_id: u32,
    pub vertex_count: i32,
}

impl Mesh {
    fn new() -> Self {
        Mesh {
            uid: 0,
            buffer_ids: Vec::new(),
            array_id: gen_vertexarray(),
            vertex_count: -1,
        }
    }

    pub fn add_floatbuffer<T>(&mut self, data: &[T], attribute_index: u32, attribute_size: i32) {
        if !(attribute_size > 0) {
            panic!("Attribute size needs to be > 0")
        }

        let buffer_id = buffer_data_static(data, gl::ARRAY_BUFFER);
        set_attribute_pointer(attribute_index, gl::FLOAT, attribute_size);

        self.buffer_ids.push(buffer_id);
    }

    pub fn downgrade(&self) -> MeshToken {
        MeshToken::from(self)
    }

    pub fn add_dynamic_floatbuffer<T>(
        &mut self,
        data: &[T],
        attribute_index: u32,
        attribute_size: i32,
    ) {
        if !(attribute_size > 0) {
            panic!("Attribute size needs to be > 0")
        }

        let buffer_id = buffer_data_dynamic(data, gl::ARRAY_BUFFER);
        set_attribute_pointer(attribute_index, gl::FLOAT, attribute_size);

        self.buffer_ids.push(buffer_id);
    }

    pub fn add_intbuffer<T>(&mut self, data: &[T], attribute_index: u32, attribute_size: i32) {
        if !(attribute_size > 0) {
            panic!("Attribute size needs to be > 0")
        }

        let buffer_id = buffer_data_static(data, gl::ARRAY_BUFFER);
        set_attribute_pointer(attribute_index, gl::INT, attribute_size);

        self.buffer_ids.push(buffer_id);
    }

    pub fn add_dynamic_intbuffer<T>(
        &mut self,
        data: &[T],
        attribute_index: u32,
        attribute_size: i32,
    ) {
        if !(attribute_size > 0) {
            panic!("Attribute size needs to be > 0")
        }

        let buffer_id = buffer_data_dynamic(data, gl::ARRAY_BUFFER);
        set_attribute_pointer(attribute_index, gl::INT, attribute_size);

        self.buffer_ids.push(buffer_id);
    }

    pub fn update_buffer<T>(&self, data: &[T], i: usize) {
        update_buffer_data(data, self.buffer_ids[i], gl::ARRAY_BUFFER);
    }

    pub fn update_elementarraybuffer(&self, elements: &[u32]) {
        update_buffer_data(
            elements,
            *self.buffer_ids.last().unwrap(),
            gl::ELEMENT_ARRAY_BUFFER,
        );
    }

    pub fn add_elementarraybuffer(&mut self, elements: &[u32]) {
        let id = buffer_data_static(elements, gl::ELEMENT_ARRAY_BUFFER);
        self.vertex_count = elements.len() as i32;
        self.buffer_ids.push(id);
    }
    pub fn bind_vertex_array(&self) {
        mesh_util::bind_vertex_array(self.array_id);
    }
    pub fn draw_triangle_elements(&self) {
        mesh_util::draw_triangle_elements(self.vertex_count);
    }
    pub fn draw_point_elements(&self) {
        mesh_util::draw_point_elements(self.vertex_count);
    }

    fn cleanup(&self) {
        unsafe {
            #[cfg(not(feature = "debug_off"))]
            println!("mesh cleanup {}", self.array_id);
            for id in self.buffer_ids.iter() {
                gl::DeleteBuffers(1, id);
            }
            gl::DeleteVertexArrays(1, &self.array_id);
        }
    }
}

impl From<&Mesh> for MeshToken {
    fn from(mesh: &Mesh) -> Self {
        Self {
            uid: mesh.uid,
            array_id: mesh.array_id,
            vertex_count: mesh.vertex_count,
        }
    }
}
#[derive(Debug, Default)]
pub struct MeshToken {
    pub uid: usize,
    array_id: u32,
    vertex_count: i32,
}

impl MeshToken {
    pub fn bind_vertex_array(&self) {
        mesh_util::bind_vertex_array(self.array_id);
    }
    pub fn draw_triangle_elements(&self) {
        mesh_util::draw_triangle_elements(self.vertex_count);
    }
    pub fn draw_point_elements(&self) {
        mesh_util::draw_point_elements(self.vertex_count);
    }
}

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
    if let Ok(mut mr) = MESH_REPO.lock() {
        if let Some(mr) = &mut *mr {
            mr.cleanup();
        }
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
                self.mesh_i_data[i].cleanup();
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

    pub fn cleanup(&mut self) {
        for mesh in self.mesh_i_data.iter() {
            mesh.cleanup();
        }
    }
}
