#version 450
#extension GL_EXT_gpu_shader4 : enable
out layout (location = 0) vec4 out_color;


in GS_OUT {
    vec4 color;
} fs_in;

void main()
{
    out_color = fs_in.color;
}  