#version 450

layout (points) in;
layout (triangle_strip, max_vertices = 3) out;


in VS_OUT {
    vec4 color1;
} vs_out[];

out vec4 color2;

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
    color2 = vs_out[0].color1;
    build_house(gl_in[0].gl_Position);
}  
