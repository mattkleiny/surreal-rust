//! Image loading and management from various formats.

pub use image::ImageFormat;

use crate::assets::{Asset, AssetContext, AssetLoader};
use crate::io::VirtualPath;

use super::*;

/// An image of RGBA pixels, loadable from a variety of different formats.
pub struct Image {
  buffer: image::RgbaImage,
}

impl Image {
  /// Creates a new empty image with the given dimensions.
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      buffer: image::RgbaImage::new(width as u32, height as u32),
    }
  }

  /// Attempts to load an image from the given path.
  pub fn from_path(path: impl Into<VirtualPath>, format: Option<ImageFormat>) -> crate::Result<Self> {
    let stream = path.into().open_input_stream()?;
    let image = Self::from_bytes(stream, format)?;

    Ok(image)
  }

  /// Attempts to load an image from the given reader.
  pub fn from_bytes(reader: impl std::io::BufRead + std::io::Seek, format: Option<ImageFormat>) -> crate::Result<Self> {
    let mut reader = image::io::Reader::new(reader);

    if let Some(format) = format {
      reader.set_format(format);
    } else {
      reader = reader.with_guessed_format()?;
    }

    let image = reader.decode()?;
    let buffer = image.to_rgba8();

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

  /// Retrieves the pixels of the image as a slice of [`Color32`]s.
  pub fn as_slice(&self) -> &[Color32] {
    let rgba = &self.buffer;

    unsafe { std::slice::from_raw_parts(rgba.as_ptr() as *const Color32, rgba.len() / 4) }
  }

  /// Retrieves the pixels of the image as a mutable slice of [`Color32`]s.
  pub fn as_slice_mut(&mut self) -> &mut [Color32] {
    let rgba = &mut self.buffer;

    unsafe { std::slice::from_raw_parts_mut(rgba.as_ptr() as *mut Color32, rgba.len() / 4) }
  }

  /// Saves the image to the given path.
  pub fn save_to(&self, path: impl Into<VirtualPath>, format: ImageFormat) -> crate::Result<()> {
    let mut stream = path.into().open_output_stream()?;

    self.buffer.write_to(&mut stream, format)?;

    Ok(())
  }
}

/// An [`AssetLoader`] for images.
#[derive(Default)]
pub struct ImageLoader {
  pub format: Option<ImageFormat>,
}

impl Asset for Image {
  type Loader = ImageLoader;
}

impl AssetLoader<Image> for ImageLoader {
  fn load(&self, context: &AssetContext) -> crate::Result<Image> {
    Image::from_path(&context.path, self.format)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn image_should_load_from_path() {
    let image = Image::from_path("local://../surreal.ico", Some(ImageFormat::Ico)).expect("Failed to load image");

    assert_eq!(image.width(), 32);
    assert_eq!(image.height(), 32);

    let colors = image.as_slice();

    assert_eq!(colors.len(), 32 * 32);
  }
}
