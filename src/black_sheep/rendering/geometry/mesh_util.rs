use gl::{types::*, POINTS, TRIANGLES};
use std::ffi::c_void;

pub fn set_attribute_pointer(index: u32, gl_type: u32, size: i32) {
    set_attribute_pointer_for_nerds(index, gl_type, size, 0, 0);
}

pub fn set_attribute_pointer_for_nerds(
    index: u32,
    gl_type: u32,
    size: i32,
    stride: i32,
    offset: i32,
) {
    unsafe {
        gl::EnableVertexAttribArray(index);
        gl::VertexAttribPointer(
            index,
            size,
            gl_type,
            gl::FALSE as GLboolean,
            stride,
            offset as *const c_void,
        );
    }
}

pub fn buffer_data_static<T>(buffer_data: &[T], buffer_type: GLenum) -> u32 {
    let mut vertex_buffer_id = 0;

    unsafe {
        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vertex_buffer_id);
        gl::BindBuffer(buffer_type, vertex_buffer_id);
        gl::BufferData(
            buffer_type,
            (buffer_data.len() * std::mem::size_of::<T>()) as GLsizeiptr,
            std::mem::transmute(&buffer_data[0]),
            gl::STATIC_DRAW,
        );
    }

    vertex_buffer_id
}
pub fn buffer_data_dynamic<T>(buffer_data: &[T], buffer_type: GLenum) -> u32 {
    let mut vertex_buffer_id = 0;

    unsafe {
        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vertex_buffer_id);
        gl::BindBuffer(buffer_type, vertex_buffer_id);
        gl::BufferData(
            buffer_type,
            (buffer_data.len() * std::mem::size_of::<T>()) as GLsizeiptr,
            std::mem::transmute(&buffer_data[0]),
            gl::DYNAMIC_DRAW,
        );
    }

    vertex_buffer_id
}

pub fn update_buffer_data<T>(buffer_data: &[T], buffer_id: u32, buffer_type: GLenum) {
    unsafe {
        gl::BindBuffer(buffer_type, buffer_id);
        gl::BufferData(
            buffer_type,
            (buffer_data.len() * std::mem::size_of::<T>()) as GLsizeiptr,
            std::mem::transmute(&buffer_data[0]),
            gl::DYNAMIC_DRAW,
        );
    }
    // unsafe {
    //     gl::BindBuffer(buffer_type, buffer_id);
    //     gl::BufferSubData(
    //         buffer_type,
    //         0,
    //         (buffer_data.len() * std::mem::size_of::<T>()) as GLsizeiptr,
    //         std::mem::transmute(&buffer_data[0]),
    //     );
    // }
}

pub fn gen_vertexarray() -> u32 {
    let mut vertex_array_id: u32 = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vertex_array_id);
        gl::BindVertexArray(vertex_array_id);
    }
    vertex_array_id
}

pub fn bind_vertex_array(array_id: u32) {
    unsafe {
        gl::BindVertexArray(array_id);
    }
}

pub fn draw_triangle_elements(vertex_count: i32) {
    unsafe {
        gl::DrawElements(
            TRIANGLES,
            vertex_count,
            gl::UNSIGNED_INT,
            0 as *const std::ffi::c_void,
        );
    }
}
pub fn draw_point_elements(vertex_count: i32) {
    unsafe {
        gl::DrawElements(
            POINTS,
            vertex_count,
            gl::UNSIGNED_INT,
            0 as *const std::ffi::c_void,
        );
    }
}

pub fn draw_point_array(vertex_count: i32) {
    unsafe {
        gl::DrawArrays(POINTS, 0, vertex_count);
    }
}
