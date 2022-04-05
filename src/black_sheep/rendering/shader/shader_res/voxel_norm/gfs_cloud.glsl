#version 450
out layout (location = 0) vec4 out_color;

in GS_OUT {
    vec4 color;
} fs_in;
uniform mat4 v;

void main()
{
    // vec3 LightDirection = mat3(v)*vec3(1.0,-1.0,-1.0);
	// vec3 n = normalize( fs_in.norm );
	// vec3 l = normalize( LightDirection );
	// float cosTheta = clamp( dot( n,l ), 0,1 );

    // out_color = fs_in.color * cosTheta * 1.4;
    // out_color.w = 1.0;
    out_color = fs_in.color;
    // out_color = fs_in.norm;
    // out_color.w = 1.0;
}   