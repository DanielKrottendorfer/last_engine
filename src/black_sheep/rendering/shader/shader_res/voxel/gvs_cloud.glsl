#version 450

layout (location = 0) in vec3 position;
layout (location = 1) in float color;

out VS_OUT {
    float color;
} vs_out;

void main()
{
    vs_out.color = color;
    gl_Position = vec4(position, 1.0); 
}