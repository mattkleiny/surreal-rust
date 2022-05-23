use std::io::BufRead;
use std::ops::Index;

use anyhow::anyhow;

use crate::io::{AsVirtualPath, FileResult};

use super::*;

/// A palette of colors of type [`P`].
pub struct ColorPalette<P> {
  colors: Vec<P>,
}

impl<P> ColorPalette<P> where P: Pixel {
  /// Creates a new empty palette.
  pub fn new() -> Self {
    Self { colors: Vec::new() }
  }

  /// Creates a palette from a list of colors.
  pub fn from_vec(colors: Vec<P>) -> Self {
    Self { colors }
  }

  /// Creates a color palette from the given slice of pixels.
  pub fn from_slice(slice: &[P]) -> Self {
    Self { colors: slice.to_vec() }
  }

  /// Loads a palette from the given JASC-PAL palette file.
  pub fn from_file(path: impl AsVirtualPath) -> FileResult<Self> {
    let path = path.as_virtual_path();
    let stream = path.open_input_stream()?;
    let lines: Vec<_> = stream.lines().collect::<Result<_, _>>()?;

    if lines[0] != "JASC-PAL" {
      return Err(anyhow!("Expected A JASC-PAL file format in file {:?}", path));
    }

    if lines[1] != "0100" {
      return Err(anyhow!("Expected a 0100 magic header in file {:?}", path));
    }

    // read palette size and start building palette
    let count: usize = lines[2].parse()?;
    let mut colors = vec![P::EMPTY; count];

    for i in 0..colors.len() {
      let index = (3 + i) % lines.len();
      let components = lines[index].split(' ').collect::<Vec<_>>();

      if components.len() != 3 {
        return Err(anyhow!("Expected 3 color components on line {}", index + 1));
      }

      colors[i] = P::from_bytes(&[
        components[0].parse()?,
        components[1].parse()?,
        components[2].parse()?,
        255,
      ]);
    }

    Ok(Self::from_vec(colors))
  }

  /// Gets the number of colors in this palette.
  pub fn len(&self) -> usize {
    self.colors.len()
  }

  /// Adds a color to the palette.
  pub fn push(&mut self, color: P) {
    self.colors.push(color);
  }

  /// Removes all colors from the palette.
  pub fn clear(&mut self) {
    self.colors.clear();
  }

  /// Returns the colors as a slice of pixels.
  pub fn as_slice(&self) -> &[P] {
    &self.colors
  }

  /// Returns the colors as a mutable slice of pixels.
  pub fn as_slice_mut(&mut self) -> &mut [P] {
    &mut self.colors
  }
}

impl<G> Texture<G> where G: GraphicsImpl {
  /// Writes the palette to the given texture at the given Y index.
  pub fn write_palette(&mut self, channel: usize, palette: &ColorPalette<Color>) {
    self.write_pixels(palette.len(), channel, palette.as_slice());
  }
}

impl<P> Index<usize> for ColorPalette<P> {
  type Output = P;

  fn index(&self, index: usize) -> &Self::Output {
    &self.colors[index]
  }
}