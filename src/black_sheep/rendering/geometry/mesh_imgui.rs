use imgui::DrawData;

use super::mesh_util::{buffer_data, gen_vertexarray, set_attribute_pointer_for_nerds};

#[derive(Debug)]
pub struct ImguiMesh {
    vertex_array_id: u32,
    vertex_buffer_id: u32,
    element_buffer_id: u32,
    pub vertex_count: i32,
}

impl ImguiMesh {
    pub fn new(
        vertex_array_id: u32,
        vertex_buffer_id: u32,
        element_buffer_id: u32,
        vertex_count: i32,
    ) -> ImguiMesh {
        ImguiMesh {
            vertex_array_id,
            vertex_buffer_id,
            element_buffer_id,
            vertex_count,
        }
    }

    pub fn cleanup(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vertex_buffer_id);
            gl::DeleteBuffers(1, &self.element_buffer_id);
            gl::DeleteVertexArrays(1, &self.vertex_array_id);
        }
    }

    pub fn bind_vertex_array(&self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array_id);
        }
    }
    pub fn draw(&self) {
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.vertex_count,
                gl::UNSIGNED_SHORT,
                0 as *const std::ffi::c_void,
            );
        }
    }
}

pub fn new_imguimesh(draw_data: &DrawData) -> Vec<ImguiMesh> {
    draw_data
        .draw_lists()
        .map(|draw_list| {
            let vtx_buffer = draw_list.vtx_buffer();
            let idx_buffer = draw_list.idx_buffer(); 

            let vertex_array_id: u32 = gen_vertexarray();

            let vertex_count = idx_buffer.len() as i32;

            let vertex_buffer_id = buffer_data(vtx_buffer, gl::ARRAY_BUFFER);
            let element_buffer_id = buffer_data(idx_buffer, gl::ELEMENT_ARRAY_BUFFER);

            set_attribute_pointer_for_nerds(0, gl::FLOAT, 2, 20, 0);
            set_attribute_pointer_for_nerds(1, gl::FLOAT, 2, 20, 4 * 2);
            set_attribute_pointer_for_nerds(2, gl::UNSIGNED_BYTE, 4, 20, 4 * 2 * 2);

            ImguiMesh::new(
                vertex_array_id,
                vertex_buffer_id,
                element_buffer_id,
                vertex_count,
            )
        })
        .collect()
}