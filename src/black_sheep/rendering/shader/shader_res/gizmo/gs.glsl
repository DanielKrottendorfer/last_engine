#version 450

layout (points) in;
layout (line_strip, max_vertices = 2) out;


in VS_OUT {
    vec4 color;
} vs_out[];

out vec4 c2;

uniform mat4 projection;

void build_house(vec4 position)
{    
    vec4 temp =  position;
    temp.z = 0.0;
    temp.w = 1.0;
    gl_Position = temp;
    EmitVertex();

    gl_Position = vec4(0.0, 0.0, 0.0, 0.0); 
    EmitVertex();
    EndPrimitive();
}

void main()
{
    c2 = vs_out[0].color;
    build_house(gl_in[0].gl_Position);
}  
