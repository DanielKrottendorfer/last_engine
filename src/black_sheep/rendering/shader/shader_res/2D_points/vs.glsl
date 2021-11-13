#version 450

in layout (location = 0) vec2 position;
// uniform mat4 projection;
uniform float radius;

out VS_OUT {
    vec3 color;
} vs_out;

void main(){
	gl_Position =  vec4(position,0,1);
}

