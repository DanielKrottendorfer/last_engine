pub mod shader_structs;
mod shader_util;
use shader_structs::*;
use shader_util::*;

pub struct ShaderRepo {
    pub imgui: ImguiShaderProgram,
    pub point_cloud: CloudGeometryShaderProgram,
    pub simple: SimpleShaderProgram,
    pub color_3d: Color3D
}

impl ShaderRepo {
    pub fn new() -> Self {
        let mut point_cloud = CloudGeometryShaderProgram::new();
        {
            let program = build_shader_program(GVS_SRC_CLOUD, Some(GS_SRC_CLOUD), GFS_SRC_CLOUD);
            point_cloud.setup(&program);
        }

        let mut imgui = ImguiShaderProgram::new();
        {
            let program = build_shader_program(IMGUI_VS_SRC, None, IMGUI_FS_SRC);
            imgui.setup(&program);
        }

        let mut simple = SimpleShaderProgram::new();
        {
            let program = build_shader_program(SIMPLE_VS_SRC, None, SIMPLE_FS_SRC);
            simple.setup(&program);
        }

        let mut color_3d= Color3D::new();
        {
            
            let program = build_shader_program(COLOR3D_VS_SRC, None, COLOR3D_FS_SRC);
            color_3d.setup(&program);
        }

        ShaderRepo {
            imgui,
            point_cloud,
            simple,
            color_3d,
        }
    }
    fn cleanup(&mut self) {
        self.imgui.cleanup();
        self.point_cloud.cleanup();
        self.color_3d.cleanup();
        self.simple.cleanup();
    }
}

impl Drop for ShaderRepo {
    fn drop(&mut self) {
        println!("shaderrepo cleanup");
        self.cleanup();
    }
}
