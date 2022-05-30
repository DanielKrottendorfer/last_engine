#version 450

out layout (location = 0) vec4 out_color;

in GS_OUT {
    vec4 color;
    vec4 pos;
    flat vec4 center;
} gs_out;

void main()
{
    vec4 c = gs_out.color;
    
    if (length(gs_out.pos-gs_out.center) > 0.02)
        c.w = 0;
        
    out_color = c ;
}  