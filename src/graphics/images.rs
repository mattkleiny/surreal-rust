use crate::assets::{AssetResult, Loadable};

/// An image of pixels, uncompressed, in RGBA format.
///
/// An image can be loaded from disc and dynamically manipulated.
pub struct Image {}

impl Image {
  pub fn new(width: usize, height: usize) -> Self {
    unimplemented!()
  }
}

impl Loadable for Image {
  fn load(path: impl AsRef<str>) -> AssetResult<Self> {
    unimplemented!()
  }
}