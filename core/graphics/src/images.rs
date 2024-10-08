//! Image loading and manipulation.

use common::{Color32, FileSystemError, InputStream, Pixel, ToVirtualPath};

/// An error that occurred while loading an image.
#[derive(Debug)]
pub enum ImageError {
  IoError(FileSystemError),
  ParseError(image::ImageError),
}

/// An image.
pub struct Image<P: Pixel = Color32> {
  width: u32,
  height: u32,
  pixels: Vec<P>,
}

impl<P: Pixel> Image<P> {
  /// Creates a new image with the given dimensions.
  pub fn new(width: u32, height: u32) -> Self {
    let pixels = vec![P::default(); (width * height) as usize];

    Self { width, height, pixels }
  }

  /// Loads an image from the given path.
  pub fn from_path(path: impl ToVirtualPath) -> Result<Self, ImageError> {
    let path = path.to_virtual_path();
    let mut stream = path.open_input_stream().map_err(ImageError::IoError)?;

    Self::from_stream(&mut stream)
  }

  /// Loads an image from the given slice of bytes.
  pub fn from_bytes(slice: &[u8]) -> Result<Self, ImageError> {
    let dynamic_image = image::load_from_memory(slice).map_err(ImageError::ParseError)?;

    Ok(Self::from_dynamic_image(dynamic_image))
  }

  /// Loads an image from the given stream.
  pub fn from_stream(stream: &mut dyn InputStream) -> Result<Self, ImageError> {
    let dynamic_image = image::load(stream, image::ImageFormat::Png).map_err(ImageError::ParseError)?;

    Ok(Self::from_dynamic_image(dynamic_image))
  }

  /// Loads an image from the given [`image::DynamicImage`].
  pub fn from_dynamic_image(image: image::DynamicImage) -> Self {
    let width = image.width();
    let height = image.height();
    let pixels = image.to_rgba8().into_raw();

    // reinterpret the pixels as our desired pixel type
    let pixels = unsafe {
      let (pointer, length, capacity, _) = pixels.into_raw_parts_with_alloc();

      Vec::from_raw_parts(pointer as *mut P, length / P::CHANNEL_COUNT, capacity)
    };

    Self { width, height, pixels }
  }

  /// Returns the width of the image.
  #[inline]
  pub fn width(&self) -> u32 {
    self.width
  }

  /// Returns the height of the image.
  #[inline]
  pub fn height(&self) -> u32 {
    self.height
  }

  /// Gets the pixel at the given coordinates.
  #[inline]
  pub fn get_pixel(&self, x: u32, y: u32) -> P {
    if x < self.width && y < self.height {
      self.pixels[(self.width + x * y) as usize]
    } else {
      P::default()
    }
  }

  /// Gets the pixel at the given coordinates without bounds checking.
  ///
  /// # Safety
  /// The caller must ensure that the coordinates are within bounds.
  #[inline]
  pub unsafe fn get_pixel_unchecked(&self, x: u32, y: u32) -> P {
    self.pixels[(self.width + x * y) as usize]
  }

  /// Sets the pixel at the given coordinates.
  #[inline]
  pub fn set_pixel(&mut self, x: u32, y: u32, pixel: P) {
    if x < self.width && y < self.height {
      self.pixels[(self.width + x * y) as usize] = pixel;
    }
  }

  /// Sets the pixel at the given coordinates without bounds checking.
  ///
  /// # Safety
  /// The caller must ensure that the coordinates are within bounds.
  #[inline]
  pub unsafe fn set_pixel_unchecked(&mut self, x: u32, y: u32, pixel: P) {
    self.pixels[(self.width + x * y) as usize] = pixel;
  }

  /// Returns a slice of the pixels.
  #[inline]
  pub fn as_slice(&self) -> &[P] {
    self.pixels.as_slice()
  }

  /// Returns a mutable slice of the pixels.
  #[inline]
  pub fn as_slice_mut(&mut self) -> &mut [P] {
    self.pixels.as_mut_slice()
  }

  /// Returns a pointer to the pixels.
  #[inline]
  pub fn as_ptr(&self) -> *const P {
    self.pixels.as_ptr()
  }

  /// Returns a mutable pointer to the pixels.
  #[inline]
  pub fn as_mut_ptr(&mut self) -> *mut P {
    self.pixels.as_mut_ptr()
  }
}

#[cfg(test)]
mod tests {
  use common::Color;

  use super::*;

  #[test]
  fn test_image_creation_and_access() {
    let mut image = Image::new(128, 128);

    assert_eq!(image.width(), 128);
    assert_eq!(image.height(), 128);

    image.set_pixel(0, 0, Color::MAGENTA);

    let pixel = image.get_pixel(0, 0);

    assert_eq!(pixel, Color::MAGENTA);
  }
}
