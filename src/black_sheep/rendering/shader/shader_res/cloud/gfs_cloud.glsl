#version 450

out layout (location = 0) vec4 out_color;

in vec4 color2;

void main()
{
    out_color = color2;
}  