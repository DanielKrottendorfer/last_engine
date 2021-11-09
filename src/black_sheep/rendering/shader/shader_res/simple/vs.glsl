#version 460

// Input vertex data, different for all executions of this shader.
in layout (location = 0) vec2 position;

void main(){
	gl_Position =  vec4(position,0.0,1.0);
    gl_Position.y = -gl_Position.y;
}

