//! Image loading and manipulation.

use common::FileSystemError;

use super::*;

// common image types
pub type ColorImage = image::ImageBuffer<Color, Vec<f32>>;
pub type Color32Image = image::ImageBuffer<Color32, Vec<u8>>;

/// An error that occurred while loading an image.
#[derive(Debug)]
pub enum ImageError {
  IoError(FileSystemError),
  ImageError(image::ImageError),
}

/// A simplified representation of an image of pixels.
pub trait Image {
  /// The pixel type of the image.
  type Pixel: Pixel;

  /// Loads an image from the path.
  fn from_path(path: impl common::ToVirtualPath) -> Result<Self, ImageError>
  where
    Self: Sized + FromDynamicImage,
  {
    let path = path.to_virtual_path();
    let stream = path.open_input_stream().map_err(|error| ImageError::IoError(error))?;

    Self::from_stream(stream)
  }

  /// Loads an image from the given stream.
  fn from_stream(stream: impl common::InputStream) -> Result<Self, ImageError>
  where
    Self: Sized + FromDynamicImage,
  {
    Ok(Self::from_dynamic_image(
      image::load(stream, image::ImageFormat::Png).map_err(|it| ImageError::ImageError(it))?,
    ))
  }

  /// Loads an image from the given slice of bytes.
  fn from_slice(slice: &[u8]) -> Result<Self, ImageError>
  where
    Self: Sized + FromDynamicImage,
  {
    Ok(Self::from_dynamic_image(
      image::load_from_memory(slice).map_err(|it| ImageError::ImageError(it))?,
    ))
  }

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

/// Implements [`Image`] for the all image type and pixel type combinations.
impl<P: image::Pixel + Pixel> Image for image::ImageBuffer<P, Vec<<P as image::Pixel>::Subpixel>> {
  type Pixel = P;

  fn width(&self) -> u32 {
    self.width()
  }

  fn height(&self) -> u32 {
    self.height()
  }

  fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
    self.get_pixel(x, y).to_owned()
  }

  fn set_pixel(&mut self, x: u32, y: u32, pixel: Self::Pixel) {
    self.put_pixel(x, y, pixel)
  }

  fn as_slice(&self) -> &[Self::Pixel] {
    unsafe { std::slice::from_raw_parts(self.as_ptr() as *const Self::Pixel, self.len() / 4) }
  }

  fn as_slice_mut(&mut self) -> &mut [Self::Pixel] {
    unsafe { std::slice::from_raw_parts_mut(self.as_ptr() as *mut Self::Pixel, self.len() / 4) }
  }
}

/// Implements [`image::Pixel`] for the given pixel type.
macro_rules! impl_pixel {
  ($type:ty, $model:expr) => {
    impl image::Pixel for $type {
      type Subpixel = <$type as crate::Pixel>::Subpixel;

      const CHANNEL_COUNT: u8 = <$type as crate::Pixel>::CHANNEL_COUNT as u8;
      const COLOR_MODEL: &'static str = $model;

      fn channels(&self) -> &[Self::Subpixel] {
        unsafe { &*(self as *const Self as *const [Self::Subpixel; 4]) }
      }

      fn channels_mut(&mut self) -> &mut [Self::Subpixel] {
        unsafe { &mut *(self as *mut Self as *mut [Self::Subpixel; 4]) }
      }

      fn channels4(&self) -> (Self::Subpixel, Self::Subpixel, Self::Subpixel, Self::Subpixel) {
        (self.r, self.g, self.b, self.a)
      }

      fn from_channels(a: Self::Subpixel, b: Self::Subpixel, c: Self::Subpixel, d: Self::Subpixel) -> Self {
        Self::rgba(a, b, c, d)
      }

      fn from_slice(slice: &[Self::Subpixel]) -> &Self {
        unsafe { &*(slice.as_ptr() as *const Self) }
      }

      fn from_slice_mut(slice: &mut [Self::Subpixel]) -> &mut Self {
        unsafe { &mut *(slice.as_mut_ptr() as *mut Self) }
      }

      fn to_rgb(&self) -> image::Rgb<Self::Subpixel> {
        image::Rgb([self.r, self.g, self.b])
      }

      fn to_rgba(&self) -> image::Rgba<Self::Subpixel> {
        image::Rgba([self.r, self.g, self.b, self.a])
      }

      fn to_luma(&self) -> image::Luma<Self::Subpixel> {
        image::Luma([self.r])
      }

      fn to_luma_alpha(&self) -> image::LumaA<Self::Subpixel> {
        image::LumaA([self.r, self.a])
      }

      fn map<F>(&self, mut f: F) -> Self
      where
        F: FnMut(Self::Subpixel) -> Self::Subpixel,
      {
        Self::rgba(f(self.r), f(self.g), f(self.b), f(self.a))
      }

      fn apply<F>(&mut self, mut f: F)
      where
        F: FnMut(Self::Subpixel) -> Self::Subpixel,
      {
        self.r = f(self.r);
        self.g = f(self.g);
        self.b = f(self.b);
        self.a = f(self.a);
      }

      fn map_with_alpha<F, G>(&self, mut f: F, mut g: G) -> Self
      where
        F: FnMut(Self::Subpixel) -> Self::Subpixel,
        G: FnMut(Self::Subpixel) -> Self::Subpixel,
      {
        Self::rgba(f(self.r), f(self.g), f(self.b), g(self.a))
      }

      fn apply_with_alpha<F, G>(&mut self, mut f: F, mut g: G)
      where
        F: FnMut(Self::Subpixel) -> Self::Subpixel,
        G: FnMut(Self::Subpixel) -> Self::Subpixel,
      {
        self.r = f(self.r);
        self.g = f(self.g);
        self.b = f(self.b);
        self.a = g(self.a);
      }

      fn map2<F>(&self, other: &Self, mut f: F) -> Self
      where
        F: FnMut(Self::Subpixel, Self::Subpixel) -> Self::Subpixel,
      {
        Self::rgba(
          f(self.r, other.r),
          f(self.g, other.g),
          f(self.b, other.b),
          f(self.a, other.a),
        )
      }

      fn apply2<F>(&mut self, other: &Self, mut f: F)
      where
        F: FnMut(Self::Subpixel, Self::Subpixel) -> Self::Subpixel,
      {
        self.r = f(self.r, other.r);
        self.g = f(self.g, other.g);
        self.b = f(self.b, other.b);
        self.a = f(self.a, other.a);
      }

      fn invert(&mut self) {
        self.r = Self::Subpixel::MAX - self.r;
        self.g = Self::Subpixel::MAX - self.g;
        self.b = Self::Subpixel::MAX - self.b;
        self.a = Self::Subpixel::MAX - self.a;
      }

      fn blend(&mut self, other: &Self) {
        self.r = self.r + other.r - self.r * other.r;
        self.g = self.g + other.g - self.g * other.g;
        self.b = self.b + other.b - self.b * other.b;
        self.a = self.a + other.a - self.a * other.a;
      }
    }
  };
}

impl_pixel!(Color, "RGBA");
impl_pixel!(Color32, "RGBA");

/// Converts the given [`image::DynamicImage`] into the given image type.
pub trait FromDynamicImage {
  fn from_dynamic_image(image: image::DynamicImage) -> Self;
}

impl FromDynamicImage for ColorImage {
  #[inline]
  fn from_dynamic_image(image: image::DynamicImage) -> Self {
    unsafe { std::mem::transmute(image.to_rgba32f()) }
  }
}

impl FromDynamicImage for Color32Image {
  #[inline]
  fn from_dynamic_image(image: image::DynamicImage) -> Self {
    unsafe { std::mem::transmute(image.to_rgba8()) }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_image_creation_and_access() {
    let mut image = ColorImage::new(128, 128);

    assert_eq!(image.width(), 128);
    assert_eq!(image.height(), 128);

    image.set_pixel(0, 0, Color::MAGENTA);

    let pixel = image.get_pixel(0, 0);

    assert_eq!(*pixel, Color::MAGENTA);
  }
}
