// Implements a simple shader for UI rendering.

#version 330 core

#shader_type vertex

uniform vec2 u_screen_size;

layout(location = 0) in vec2 a_position;
layout(location = 1) in vec2 a_uv;
layout(location = 2) in vec4 a_color;

out vec2 v_uv;
out vec4 v_color;

void main() {
  v_uv    = a_uv;
  v_color = a_color;

  gl_Position = vec4(
    2.0 * a_position.x / u_screen_size.x - 1.0,
    1.0 - 2.0 * a_position.y / u_screen_size.y,
    0.0,
    1.0
  );
}

#shader_type fragment

uniform sampler2D u_texture;

in vec2 v_uv;
in vec4 v_color;

void main() {
  gl_FragColor = texture(u_texture, v_uv) * v_color;
}
