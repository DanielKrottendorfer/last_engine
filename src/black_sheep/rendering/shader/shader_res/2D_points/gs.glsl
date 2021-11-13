#version 450

layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

out vec4 screen_pos;
out vec4 center_pos;

uniform float radius;
uniform mat4 projection;

void build_square(vec4 position)
{    
    center_pos = position;

    vec4 p1 = (position + vec4( radius, radius, 0.0, 0.0));
    screen_pos = p1;
    gl_Position = projection * p1;
    EmitVertex();

    vec4 p2 = (position + vec4( -radius, radius, 0.0, 0.0));
    screen_pos = p2;
    gl_Position = projection * p2;
    EmitVertex();

    vec4 p3 = (position + vec4( radius, -radius, 0.0, 0.0));
    screen_pos = p3;
    gl_Position = projection * p3;
    EmitVertex();

    vec4 p4 = (position + vec4( -radius, -radius, 0.0, 0.0));
    screen_pos = p4;
    gl_Position = projection * p4;
    EmitVertex();

    EndPrimitive();
}

void main()
{
    build_square(gl_in[0].gl_Position);
}  
