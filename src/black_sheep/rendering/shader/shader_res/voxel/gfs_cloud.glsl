#version 450
out layout (location = 0) vec4 out_color;

uniform vec3 gEyeWorldPos;

in GS_OUT {
    vec4 color;
    vec3 norm;
    vec3 w_pos;
} fs_in;

void main()
{

    vec3 LightDirection = normalize(vec3(1.0,-1.0,1.0));
    vec3 Normal = normalize(fs_in.norm);
    vec4 white_light = vec4(1.0,1.0,1.0,0.0);

    float DiffuseFactor = clamp(dot(LightDirection, Normal),0,1);


    vec3 VertexToEye = normalize(gEyeWorldPos - fs_in.w_pos);
    vec3 LightReflect = normalize(reflect(-LightDirection, Normal));
    float SpecularFactor = clamp(dot(VertexToEye, LightReflect),0,1);

    out_color =  fs_in.color * 0.5 +
    (white_light * DiffuseFactor * 0.3) + 
    (white_light * pow(SpecularFactor*0.9 ,7.0)) ;

    out_color.w = 1.0;
} 