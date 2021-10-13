use gl::types::*;
use std::ffi::CString;
use std::ptr;
use std::str;

// // Shader sources

pub static GVS_SRC_CLOUD: &'static str = include_str!("./shader_res/cloud/gvs_cloud.glsl");
pub static GS_SRC_CLOUD: &'static str = include_str!("./shader_res/cloud/gs_cloud.glsl");
pub static GFS_SRC_CLOUD: &'static str = include_str!("./shader_res/cloud/gfs_cloud.glsl");

pub static IMGUI_FS_SRC: &'static str = include_str!("./shader_res/imgui/glsl_400.frag");
pub static IMGUI_VS_SRC: &'static str = include_str!("./shader_res/imgui/glsl_400.vert");

pub static SIMPLE_VS_SRC: &'static str = include_str!("./shader_res/simple/vs.glsl");
pub static SIMPLE_FS_SRC: &'static str = include_str!("./shader_res/simple/fs.glsl");

pub fn build_shader_program(
	vertex_shader: &str, 
	geometry_shader: Option<&str>, 
	fragment_shader: &str) -> u32 {

		let vs = compile_shader(vertex_shader, gl::VERTEX_SHADER);
		let gs = geometry_shader.map(|gs| compile_shader(gs, gl::GEOMETRY_SHADER));
		let fs = compile_shader(fragment_shader, gl::FRAGMENT_SHADER);

		let program = link_shaders(vs, gs, fs);

		delete_shader(fs);
		gs.map(|gs|delete_shader(gs));
		delete_shader(vs);

		program
}

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