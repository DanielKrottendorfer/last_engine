#version 450

in vec2 UV;
out vec4 out_color;

uniform sampler2D myTextureSampler;

void main(){
	out_color = texture( myTextureSampler, UV );
}