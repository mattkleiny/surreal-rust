// Implements a sprite shader with per-vertex tint.

#shader_type vertex

uniform mat4 u_projection_view;

layout(location = 0) in vec2 a_position;
layout(location = 1) in vec2 a_texcoord_0;
layout(location = 2) in vec4 a_color;

out vec2 v_texcoord_0;
out vec4 v_color;

void main() {
  v_texcoord_0 = a_texcoord_0;
  v_color = a_color;

  gl_Position = u_projection_view * vec4(a_position, 0.0, 1.0);
}

#shader_type fragment

uniform sampler2D u_texture;

in vec2 v_texcoord_0;
in vec4 v_color;

out vec4 frag_color;

void main() {
  frag_color = texture(u_texture, v_texcoord_0) * v_color;
}
