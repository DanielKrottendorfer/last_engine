#version 460

layout (lines) in;
layout (triangle_strip, max_vertices = 3) out;

uniform mat4 projection;

in VS_OUT {
    float force;
} gs_in[];

out float f;

void build_arrow(vec4 root,vec4 tip)
{    
    vec2 v = (tip-root).xy/2.0;
    vec2 vn =  normalize(-v) * 0.01 * mat2(0,-1,1,0);

    vec4 tip2 = tip - vec4(v,0,0);

    gl_Position = tip2;
    EmitVertex();
    gl_Position = tip2 + vec4(-v  * 0.1 + vn,0,0);
    EmitVertex();
    gl_Position = tip2 + vec4(-v * 0.1 - vn,0,0);
    EmitVertex();
    EndPrimitive();
}

void main()
{
    f = gs_in[0].force;
    if (( gl_in[0].gl_Position-gl_in[1].gl_Position ).y > 0.0)
        build_arrow(gl_in[0].gl_Position,gl_in[1].gl_Position);
    else
        build_arrow(gl_in[1].gl_Position,gl_in[0].gl_Position);
}  
