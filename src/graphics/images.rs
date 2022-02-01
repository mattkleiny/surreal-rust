use crate::graphics::Color;

/// An image of pixels, uncompressed, in RGBA format.
pub struct Image {
  width: usize,
  height: usize,
  pixels: Vec<Color>,
}

impl Image {
  /// Constructs a new image with the given width and height.
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      width,
      height,
      pixels: vec![Color::CLEAR; width * height],
    }
  }

  /// The width of the image, in pixels.
  #[inline(always)]
  pub fn width(&self) -> usize {
    self.width
  }

  /// The height of the image, in pixels.
  #[inline(always)]
  pub fn height(&self) -> usize {
    self.height
  }

  /// Retrieves a slice of the image's pixels.
  pub fn as_slice(&self) -> &[Color] {
    self.pixels.as_slice()
  }

  /// Retrieves a mutable slice of the image's pixels.
  pub fn as_mut_slice(&mut self) -> &mut [Color] {
    self.pixels.as_mut_slice()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_write_and_mutate_image_contents() {
    let mut image = Image::new(256, 256);
    let pixels = image.as_mut_slice();

    for pixel in pixels.iter_mut() {
      *pixel = Color::WHITE;
    }
  }
}