//! Image loading and manipulation.

use crate::Color32;

/// Represents an image of pixels.
pub trait Image<P> {
  /// Returns the width of the image.
  fn width(&self) -> u32;

  /// Returns the width of the image.
  fn height(&self) -> u32;

  /// Returns the pixels of the image.
  fn pixels(&self) -> &[P];
}

impl Image<Color32> for image::RgbaImage {
  fn width(&self) -> u32 {
    self.width()
  }

  fn height(&self) -> u32 {
    self.height()
  }

  fn pixels(&self) -> &[Color32] {
    todo!()
  }
}
