#ifndef SURREAL_PALETTES_H_
#define SURREAL_PALETTES_H_

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

#endif
