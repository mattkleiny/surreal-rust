use std::io::BufReader;

use image::{DynamicImage, ImageError, ImageFormat, RgbaImage};

use crate::assets::{AssetResult, Loadable};
use crate::graphics::Color;

/// An image of pixels, uncompressed, in RGBA format.
///
/// An image can be loaded from disc and dynamically manipulated.
pub struct Image {
  image: RgbaImage
}

impl Image {
  /// Constructs a new empty image, filled with (0,0,0,0) pixels.
  pub fn new(width: usize, height: usize) -> Self {
    let image = RgbaImage::new(width as u32, height as u32);

    Self::from_rgba(image)
  }

  /// Constructs an image from an existing `DynamicImage`, converting it into RGBA.
  pub fn from_dynamic(image: DynamicImage) -> Self {
    Self { image: image.into_rgba() }
  }

  /// Constructs an image from an existing `RgbaImage`.
  pub fn from_rgba(image: RgbaImage) -> Self {
    Self { image }
  }

  pub fn width(&self) -> usize { self.image.width() as usize }
  pub fn height(&self) -> usize { self.image.height() as usize }
}

/// Support de-referencing images to pixels/colors.
impl std::ops::Deref for Image {
  type Target = [Color];

  fn deref(&self) -> &Self::Target {
    bytemuck::cast_slice(&self.image)
  }
}

impl std::ops::DerefMut for Image {
  fn deref_mut(&mut self) -> &mut Self::Target {
    bytemuck::cast_slice_mut(&mut self.image)
  }
}

impl Loadable for Image {
  fn load(path: impl AsRef<str>) -> AssetResult<Self> {
    let path = path.as_ref();

    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);

    let format = ImageFormat::from_path(path)?;
    let image = image::load(reader, format)?;

    Ok(Self::from_dynamic(image))
  }
}

impl From<ImageError> for crate::assets::Error {
  fn from(error: ImageError) -> Self {
    Self::General
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::Random;

  use super::*;

  #[test]
  fn image_should_support_dynamic_mutation() {
    let mut image = Image::new(4, 4);

    for color in image.iter_mut() {
      *color = Color::random()
    }
  }
}