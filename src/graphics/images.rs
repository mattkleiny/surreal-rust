use image::RgbaImage;

use crate::assets::{AssetLoadContext, AssetLoader, AssetResult};
use crate::graphics::Color;
use crate::io::VirtualPath;

/// An image of RGBA pixels, loadable from a variety of different formats.
pub struct Image {
  /// Internally we back images with the `image` crate.
  buffer: RgbaImage,
}

impl Image {
  /// Creates a new empty image.
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      buffer: RgbaImage::new(width as u32, height as u32),
    }
  }

  /// Attempts to load an image from the given path.
  pub fn load(path: VirtualPath) -> AssetResult<Self> {
    todo!()
  }

  /// Returns the width of the image.
  pub fn width(&self) -> usize {
    self.buffer.width() as usize
  }

  /// Returns the height of the image.
  pub fn height(&self) -> usize {
    self.buffer.height() as usize
  }

  /// Gets the color of the pixel at the given coordinates.
  pub fn get_pixel(&self, x: usize, y: usize) -> Color {
    todo!()
  }

  /// Sets the color of the pixel at the given coordinates.
  pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
    todo!()
  }
}