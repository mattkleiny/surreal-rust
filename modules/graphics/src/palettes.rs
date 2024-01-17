//! Color palette loading and managements.
//!
//! Color palettes are so frequently used in projects that we've built-in
//! support for basic palette operations and slicing.
//!
//! JASC-PAL files can be loaded from disc, as well.

use core::io::VirtualPath;
use std::ops::Index;

use super::*;

/// A palette of colors of type [`P`].
#[derive(Default, Clone)]
pub struct ColorPalette<P> {
  colors: Vec<P>,
}

impl<P: Pixel> ColorPalette<P> {
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

  /// Loads a palette from the given file path.
  pub fn from_file<'a>(path: impl Into<VirtualPath<'a>>) -> core::Result<Self> {
    let path = path.into();
    let stream = path.open_input_stream()?;

    Self::from_bytes(stream)
  }

  /// Loads a palette from the given reader.
  pub fn from_bytes(reader: impl std::io::BufRead) -> core::Result<Self> {
    let lines: Vec<_> = reader.lines().collect::<Result<_, _>>()?;

    if lines[0] != "JASC-PAL" {
      return Err(core::anyhow!("Expected A JASC-PAL file format"));
    }

    if lines[1] != "0100" {
      return Err(core::anyhow!("Expected a 0100 magic header"));
    }

    // read palette size and start building palette
    let count: usize = lines[2].parse()?;
    let mut colors = vec![P::default(); count];

    for (index, color) in colors.iter_mut().enumerate() {
      let index = (3 + index) % lines.len();
      let components = lines[index].split(' ').collect::<Vec<_>>();

      if components.len() != 3 {
        return Err(core::anyhow!("Expected 3 color components on line {}", index + 1));
      }

      *color = P::from_bytes(&[
        components[0].parse()?,
        components[1].parse()?,
        components[2].parse()?,
        255,
      ]);
    }

    Ok(Self::from_vec(colors))
  }

  /// Is the palette empt?
  pub fn is_empty(&self) -> bool {
    self.colors.is_empty()
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

impl<P> Index<usize> for ColorPalette<P> {
  type Output = P;

  fn index(&self, index: usize) -> &Self::Output {
    &self.colors[index]
  }
}
