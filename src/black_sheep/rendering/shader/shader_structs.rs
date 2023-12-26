use cgmath::{Matrix4, Vector3,Vector2};
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
    CircleCloudGeometryShaderProgram {
        uniform mat4 mv;
        uniform mat4 projection;
    }
);

shader_program!(
    SimpleShaderProgram{
        uniform vec3 color;
    }
);

shader_program!(
    Color3D{
        uniform mat4 MVP;
        uniform vec3 col;
    }
);

shader_program!(
    DoubleSphere{
        uniform mat4 M;
        uniform vec3 col;
        uniform vec3 light_position;
        uniform float aa;
        uniform float tt;
        uniform float light_power;
    }
);

shader_program!(
    Color3DLight{
        uniform mat4 MVP;
        uniform mat4 M;
        uniform vec3 col;
        uniform vec3 light_position;
        uniform float light_power;
    }
);

shader_program!(
    GizmoProgram{
        uniform mat4 view;
    }
);

shader_program!(
    Point2D{
        uniform mat4 projection;
    }
);
shader_program!(
    Simple2D{
        uniform mat4 projection;
    }
);

shader_program!(
    ColoredTriangles{
        uniform mat4 projection;
    }
);

shader_program!(
    Sprite{
        uniform int myTextureSampler;
        uniform vec3 CameraRight_worldspace;
        uniform vec3 CameraUp_worldspace;
        uniform mat4 VP; 
        uniform vec3 BillboardPos; 
        uniform vec2 BillboardSize; 
    }
);
