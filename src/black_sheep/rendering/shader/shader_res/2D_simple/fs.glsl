#version 450

out vec4 out_color;

in VS_OUT {
    vec3 color;
    float radius;
} fs_in;

void main() {
  out_color = vec4(fs_in.color,1.0);
};
