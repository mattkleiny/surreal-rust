//! Image loading and management.

use crate::assets::{AssetResult, Loadable};
use crate::graphics::Color;
use crate::io::VirtualPath;

/// An image of pixels, uncompressed, in RGBA format.
///
/// An image can be loaded from disc and dynamically manipulated.
pub struct Image {
  width: usize,
  height: usize,
  pixels: Vec<Color>,
}

impl Image {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      width,
      height,
      pixels: vec![Color::CLEAR; width * height],
    }
  }

  pub fn width(&self) -> usize { self.width }
  pub fn height(&self) -> usize { self.height }

  pub fn as_slice(&self) -> &[Color] { self.pixels.as_slice() }
  pub fn as_mut_slice(&mut self) -> &mut [Color] { self.pixels.as_mut_slice() }
}

impl Loadable for Image {
  fn load(path: VirtualPath) -> AssetResult<Self> {
    // TODO: switch on the file extension

    unimplemented!()
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