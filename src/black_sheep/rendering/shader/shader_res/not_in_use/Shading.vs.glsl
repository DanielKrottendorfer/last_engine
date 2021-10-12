#version 460

// Input vertex data, different for all executions of this shader.
in layout (location = 0) vec3 position;
in layout (location = 1) vec3 normal;
in layout (location = 2) vec2 uv;

out vec2 UV;

uniform int I[10]; 
uniform mat4 MVP;


void main(){

	gl_Position =  MVP * vec4(position,1) * I[2];
	
	UV = uv;
}

