#version 450


uniform float tt = -0.18;
uniform float aa = 0.55;

vec3 project(vec3 v) {
    // float fx = 565.0;
    // float fy = 565.0;
    // float cx = 635.0;
    // float cy = 521.0;
    // float tt = -0.18;
    // float aa = 0.55;

    float d1 = length(v);
    float d2 = length(vec3(v.x, v.y, (tt * d1) + v.z));

    float div = (aa * d2) + ((1.0 - aa) * ((tt * d1) + v.z));

    float z = 0.0;

    if(v.z < 0.0){
        z =  (d1 - 20.0)/1000.0;
    }else{
        z = 1.0;
    }
    
    return vec3(vec2((v.x / div), (v.y / div)) * 0.4,z) ; //+ vec2(cx, cy)
}

in layout (location = 0) vec3 position;
in layout (location = 1) vec3 normal;

uniform mat4 M;


out vec4 world_pos;
out vec4 world_nor;

void main(){
	world_pos = M * vec4(position,1);
	world_nor = M * vec4(normal,0);
	gl_Position =  vec4( project( (M * vec4(position,1)).xyz), 1.0);
}