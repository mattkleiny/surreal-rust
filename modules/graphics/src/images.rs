//! Image loading and manipulation.

pub use image::{GrayAlphaImage, GrayImage, Luma, Rgb, RgbImage, Rgba, Rgba32FImage, RgbaImage};

/// Represents an image of pixels.
pub trait Image {
  /// The pixel type of the image.
  type Pixel;

  // image queries
  fn width(&self) -> u32;
  fn height(&self) -> u32;

  // pixel access
  fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel;
  fn set_pixel(&mut self, x: u32, y: u32, pixel: Self::Pixel);

  // slice access
  fn as_slice(&self) -> &[Self::Pixel];
  fn as_slice_mut(&mut self) -> &mut [Self::Pixel];
}

/// Implements [`Image`] for the given image type.
macro_rules! impl_image {
  ($type:ty, $pixel:ty) => {
    impl Image for $type {
      type Pixel = $pixel;

      fn width(&self) -> u32 {
        self.width()
      }

      fn height(&self) -> u32 {
        self.height()
      }

      fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        self.get_pixel(x, y).clone()
      }

      fn set_pixel(&mut self, x: u32, y: u32, pixel: Self::Pixel) {
        self.put_pixel(x, y, pixel)
      }

      fn as_slice(&self) -> &[Self::Pixel] {
        todo!()
      }

      fn as_slice_mut(&mut self) -> &mut [Self::Pixel] {
        todo!()
      }
    }
  };
}

impl_image!(image::RgbImage, image::Rgb<u8>);
impl_image!(image::RgbaImage, image::Rgba<u8>);
impl_image!(image::Rgba32FImage, image::Rgba<f32>);
impl_image!(image::GrayImage, image::Luma<u8>);
impl_image!(image::GrayAlphaImage, image::LumaA<u8>);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_image_creation_and_accessor() {
    let mut image = image::RgbaImage::new(128, 128);

    assert_eq!(image.width(), 128);
    assert_eq!(image.height(), 128);

    image.set_pixel(0, 0, image::Rgba([255, 0, 255, 255]));

    let pixel = image.get_pixel(0, 0);

    assert_eq!(*pixel, image::Rgba([255, 0, 255, 255]));
  }
}
