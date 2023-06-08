#version 450

// Input vertex data, different for all executions of this shader.
in layout (location = 0) vec2 position;
in layout (location = 1) vec3 col;

uniform mat4 proj;

out vec3 color;

void main(){
	gl_Position =  proj*vec4(position,0.0,1.0);
	color = col;
}

