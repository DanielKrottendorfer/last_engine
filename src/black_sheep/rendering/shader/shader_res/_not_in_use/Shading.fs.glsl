#version 460

in vec2 UV;

out vec4 color;

void main(){
	color.xyz = vec3(0.8,0.8,0.8);
	color.a = 1.0;
}