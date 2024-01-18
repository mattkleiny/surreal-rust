//! Image loading and manipulation.

use image::{Rgba32FImage, RgbaImage};

use super::*;

/// Represents an image of pixels.
pub trait Image {
  type Pixel;

  fn width(&self) -> u32;
  fn height(&self) -> u32;
  fn pixels(&self) -> &[Self::Pixel];
}

/// A integral point 32-bit image.
impl Image for RgbaImage {
  type Pixel = Color32;

  fn width(&self) -> u32 {
    self.width()
  }

  fn height(&self) -> u32 {
    self.height()
  }

  fn pixels(&self) -> &[Self::Pixel] {
    todo!()
  }
}

/// A floating point 32-bit image.
impl Image for Rgba32FImage {
  type Pixel = Color;

  fn width(&self) -> u32 {
    self.width()
  }

  fn height(&self) -> u32 {
    self.height()
  }

  fn pixels(&self) -> &[Self::Pixel] {
    todo!()
  }
}
