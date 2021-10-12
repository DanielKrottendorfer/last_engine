use cgmath::{Matrix3, Vector3};
use gl::types::*;
use std::ffi::{c_void, CString};
use std::ptr;
use std::str;

// // Shader sources

pub static GVS_SRC_CLOUD: &'static str = include_str!("./shader_res/cloud/gvs_cloud.glsl");
pub static GS_SRC_CLOUD: &'static str = include_str!("./shader_res/cloud/gs_cloud.glsl");
pub static GFS_SRC_CLOUD: &'static str = include_str!("./shader_res/cloud/gfs_cloud.glsl");

pub static IMGUI_FS_SRC: &'static str = include_str!("./shader_res/imgui/glsl_400.frag");
pub static IMGUI_VS_SRC: &'static str = include_str!("./shader_res/imgui/glsl_400.vert");

pub fn compile_shader(src: &str, _type: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(_type);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}

pub fn link_shaders(vs: GLuint, gs: Option<GLuint>, fs: GLuint) -> GLuint {
    let program;
    unsafe {
        program = gl::CreateProgram();

        gl::AttachShader(program, vs);
        if let Some(gs) = gs {
            gl::AttachShader(program, gs);
        }
        gl::AttachShader(program, fs);

        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
            );
        }
    }
    program
}

pub fn delete_shader(shader_id: u32){
    unsafe {
        gl::DeleteShader(shader_id);
    }
}

pub fn three_d_rendering_setup() {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
}

pub fn ui_rendering_setup() {
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendEquation(gl::FUNC_ADD);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Disable(gl::CULL_FACE);
        gl::Disable(gl::DEPTH_TEST);
        //gl::Enable(gl::SCISSOR_TEST);
    }
}

pub fn view_to_screen(w: f32, h: f32) -> Matrix3<f32> {
    Matrix3::from_cols(
        Vector3::new(2.0 / w, 0.0, 0.0),
        Vector3::new(0.0, -2.0 / h, 0.0),
        Vector3::new(-1.0, 1.0, 1.0),
    )
}
