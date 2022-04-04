#version 450
#extension GL_EXT_gpu_shader4 : enable
out layout (location = 0) vec4 out_color;


in GS_OUT {
    float color;
} fs_in;

void main()
{
    //out_color = vec4(1.0,fs_in.color,1.0,1.0);
    out_color = vec4(fs_in.color,0.0,0.0,1.0);
}  