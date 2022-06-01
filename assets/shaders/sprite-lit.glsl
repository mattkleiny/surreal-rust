// Implements a light-aware shader for sprites.

#version 330 core

#shader_type vertex

uniform mat4 u_projectionView = mat4(1.0);
uniform vec4 u_color = vec4(1.0);

layout(location = 0) in vec2  a_position;
layout(location = 1) in vec4  a_color;
layout(location = 2) in float a_emission;

out vec4 v_color;

void main() {
  v_color = a_color * u_color * a_emission;

  gl_Position = vec4(a_position, 0.0, 1.0) * u_projectionView;
}

#shader_type fragment

in vec4 v_color;

void main() {
  gl_FragColor = v_color;
}