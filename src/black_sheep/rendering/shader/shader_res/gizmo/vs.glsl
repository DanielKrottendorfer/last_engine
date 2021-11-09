#version 460

layout (location = 0) in vec3 position;

out VS_OUT {
    vec4 color;
} vs_out;

uniform mat4 view;

void main()
{
    vs_out.color = vec4(position,1.0);

    vec4 temp = view * vec4(position, 0.0);
    temp.y *= (-1);
    gl_Position = temp;
}