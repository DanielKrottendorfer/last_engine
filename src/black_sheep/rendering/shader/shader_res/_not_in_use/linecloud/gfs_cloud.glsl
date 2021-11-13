#version 460

out vec4 out_color;

in vec4 fs_color;

void main()
{
    out_color = fs_color;
}  