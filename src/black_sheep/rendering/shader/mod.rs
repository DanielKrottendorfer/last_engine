pub mod shader_structs;
mod shader_util;

use std::sync::Mutex;

use shader_structs::*;
use shader_util::*;

#[derive(Default, Clone)]
pub struct ShaderRepo {
    pub imgui: ImguiShaderProgram,
    pub point_cloud: CloudGeometryShaderProgram,
    pub circle_point_cloud: CircleCloudGeometryShaderProgram,
    pub simple: SimpleShaderProgram,
    pub color_3d: Color3D,
    pub color_3d_light: Color3DLight,
    pub gizmo: GizmoProgram,
    pub point_2d: Point2D,
    pub simple_2d: Simple2D,
    pub colored_triangles: ColoredTriangles,
    pub sprite: Sprite,
    pub double_sphere: DoubleSphere
}

lazy_static! {
    static ref SHADER_REPO: Mutex<Option<ShaderRepo>> = Mutex::new(None);
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
    if let Ok(mut sr) = SHADER_REPO.lock() {
        if let Some(sr) = &mut *sr {
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

        let mut circle_point_cloud = CircleCloudGeometryShaderProgram::new();
        {
            let program = build_shader_program(
                GVS_SRC_CIRCLE_CLOUD,
                Some(GS_SRC_CIRCLE_CLOUD),
                GFS_SRC_CIRCLE_CLOUD,
            );
            circle_point_cloud.setup(&program);
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

        let mut color_3d_light = Color3DLight::new();
        {
            let program = build_shader_program(COLOR3DLIGHT_VS_SRC, None, COLOR3DLIGHT_FS_SRC);
            color_3d_light.setup(&program);
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
        let mut simple_2d = Simple2D::new();
        {
            let program = build_shader_program(SIMPLE_2D_VS, None, SIMPLE_2D_FS);
            simple_2d.setup(&program);
        }

        let mut colored_triangles = ColoredTriangles::new();
        {
            let program =
                build_shader_program(COLORED_TRIANGLES_VS_SRC, None, COLORED_TRIANGLES_FS_SRC);
            colored_triangles.setup(&program);
        }

        let mut sprite = Sprite::new();
        {
            let program = build_shader_program(SPRITE_VS_SRC, None, SPRITE_FS_SRC);
            sprite.setup(&program);
        }

        let mut double_sphere = DoubleSphere::new();
        {
            let program = build_shader_program(DOUBLE_VS_SRC, None, DOUBLE_FS_SRC);
            double_sphere.setup(&program);
        }

        ShaderRepo {
            imgui,
            point_cloud,
            circle_point_cloud,
            simple,
            color_3d,
            gizmo,
            point_2d,
            simple_2d,
            colored_triangles,
            color_3d_light,
            sprite,
            double_sphere
        }
    }
    fn cleanup(&mut self) {
        self.imgui.cleanup();
        self.point_cloud.cleanup();
        self.circle_point_cloud.cleanup();
        self.simple.cleanup();
        self.color_3d.cleanup();
        self.color_3d_light.cleanup();
        self.gizmo.cleanup();
        self.point_2d.cleanup();
        self.simple_2d.cleanup();
        self.colored_triangles.cleanup();
        self.double_sphere.cleanup();
    }
}
