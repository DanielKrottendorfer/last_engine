use cgmath::{Matrix4, Vector3};
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

shader_program!(
    Color3D{
        uniform mat4 MVP;
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
    ColoredTriangles{
        uniform mat4 projection;
    }
);

shader_program!(
    VoexelProgram{
        uniform mat4 mv;
        uniform mat4 projection;
    }
);

