use std::path::Path;

use crate::assets::{AssetContext, LoadableAsset};
use crate::graphics::Color;
use crate::maths::DenseGrid;

/// An image is essentially a 2d-grid of `Color`s, uncompressed.
pub struct Image {
  pixels: DenseGrid<Color>,
}

impl Image {
  pub fn width(&self) -> usize {
    self.pixels.width()
  }
  pub fn height(&self) -> usize {
    self.pixels.height()
  }

  /// Accesses the pixels of the `Image`.
  pub fn pixels(&self) -> &[Color] {
    unimplemented!()
  }

  /// Mutably accesses the pixels of the `Image`.
  pub fn pixels_mut(&mut self) -> &mut [Color] {
    unimplemented!()
  }
}

// TODO: actually implement me

impl LoadableAsset for Image {
  fn load(path: &impl AsRef<Path>, context: &mut impl AssetContext) -> Self {
    Self {
      pixels: DenseGrid::new(16, 16, Color::BLACK),
    }
  }
}
