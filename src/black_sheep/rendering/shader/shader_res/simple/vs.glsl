#version 460

// Input vertex data, different for all executions of this shader.
in layout (location = 0) vec3 position;

void main(){
	gl_Position =  vec4(position,1.0);
}

