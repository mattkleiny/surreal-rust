// Implements a simple shader for wire geometry.

#version 330 core

#shader_type vertex

uniform mat4 u_projectionView = mat4(1.0);
uniform vec4 u_color = vec4(1.0);

layout (location = 0) in vec3 a_position;
layout (location = 1) in vec4 a_color;

out vec4 v_color;

void main() {
  v_color = a_color * u_color;

  gl_Position = vec4(a_position, 1.0) * u_projectionView;
}

#shader_type fragment

in vec4 v_color;

void main() {
  gl_FragColor = v_color;
}

