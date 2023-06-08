#version 450

// Input vertex data, different for all executions of this shader.
in layout (location = 0) vec2 position;

uniform mat4 proj;

void main(){
	gl_Position =  proj*vec4(position,0.0,1.0);
}

