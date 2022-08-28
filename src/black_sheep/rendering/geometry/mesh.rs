use super::mesh_util::*;

pub struct Mesh {
    pub uid: usize,
    buffer_ids: Vec<u32>,
    array_id: u32,
    pub vertex_count: i32,
}

impl Mesh {
    pub fn new() -> Self {
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
        bind_vertex_array(self.array_id);
    }
    pub fn draw_triangle_elements(&self) {
        draw_triangle_elements(self.vertex_count);
    }
    pub fn draw_point_elements(&self) {
        draw_point_elements(self.vertex_count);
    }

    pub fn cleanup(&self) {
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
#[derive(Debug, Default, Clone)]
pub struct MeshToken {
    pub uid: usize,
    array_id: u32,
    vertex_count: i32,
}

impl MeshToken {
    pub fn bind_vertex_array(&self) {
        bind_vertex_array(self.array_id);
    }
    pub fn draw_triangle_elements(&self) {
        draw_triangle_elements(self.vertex_count);
    }
    pub fn draw_point_elements(&self) {
        draw_point_elements(self.vertex_count);
    }
}
