// Implements a simple shader for palette shifted sprites.

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

uniform sampler2D u_palette; // The combined palette texture.
uniform int u_paletteWidth; // The width of each palette in the palette texture.

// Performs a simple palette-shifting effect on the given color.
vec4 sample_palette(vec4 color, int channel) {
  float normalized_y = channel * 0.25 - 0.125;
  
  // sample the palette texture, discretizing source colors into N equal
  // sections across each of the palettes.
  //
  // the channel parameter allows for more than 1 palette per texture,
  // aligned vertically. each channel is expected to be 1 texel high.
  float index = ceil(color.r * (u_paletteWidth - 1)) / u_paletteWidth;
  vec4 final = texture(u_palette, vec2(index, normalized_y));
  
  final.a = color.a;
  final.rgb *= color.a;
  
  return final;
}

void main() {
  vec4 main_color = texture(u_texture, v_uv);
  vec4 final_color = sample_palette(main_color, 1);

  gl_FragColor = final_color * v_color;
}
