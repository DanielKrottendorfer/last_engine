#version 460

out layout (location = 0) vec4 out_color;

in vec4 c2;

void main()
{
    out_color = c2;
    out_color.w = 1.0;
}  