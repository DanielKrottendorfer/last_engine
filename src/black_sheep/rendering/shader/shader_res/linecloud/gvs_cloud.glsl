#version 460

layout (location = 0) in vec3 position;
layout (location = 1) in vec4 color;

out vec4 fs_color;

uniform mat4 mvp;

void main()
{
    fs_color = color;
    gl_Position = mvp * vec4(position, 1.0); 
}