// Implements a sprite shader that supports multiple texture per batch.

#version 330 core

#shader_type vertex

uniform mat4 u_projection_view;

layout(location = 0) in vec2 a_position;
layout(location = 1) in vec2 a_texcoord_0;
layout(location = 2) in vec4 a_color;
layout(location = 3) in uint a_texture_id;

varying vec2 v_texcoord_0;
varying vec4 v_color;
varying uint v_texture_id;

void main() {
  v_texcoord_0 = a_texcoord_0;
  v_color = a_color;
  v_texture_id = a_texture_id;

  gl_Position = u_projection_view * vec4(a_position, 0.0, 1.0);
}

#shader_type fragment

#constant MAX_TEXTURES;

uniform sampler2D u_texture[MAX_TEXTURES];

void main() {
  gl_FragColor = texture(u_texture[v_texture_id], v_texcoord_0) * v_color;
}
