#version 450

in layout (location = 0) vec2 position;
in layout (location = 1) float radius;
in layout (location = 2) vec3 color;

out VS_OUT {
    vec3 color;
    float radius;
} vs_out;

void main(){
	gl_Position =  vec4(position,0,1);
    vs_out.radius = radius;
    vs_out.color = color;
}

