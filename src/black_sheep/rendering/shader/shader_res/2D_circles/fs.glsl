#version 450

out vec4 out_color;

in GS_OUT {
    vec2 screen_pos;
    
    flat vec2 center_pos;
    flat vec3 color;
    flat float radius;
} fs_in;

void main() {

  vec2 diff = fs_in.center_pos.xy - fs_in.screen_pos.xy;

  float dist = pow(diff.x,2)+pow(diff.y,2);

  if (dist > pow(fs_in.radius,2)) {
    //out_color = vec4(1.0,0.0,1.0,0.0);
  }else{
    out_color = vec4(fs_in.color,1.0);
  }
};
