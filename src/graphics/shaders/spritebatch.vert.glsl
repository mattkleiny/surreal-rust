#version 330

attribute vec4 a_position;
attribute vec4 a_color;
attribute vec2 a_texCoords;

uniform mat4 u_projView;

varying vec4 v_color;
varying vec2 v_texCoords;

void main() {
  v_color = a_color;
  v_texCoords = a_texCoords;

  // TODO: sprite instancing?
  // TODO: shared transforms?
  // TODO: shared colors (instead of per-vertex)?
  // TODO: normal mapping and projections?

  gl_Position = u_projView * a_position;
}