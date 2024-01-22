// Implements a simple shader for screen-space canvas rendering.

#version 330 core

#shader_type vertex

uniform vec2 u_viewport_size;

layout(location = 0) in vec2 a_position;
layout(location = 1) in vec2 a_texcoord_0;
layout(location = 2) in vec4 a_color;

varying vec2 v_texcoord_0;
varying vec4 v_color;

void main() {
  v_texcoord_0 = a_texcoord_0;
  v_color = a_color;

  gl_Position = vec4(2.0 * a_position.x / u_viewport_size.x - 1.0, 1.0 - 2.0 * a_position.y / u_viewport_size.y, 0.0, 1.0);
}

#shader_type fragment

uniform sampler2D u_texture;

void main() {
  gl_FragColor = texture(u_texture, v_texcoord_0) * v_color;
}
