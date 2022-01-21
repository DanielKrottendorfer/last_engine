use gl::types::GLushort;
use imgui::{DrawCmdParams, DrawData, DrawList};

use crate::black_sheep::rendering::shader;

use super::mesh_util::*;

#[derive(Debug)]
pub struct ImguiMesh {
    vertex_array_id: u32,
    vertex_buffer_id: u32,
    element_buffer_id: u32,
    draw_params: Vec<(i32, DrawCmdParams)>,
    imgui_shader: shader::shader_structs::ImguiShaderProgram,
}

impl ImguiMesh {
    pub fn new(
        vertex_array_id: u32,
        vertex_buffer_id: u32,
        element_buffer_id: u32,
        draw_params: Vec<(i32, DrawCmdParams)>,
    ) -> Self {
        Self {
            vertex_array_id,
            vertex_buffer_id,
            element_buffer_id,
            draw_params,
            imgui_shader: shader::get_shader_repo().imgui,
        }
    }
    pub fn bind_vertex_array(&self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array_id);
        }
    }
    pub fn draw(&self, window_size: [f32; 2]) {
        for dp in self.draw_params.iter() {
            let count = dp.0;
            let offset = dp.1.idx_offset * std::mem::size_of::<GLushort>();
            let (left, up, right, down) = (
                dp.1.clip_rect[0],
                window_size[1] - dp.1.clip_rect[1],
                dp.1.clip_rect[2],
                window_size[1] - dp.1.clip_rect[3],
            );

            self.imgui_shader.set_tex(dp.1.texture_id.id() as i32);

            unsafe {
                gl::Scissor(
                    (left) as i32,
                    (down) as i32,
                    (right - left) as i32,
                    (up - down) as i32,
                );
                gl::DrawElements(
                    gl::TRIANGLES,
                    count,
                    gl::UNSIGNED_SHORT,
                    offset as u16 as *const std::ffi::c_void,
                );
            }
        }
    }

    pub fn update_vertex_buffer(&mut self, draw_list: &DrawList) {
        let vtx_buffer = draw_list.vtx_buffer();
        let idx_buffer = draw_list.idx_buffer();

        self.draw_params = draw_list
            .commands()
            .map(|d| match d {
                imgui::DrawCmd::Elements { count, cmd_params } => (count as i32, cmd_params),
                imgui::DrawCmd::ResetRenderState => todo!(),
                imgui::DrawCmd::RawCallback { .. } => todo!(),
            })
            .collect::<Vec<(i32, DrawCmdParams)>>();

        bind_vertex_array(self.vertex_array_id);

        update_buffer_data(vtx_buffer, self.vertex_buffer_id, gl::ARRAY_BUFFER);
        update_buffer_data(idx_buffer, self.element_buffer_id, gl::ELEMENT_ARRAY_BUFFER);
    }
    fn cleanup(&self) {
        #[cfg(not(feature = "debug_off"))]
        println!("imguimesh cleanup {}", self.vertex_array_id);

        unsafe {
            gl::DeleteBuffers(1, &self.vertex_buffer_id);
            gl::DeleteBuffers(1, &self.element_buffer_id);
            gl::DeleteVertexArrays(1, &self.vertex_array_id);
        }
    }
}

impl Drop for ImguiMesh {
    fn drop(&mut self) {
        self.cleanup();
    }
}

pub fn imguimesh_from_drawdata(draw_data: &DrawData) -> Vec<ImguiMesh> {
    draw_data
        .draw_lists()
        .map(|draw_list| {
            let draw_params = draw_list
                .commands()
                .map(|d| match d {
                    imgui::DrawCmd::Elements { count, cmd_params } => (count as i32, cmd_params),
                    imgui::DrawCmd::ResetRenderState => todo!(),
                    imgui::DrawCmd::RawCallback { .. } => todo!(),
                })
                .collect::<Vec<(i32, DrawCmdParams)>>();

            let vtx_buffer = draw_list.vtx_buffer();
            let idx_buffer = draw_list.idx_buffer();

            let vertex_array_id: u32 = gen_vertexarray();

            let vertex_buffer_id = buffer_data_static(vtx_buffer, gl::ARRAY_BUFFER);
            let element_buffer_id = buffer_data_static(idx_buffer, gl::ELEMENT_ARRAY_BUFFER);

            set_attribute_pointer_for_nerds(0, gl::FLOAT, 2, 20, 0);
            set_attribute_pointer_for_nerds(1, gl::FLOAT, 2, 20, 4 * 2);
            set_attribute_pointer_for_nerds(2, gl::UNSIGNED_BYTE, 4, 20, 4 * 2 * 2);

            ImguiMesh::new(
                vertex_array_id,
                vertex_buffer_id,
                element_buffer_id,
                draw_params,
            )
        })
        .collect()
}
