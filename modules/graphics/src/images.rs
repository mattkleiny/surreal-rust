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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_image_should_work() {
    let image = RgbaImage::new(16, 16);

    test(&image);
  }

  fn test(image: &impl Image<Pixel = Color32>) {
    let width = image.width();
    let height = image.height();
    let pixels = image.pixels();

    assert_eq!(width, 16);
    assert_eq!(height, 16);
    assert_eq!(pixels.len(), 16 * 16);
  }
}
