#version 460

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 norm;

out VS_OUT {
    vec3 normal;
} vs_out;

uniform mat4 mvp;

void main()
{
    gl_Position = mvp * vec4(position, 1.0); 

    vec4 normal_transpose = mvp*vec4(norm,0.0);
    //vec4 normal_transpose = transpose(inverse(mvp))*vec4(norm,1.0);

    vs_out.normal = normalize(vec3(normal_transpose));
}