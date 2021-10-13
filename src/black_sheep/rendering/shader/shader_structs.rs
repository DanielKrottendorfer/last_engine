use cgmath::{Matrix4,Vector3};
use shader_macro::shader_program;

use std::ffi::CString;

shader_program!(
	ImguiShaderProgram {
		uniform mat4 matrix;
		uniform int tex;
	}
);

shader_program!(
	CloudGeometryShaderProgram {
		uniform mat4 mv;
		uniform mat4 projection;
	}
);

shader_program!(
	SimpleShaderProgram{
		uniform vec3 color;
	}
);
