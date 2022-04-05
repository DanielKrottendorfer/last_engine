#version 450
#extension GL_EXT_gpu_shader4 : enable
out layout (location = 0) vec4 out_color;


in GS_OUT {
    vec4 color;
    vec3 norm;
} fs_in;

void main()
{
    vec3 LightDirection = vec3(1.0,-1.0,-1.0);
	vec3 n = normalize( fs_in.norm );
	vec3 l = normalize( LightDirection );
	float cosTheta = clamp( dot( n,l ), 0,1 );

    out_color = fs_in.color * cosTheta * 1.4;
    out_color.w = 1.0;

    // out_color = fs_in.norm;
    // out_color.w = 1.0;
}   