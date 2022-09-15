#version 450

out layout (location = 0) vec4 out_color;

in GS_OUT {
    vec4 pos;
    flat vec4 center;
} gs_out;

void main()
{
    vec4 c = vec4(1,0,0,1);
    
    if (length(gs_out.pos-gs_out.center) < 1.0)
        out_color = c ;
        
}  