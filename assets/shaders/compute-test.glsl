// A sample compute shader for testing purposes.

#version 430 core

#shader_type compute

layout(local_size_x = 1, local_size_y = 1)in;
layout(rgba8, binding = 0)writeonly uniform image2D u_image;

uniform vec2 u_resolution;

void main() {
  ivec2 coords = ivec2(gl_GlobalInvocationID.xy);
  vec2 uv = vec2(gl_GlobalInvocationID.xy) / u_resolution;
  vec4 color = vec4(uv.x, uv.y, 1, 1);
  
  imageStore(u_image, coords, color);
}