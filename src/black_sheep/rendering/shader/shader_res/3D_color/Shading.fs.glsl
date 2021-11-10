#version 450

in vec3 cl;

out vec4 out_color;

void main(){
	out_color = vec4(cl,1.0);
}