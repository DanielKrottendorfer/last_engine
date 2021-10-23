mod mesh_util;
mod unique_index;

use mesh_util::*;
use unique_index::*;

pub struct Mesh {
    buffer_ids: Vec<u32>,
    array_id: u32,
    vertex_count: i32,
}

impl Mesh {
    fn new() -> Self {
        Mesh {
            buffer_ids: Vec::new(),
            array_id: gen_vertexarray(),
            vertex_count: -1,
        }
    }

    pub fn add_floatbuffer<T>(&mut self, data: &[T], attribute_index: u32, attribute_size: i32) {
        if !(attribute_size > 0) {
            panic!("Attribute size needs to be > 0")
        }

        let buffer_id = buffer_data(data, gl::ARRAY_BUFFER);
        set_attribute_pointer(attribute_index, gl::FLOAT, attribute_size);

        self.buffer_ids.push(buffer_id);
    }

    pub fn add_elementarraybuffer(&mut self, elements: &[u32]) {
        let id = buffer_data(elements, gl::ELEMENT_ARRAY_BUFFER);
        self.vertex_count = elements.len() as i32;
        self.buffer_ids.push(id);
    }

    fn cleanup(&self) {
        unsafe {
            println!("mesh cleanup {}", self.array_id);
            for id in self.buffer_ids.iter() {
                gl::DeleteBuffers(1, id);
            }
            gl::DeleteVertexArrays(1, &self.array_id);
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        self.cleanup();
    }
}

pub struct MeshRepo {
    unique_index: UniqueIndex,
    mesh_i_data: Vec<(Mesh, usize)>,
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
    pub fn draw_elements(&self) {
        mesh_util::draw_elements(self.vertex_count);
    }
}

impl MeshRepo {
    pub fn new() -> Self {
        MeshRepo {
            unique_index: UniqueIndex::new(),
            mesh_i_data: Vec::new(),
        }
    }

    pub fn add_mesh<T: Fn(&mut Mesh)>(&mut self, init_mesh: T) -> MeshToken {
        let index = self.unique_index.get_next();

        let mut mesh = Mesh::new();
        init_mesh(&mut mesh);

        let mesh_token = MeshToken {
            uid: index,
            array_id: mesh.array_id,
            vertex_count: mesh.vertex_count,
        };

        self.mesh_i_data.push((mesh, index));

        mesh_token
    }

    pub fn get_mesh(&self, uid: u32) -> Option<&Mesh> {
        self.mesh_i_data
            .binary_search_by_key(&uid, |x| x.1 as u32)
            .ok()
            .map(|i| &self.mesh_i_data[i].0)
    }

    fn cleanup(&self) {
        for mesh_i in self.mesh_i_data.iter() {
            mesh_i.0.cleanup();
        }
    }
}
