pub mod shader_structs;
mod shader_util;
use shader_structs::*;
use shader_util::*;

pub struct ShaderRepo {
	pub imgui:			ImguiShaderProgram,
	pub point_cloud:	CloudGeometryShaderProgram,
	pub simple:			SimpleShaderProgram,
}

impl ShaderRepo{
	pub fn new() -> Self {
		let mut point_cloud = CloudGeometryShaderProgram::new();
		{
			let program = build_shader_program(
				GVS_SRC_CLOUD, 
				Some(GS_SRC_CLOUD), 
				GFS_SRC_CLOUD);
			point_cloud.setup(&program);
		}
	
		let mut imgui = ImguiShaderProgram::new();
		{
			let program = build_shader_program(
				IMGUI_VS_SRC, 
				None, 
				IMGUI_FS_SRC);
			imgui.setup(&program);
		}
		
		let mut simple = SimpleShaderProgram::new();
		{
			let program = build_shader_program(
				SIMPLE_VS_SRC, 
				None, 
				SIMPLE_FS_SRC);
			simple.setup(&program);
		}
		
		ShaderRepo{
			imgui,
			point_cloud,
			simple
		}
	}
	fn cleanup(&mut self) {
		self.imgui.cleanup();
		self.point_cloud.cleanup();
	}
}

impl Drop for ShaderRepo{
	fn drop(&mut self) {
		println!("shaderrepo cleanup");
		self.cleanup();
	}
}
