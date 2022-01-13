pub mod shader_structs;
mod shader_util;

use std::sync::Mutex;

use shader_structs::*;
use shader_util::*;

#[derive(Default,Clone)]
pub struct ShaderRepo {
    pub imgui: ImguiShaderProgram,
    pub point_cloud: CloudGeometryShaderProgram,
    pub simple: SimpleShaderProgram,
    pub color_3d: Color3D,
    pub gizmo: GizmoProgram,
    pub point_2d: Point2D,
}

lazy_static!{
    static ref SHADER_REPO:Mutex<Option<ShaderRepo>> = Mutex::new(None);
}

pub fn init() {
    let sr = SHADER_REPO.lock();
    if sr.is_err() {
        panic!("shader_repo locked failed");
    }

    let mut sr = sr.unwrap();

    if sr.is_some() {
        panic!("shader_repo already initialized")
    }

    *sr = Some(ShaderRepo::new());
}

pub fn cleanup() {
    if let Ok(mut sr) = SHADER_REPO.lock(){
        if let Some(sr) = &mut *sr{
            sr.cleanup();
        }
    }
}

pub fn get_shader_repo() -> ShaderRepo {
    let sr = SHADER_REPO.lock();
    if sr.is_err() {
        panic!("shader_repo locked failed");
    }

    let sr = sr.unwrap();
    if sr.is_none() {
        panic!("shader_repo already initialized");
    }

    sr.clone().unwrap()
}

impl ShaderRepo {
    fn new() -> Self {
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

        let mut color_3d = Color3D::new();
        {
            let program = build_shader_program(COLOR3D_VS_SRC, None, COLOR3D_FS_SRC);
            color_3d.setup(&program);
        }

        let mut gizmo = GizmoProgram::new();
        {
            let program = build_shader_program(GIZMO_VS, Some(GIZMO_GS), GIZMO_FS);
            gizmo.setup(&program);
        }

        let mut point_2d = Point2D::new();
        {
            let program = build_shader_program(CIRLE_2D_VS, Some(CIRLE_2D_GS), CIRLE_2D_FS);
            point_2d.setup(&program);
        }

        ShaderRepo {
            imgui,
            point_cloud,
            simple,
            color_3d,
            gizmo,
            point_2d,
        }
    }
    fn cleanup(&mut self) {
        self.imgui.cleanup();
        self.point_cloud.cleanup();
        self.simple.cleanup();
        self.color_3d.cleanup();
        self.gizmo.cleanup();
        self.point_2d.cleanup();
    }
}

