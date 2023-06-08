#version 450

in vec2 TexCoord;

out vec4 out_color;

uniform sampler2D image1;

void main() { 
  	out_color = texture(image1,TexCoord);
};
