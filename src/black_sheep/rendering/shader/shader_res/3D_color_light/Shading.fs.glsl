#version 450

uniform vec3 col;
uniform vec3 light_position;
uniform float light_power;

in vec4 world_pos;
in vec4 world_nor;

out vec4 out_color;

void main(){

	vec3 dir_ = light_position-world_pos.xyz;
	float dist = length(dir_);
	vec3 dir = dir_/dist;
	float f = dot(dir,world_nor.xyz) *light_power * (1/pow(dist,2));

	out_color = vec4(col * f,1.0);
}