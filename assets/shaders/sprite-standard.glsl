#version 330 core

#shader_type vertex

uniform mat4 u_projectionView = mat4(1.0);
uniform vec4 u_color = vec4(1.0);

layout(location = 0) in vec2 a_position;
layout(location = 1) in vec2 a_uv;
layout(location = 2) in vec4 a_color;

out vec2 v_uv;
out vec4 v_color;

void main() {
  v_uv    = a_uv;
  v_color = a_color * u_color;

  gl_Position = vec4(a_position, 0.0, 1.0) * u_projectionView;
}

#shader_type fragment

uniform sampler2D u_texture;

in vec2 v_uv;
in vec4 v_color;

void main() {
  gl_FragColor = texture(u_texture, v_uv) * v_color;
}
