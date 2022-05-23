pub use image::ImageFormat as ImageFormat;

use crate::io::{AsVirtualPath, FileResult};

use super::*;

/// An image of RGBA pixels, loadable from a variety of different formats.
pub struct Image {
  buffer: image::Rgba32FImage,
}

impl Image {
  /// Creates a new empty image with the given dimensions.
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      buffer: image::Rgba32FImage::new(width as u32, height as u32),
    }
  }

  /// Attempts to load an image from the given path with a discovered format.
  pub fn load(path: impl AsVirtualPath) -> FileResult<Self> {
    let stream = path.as_virtual_path().open_input_stream()?;
    let reader = image::io::Reader::new(stream).with_guessed_format()?;

    let image = reader.decode()?;
    let buffer = image.to_rgba32f();

    Ok(Self { buffer })
  }

  /// Attempts to load an image from the given path with the given format.
  pub fn load_with_format(path: impl AsVirtualPath, format: ImageFormat) -> FileResult<Self> {
    let stream = path.as_virtual_path().open_input_stream()?;
    let mut reader = image::io::Reader::new(stream);

    reader.set_format(format);

    let image = reader.decode()?;
    let buffer = image.to_rgba32f();

    Ok(Self { buffer })
  }

  /// Returns the width of the image.
  pub fn width(&self) -> usize {
    self.buffer.width() as usize
  }

  /// Returns the height of the image.
  pub fn height(&self) -> usize {
    self.buffer.height() as usize
  }

  /// Retrieves the pixels of the image as a slice of [`Color`]s.
  pub fn as_slice(&self) -> &[Color] {
    let rgba = self.buffer.as_ref();

    unsafe {
      std::slice::from_raw_parts(rgba.as_ptr() as *const Color, rgba.len() / 4)
    }
  }

  /// Retrieves the pixels of the image as a mutable slice of [`Color`]s.
  pub fn as_slice_mut(&mut self) -> &mut [Color] {
    let rgba = self.buffer.as_mut();

    unsafe {
      std::slice::from_raw_parts_mut(rgba.as_ptr() as *mut Color, rgba.len() / 4)
    }
  }

  /// Saves the image to the given path.
  pub fn save_to(&self, path: impl AsVirtualPath, format: ImageFormat) -> FileResult<()> {
    let mut stream = path.as_virtual_path().open_output_stream()?;

    self.buffer.write_to(&mut stream, format)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn image_should_load_from_path() {
    let image = Image::load_with_format("local://surreal.ico", ImageFormat::Ico).expect("Failed to load image");

    assert_eq!(image.width(), 32);
    assert_eq!(image.height(), 32);

    let colors = image.as_slice();

    assert_eq!(colors.len(), 32 * 32);
  }
}