pub mod shader_structs;
mod shader_util;

use std::{borrow::Borrow, sync::Once};

use shader_structs::*;
use shader_util::*;

pub struct ShaderRepo {
    imgui:       ImguiShaderProgram,
    point_cloud: CloudGeometryShaderProgram,
}


impl ShaderRepo{
    pub fn new() -> Self {
        let mut point_cloud = CloudGeometryShaderProgram::new();
        {
            let gvs = compile_shader(GVS_SRC_CLOUD, gl::VERTEX_SHADER);
            let gs  = compile_shader(GS_SRC_CLOUD, gl::GEOMETRY_SHADER);
            let gfs = compile_shader(GFS_SRC_CLOUD, gl::FRAGMENT_SHADER);
            let program = link_shaders(gvs, Some(gs), gfs);
            delete_shader(gfs);
            delete_shader(gs);
            delete_shader(gvs);
            point_cloud.setup(&program);
        }
    
        let mut imgui = ImguiShaderProgram::new();
        {
            let vs = compile_shader(IMGUI_VS_SRC, gl::VERTEX_SHADER);
            let fs = compile_shader(IMGUI_FS_SRC, gl::FRAGMENT_SHADER);
            let program = link_shaders(vs, None, fs);
            delete_shader(fs);
            delete_shader(vs);
            imgui.setup(&program);
        }
        
        ShaderRepo{
            imgui,
            point_cloud
        }
    }
    fn cleanup(&mut self) {
        self.imgui.cleanup();
        self.point_cloud.cleanup();
    }
}

impl Drop for ShaderRepo{
    fn drop(&mut self) {
        self.cleanup();
    }
}
