#version 450

layout(location = 0) in vec3 squareVertices;
layout(location = 1) in vec2 uvVertices;

out vec2 UV;

uniform vec3 CameraRight_worldspace;
uniform vec3 CameraUp_worldspace;
uniform mat4 VP; 
uniform vec3 BillboardPos; 
uniform vec2 BillboardSize; 

void main()
{
	vec3 particleCenter_wordspace = BillboardPos;
	
	vec3 vertexPosition_worldspace = 
		particleCenter_wordspace
		+ CameraRight_worldspace * squareVertices.x * BillboardSize.x
		+ CameraUp_worldspace * squareVertices.y * BillboardSize.y;

	gl_Position = VP * vec4(vertexPosition_worldspace, 1.0f);

	UV = uvVertices;
}
