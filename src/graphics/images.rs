use image::{DynamicImage, ImageError, ImageFormat, RgbaImage};

use crate::assets::*;
use crate::graphics::Color;

/// A 2d image, iwth
pub struct Image {
  image: RgbaImage
}

impl Image {
  pub fn new(width: usize, height: usize, default_color: Color) -> Self {
    Self {
      image: RgbaImage::from_raw(
        width as u32,
        height as u32,
        Vec::with_capacity(width * height),
      ).unwrap()
    }
  }

  pub fn from(image: DynamicImage) -> Self {
    Self { image: image.into_rgba() }
  }

  #[inline]
  pub fn width(&self) -> usize {
    self.image.width() as usize
  }

  #[inline]
  pub fn height(&self) -> usize {
    self.image.height() as usize
  }
}

impl LoadableAsset for Image {
  fn load(path: impl AsRef<str>, context: &mut impl AssetContext) -> AssetResult<Self> {
    let format = ImageFormat::from_path(path.as_ref())?;
    let file = std::fs::File::open(path.as_ref())?;
    let reader = std::io::BufReader::new(file);
    let image = image::load(reader, format)?;

    Ok(Self::from(image))
  }
}

impl From<ImageError> for crate::assets::Error {
  fn from(error: ImageError) -> Self {
    Self::General
  }
}