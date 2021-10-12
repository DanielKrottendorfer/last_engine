#version 460

layout (location = 0) in vec3 position;
layout (location = 1) in float force;

out VS_OUT {
    float force;
} vs_out;


uniform mat4 mvp;

void main()
{
    vs_out.force = force;
    gl_Position = mvp * vec4(position, 1.0); 
} 