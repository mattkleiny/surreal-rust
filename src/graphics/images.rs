use crate::assets::{AssetContext, AssetResult, LoadableAsset};
use crate::graphics::Color;
use crate::io::Path;
use crate::maths::DenseGrid;

/// An image is essentially a 2d-grid of `Color`s, uncompressed.
pub struct Image {
  pixels: DenseGrid<Color>,
}

impl Image {
  pub fn new(width: usize, height: usize, default_color: Color) -> Self {
    Self {
      pixels: DenseGrid::new(width, height, default_color)
    }
  }

  #[inline]
  pub fn width(&self) -> usize {
    self.pixels.width()
  }

  #[inline]
  pub fn height(&self) -> usize {
    self.pixels.height()
  }

  #[inline]
  pub fn as_slice(&self) -> &[Color] {
    self.pixels.as_slice()
  }

  #[inline]
  pub fn as_mut_slice(&mut self) -> &mut [Color] {
    self.pixels.as_mut_slice()
  }
}

impl LoadableAsset for Image {
  fn load(path: Path, context: &mut impl AssetContext) -> AssetResult<Self> {
    unimplemented!()
  }
}
