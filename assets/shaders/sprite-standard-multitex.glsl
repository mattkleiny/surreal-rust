// Implements a simple shader for sprites, with support for multiple textures per draw call.

#version 330 core

#shader_type vertex

uniform mat4 u_projectionView = mat4(1.0);
uniform vec4 u_color = vec4(1.0);

layout(location = 0)in vec2 a_position;
layout(location = 1)in vec2 a_uv;
layout(location = 2)in vec4 a_color;
layout(location = 3)in uint a_textureId;

out vec2 v_uv;
out vec4 v_color;
flat out uint v_textureId;

void main() {
  v_uv = a_uv;
  v_color = a_color * u_color;
  v_textureId = a_textureId;
  
  gl_Position = vec4(a_position, 0.0, 1.0) * u_projectionView;
}

#shader_type fragment

uniform sampler2D u_textures[16];

in vec2 v_uv;
in vec4 v_color;
flat in uint v_textureId;

void main() {
  // gl_FragColor = v_color * texture(u_textures[v_textureId], v_uv);
  gl_FragColor = vec4(v_textureId, v_uv.x, v_uv.y, 1) * v_color;
}
