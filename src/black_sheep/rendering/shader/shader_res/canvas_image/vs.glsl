#version 450

// Input vertex data, different for all executions of this shader.
in layout (location = 0) vec2 position;
in layout (location = 1) vec2 uv;

out vec2 TexCoord;

uniform mat4 proj;

void main(){
	gl_Position = proj * vec4(position,0.0,1.0);
    TexCoord = uv;
}
