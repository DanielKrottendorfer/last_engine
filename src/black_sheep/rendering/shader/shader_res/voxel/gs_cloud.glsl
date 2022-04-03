#version 450

layout (points) in;
layout (triangle_strip, max_vertices = 3) out;

uniform mat4 projection;

void build_house(vec4 position)
{    
    gl_Position = projection * (position + vec4(-0.02,  -0.02, 0.0, 0.0));    // 3:bot-left
    EmitVertex();
    gl_Position = projection * (position + vec4( 0.02,  -0.02, 0.0, 0.0));    // 4:bot-right
    EmitVertex();
    gl_Position = projection * (position + vec4( 0.0,  0.02, 0.0, 0.0));    // 5:top
    EmitVertex();
    EndPrimitive();
}

void main()
{
    build_house(gl_in[0].gl_Position);
}  
