#version 450

in layout (location = 0) vec2 position;
in layout (location = 1) vec3 color;

out VS_OUT {
    vec3 color;
} vs_out;

uniform mat4 projection;

void main(){
    vs_out.color = color;
	gl_Position = projection * vec4(position.xy,0,1);
}

