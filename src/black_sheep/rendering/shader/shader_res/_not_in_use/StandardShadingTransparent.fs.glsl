#version 460

// Interpolated values from the vertex shaders
in vec2 UV;

// Ouput data
out vec4 color;

// Values that stay constant for the whole mesh.
uniform sampler2D myTextureSampler;


void main(){
	color  =  texture( myTextureSampler, UV ).rgba;
	// vec3 MaterialDiffuseColor =  texture( myTextureSampler, UV ).rgb;
	// color = vec4(MaterialDiffuseColor,1.0);
}