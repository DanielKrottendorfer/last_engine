use cgmath::Matrix4;

use my_macro::ShaderProgram;


use std::ffi::CString;

#[derive(Debug, ShaderProgram, Clone)]
pub struct CloudGeometryShaderProgram {
    program_id: u32,
    uniform_mat4_mv: i32,
    uniform_mat4_projection: i32,
}


#[derive(Debug, ShaderProgram, Clone)]
pub struct ImguiShaderProgram {
    program_id: u32,
    uniform_mat4_matrix: i32,
    uniform_i_tex: i32,
}


#[allow(non_snake_case, dead_code)]
mod not_in_use{
    use cgmath::Matrix4;
    use my_macro::ShaderProgram;
    use std::ffi::CString;

    #[derive(Debug, ShaderProgram)]
    pub struct LineGeometryShaderProgram {
        program_id: u32,
        uniform_mat4_mvp: i32,
    }
    
    #[derive(Debug, ShaderProgram)]
    pub struct ArrowGeometryShaderProgram {
        program_id: u32,
        uniform_mat4_mvp: i32,
    }

    #[derive(Debug, ShaderProgram)]
    pub struct BasicShaderProgram {
        pub program_id: u32,
        uniform_mat4_MVP: i32,
    }

    #[derive(Debug, ShaderProgram)]
    pub struct StandardShadingTransparent {
        program_id: u32,
        uniform_mat4_MVP: i32,
        uniform_i_myTextureSampler: i32,
    }
    
    #[derive(Debug, ShaderProgram)]
    pub struct NormalGeometryShaderProgram {
        program_id: u32,
        uniform_mat4_mvp: i32,
        uniform_mat4_projection: i32,
    }
}