#version 450

layout (points) in;
layout (triangle_strip, max_vertices = 3) out;

in VS_OUT {
    vec3 color;
} vs_out[];

out GS_OUT {
    vec4 pos;
    vec3 color;
    flat vec4 center;
} gs_out;

out vec4 color2;

uniform mat4 projection;

float r_constant = 3.4641016151377545;
float r = 0.5;
float a2 = r * r_constant / 2.0;

void build_triangle(vec4 position)
{    
    gs_out.center = gl_in[0].gl_Position;

    vec4 t = (position + vec4(-a2,  -r, 0.0, 0.0));    // 3:bot-left;
    gs_out.pos = t;
    gs_out.color = vs_out[0].color;
    gl_Position = projection * t;
    EmitVertex();

    t = (position + vec4( a2,  -r, 0.0, 0.0));    // 4:bot-right
    gs_out.pos = t;
    gs_out.color = vs_out[0].color;
    gl_Position = projection * t;
    EmitVertex();

    t = (position + vec4( 0.0, r * 2.0, 0.0, 0.0));    // 5:top
    gs_out.pos = t;
    gs_out.color = vs_out[0].color;
    gl_Position = projection * t;
    EmitVertex();
    EndPrimitive();

}

void main()
{
    
    build_triangle(gl_in[0].gl_Position);
}  
