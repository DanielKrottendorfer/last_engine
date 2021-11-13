#version 460

out vec4 out_color;
in float f;


void main()
{
    out_color = vec4(1.0 - abs(f)*2 , abs(f)*2 , 0.0 , 1.0);
}  