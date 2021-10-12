#version 460

layout (location = 0) in vec3 position;
layout (location = 1) in vec4 color;


out VS_OUT {
    vec4 color;
} vs_out;

uniform mat4 mv;

void main()
{
    vs_out.color = color;
    gl_Position = mv * vec4(position, 1.0); 
}