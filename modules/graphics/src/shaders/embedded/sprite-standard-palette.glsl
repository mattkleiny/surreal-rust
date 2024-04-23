// Implements a sprite shader with per-vertex tint.

#version 330 core

#shader_type vertex

uniform mat4 u_projection_view;

layout(location = 0) in vec2 a_position;
layout(location = 1) in vec2 a_texcoord_0;
layout(location = 2) in vec4 a_color;

varying vec2 v_texcoord_0;
varying vec4 v_color;

void main() {
  v_texcoord_0 = a_texcoord_0;
  v_color = a_color;

  gl_Position = u_projection_view * vec4(a_position, 0.0, 1.0);
}

#shader_type fragment

uniform sampler2D u_texture;
uniform sampler2D u_palette_tex;
uniform uint u_palette_width;

vec4 sample_palette(vec4 color) {
  uint index = uint(color.r * 255.0) + uint(color.g * 255.0) * 256u;
  vec2 uv = vec2(index / float(u_palette_width), 0.5);

  return texture(u_palette_tex, uv);
}

void main() {
  vec4 base_color = texture(u_texture, v_texcoord_0);
  vec4 palette_color = sample_palette(base_color);

  gl_FragColor = palette_color * v_color;
}
