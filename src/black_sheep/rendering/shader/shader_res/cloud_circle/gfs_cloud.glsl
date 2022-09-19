#version 450

out layout (location = 0) vec4 out_color;

in GS_OUT {
    vec4 pos;
    vec3 color;
    flat vec4 center;
} gs_out;

void main()
{
    
    if (length(gs_out.pos-gs_out.center) < 0.5)
        out_color = vec4(gs_out.color, 1.0) ;
        
}  