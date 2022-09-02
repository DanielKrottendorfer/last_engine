#version 450

in layout (location = 0) vec3 position;
in layout (location = 1) vec3 color;

uniform mat4 MVP;

void main(){

	gl_Position =  MVP * vec4(position,1);
}

