#version 450

out vec4 out_color;

in vec4 screen_pos;
flat in vec4 center_pos;

uniform float radius;

void main() {

  vec2 diff = center_pos.xy - screen_pos.xy;

  float dist = pow(diff.x,2)+pow(diff.y,2);

  if (dist > pow(radius,2)) {
    out_color = vec4(0.0,0.0,0.0,0.0);
  }else{
    out_color = vec4(1.0,0.0,0.0,1.0);
  }
};
