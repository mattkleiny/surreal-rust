// This is a simple shader that enables skeletal animation.

#version 330

#shader_type vertex

uniform mat4 u_modelViewProjectionMatrix;
uniform mat4[] u_boneMatrices;

attribute vec3 a_position;
attribute vec3 a_normal;
attribute vec2 a_texcoord0;
attribute uvec4 a_boneIndices;
attribute vec4 a_boneWeights;

varying vec2 v_texcoord0;
varying vec3 v_normal;
varying vec3 v_position;

void main() {
  // blend the bone matrices
  mat4 boneMatrix = u_boneMatrices[a_boneIndices.x] * a_boneWeights.x +
    u_boneMatrices[a_boneIndices.y] * a_boneWeights.y +
    u_boneMatrices[a_boneIndices.z] * a_boneWeights.z +
    u_boneMatrices[a_boneIndices.w] * a_boneWeights.w;

  vec4 position = boneMatrix * vec4(a_position, 1.0);
  vec4 normal = boneMatrix * vec4(a_normal, 0.0);

  v_texcoord0 = a_texcoord0;
  v_normal = normal.xyz;

  gl_Position = u_modelViewProjectionMatrix * position;
}

#shader_type fragment

uniform sampler2D u_texture;

void main() {
  gl_FragColor = texture2D(u_texture, v_texcoord0);
}
