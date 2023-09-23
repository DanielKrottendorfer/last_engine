#version 450

in layout (location = 0) vec3 position;
in layout (location = 1) vec3 normal;

uniform mat4 MVP;
uniform mat4 M;

out vec4 world_pos;
out vec4 world_nor;

void main(){
	world_pos = M * vec4(position,1);
	world_nor = M * vec4(normal,0);
	gl_Position =  MVP * vec4(position,1);
}

