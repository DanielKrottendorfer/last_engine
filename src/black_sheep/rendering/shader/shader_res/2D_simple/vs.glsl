#version 450

in layout (location = 0) vec2 position;
in layout (location = 1) vec3 color;
in layout (location = 2) float radius;

out VS_OUT {
    vec3 color;
    float radius;
} vs_out;


uniform mat4 projection;


void main(){
	gl_Position =  projection * vec4(position,0,1);
    vs_out.color = color;
    vs_out.radius = radius;
}

