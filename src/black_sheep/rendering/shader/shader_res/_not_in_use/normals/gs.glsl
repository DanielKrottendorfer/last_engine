#version 460

layout (triangles) in;
layout (line_strip, max_vertices = 6) out;

in VS_OUT {
    vec3 normal;
} gs_in[];


const float MAGNITUDE = 0.05;
  
uniform mat4 projection;

void GenerateLine(int index)
{
    vec3 P = gl_in[index].gl_Position.xyz;
    vec3 N = gs_in[index].normal.xyz;
    
    gl_Position = projection * vec4(P, 1.0);
    EmitVertex();
    
    gl_Position = projection * vec4(P + N * MAGNITUDE, 1.0);
    EmitVertex();
    
    EndPrimitive();

}

void main()
{
    GenerateLine(0); // first vertex normal
    GenerateLine(1); // second vertex normal
    GenerateLine(2); // third vertex normal
}  
