#version 450

layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

uniform mat4 projection;

in VS_OUT {
    vec3 color;
    float radius;
} gs_in[];

out GS_OUT {
    vec2 screen_pos;
    vec2 center_pos;
    vec3 color;
    float radius;
} gs_out;


void main()
{
    vec4 position = gl_in[0].gl_Position;


    gs_out.center_pos = position.xy;
    gs_out.color = gs_in[0].color;
    gs_out.radius = gs_in[0].radius;


    vec4 p1 = (position + vec4( gs_in[0].radius, gs_in[0].radius, 0.0, 0.0));
    gs_out.screen_pos = p1.xy;
    gl_Position = projection * p1;
    EmitVertex();

    vec4 p2 = (position + vec4( -gs_in[0].radius, gs_in[0].radius, 0.0, 0.0));
    gs_out.screen_pos = p2.xy;
    gl_Position = projection * p2;
    EmitVertex();

    vec4 p3 = (position + vec4( gs_in[0].radius, -gs_in[0].radius, 0.0, 0.0));
    gs_out.screen_pos = p3.xy;
    gl_Position = projection * p3;
    EmitVertex();

    vec4 p4 = (position + vec4( -gs_in[0].radius, -gs_in[0].radius, 0.0, 0.0));
    gs_out.screen_pos = p4.xy;
    gl_Position = projection * p4;
    EmitVertex();

    EndPrimitive();
}  
