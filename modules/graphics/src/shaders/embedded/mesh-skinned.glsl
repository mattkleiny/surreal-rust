// This is a simple shader that enables skeletal animation.

#shader_type vertex

uniform mat4 u_model_view_projection;
uniform mat4[] u_bone_matrices;

attribute vec3 a_position;
attribute vec3 a_normal;
attribute vec2 a_texcoord0;
attribute uvec4 a_bone_indices;
attribute vec4 a_bone_weights;

varying vec2 v_texcoord0;
varying vec3 v_normal;

void main() {
  // blend the bone matrices
  mat4 boneMatrix = u_bone_matrices[a_bone_indices.x] * a_bone_weights.x +
  u_bone_matrices[a_bone_indices.y] * a_bone_weights.y +
  u_bone_matrices[a_bone_indices.z] * a_bone_weights.z +
  u_bone_matrices[a_bone_indices.w] * a_bone_weights.w;

  vec4 position = boneMatrix * vec4(a_position, 1.0);
  vec4 normal = boneMatrix * vec4(a_normal, 0.0);

  v_texcoord0 = a_texcoord0;
  v_normal = normal.xyz;

  gl_Position = u_model_view_projection * position;
}

#shader_type fragment

uniform sampler2D u_texture;

void main() {
  gl_FragColor = texture2D(u_texture, v_texcoord0);
}
