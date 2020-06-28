//! An asset management system.

use std::sync::Arc;

pub struct AssetManager {}

/// A reference to an asset.
#[derive(Clone, Debug)]
pub struct Asset<T> {
  asset: Arc<AssetBox<T>>,
}

impl<T> Asset<T> {
  pub fn new(asset: T) -> Self {
    Self {
      asset: Arc::new(AssetBox { asset }),
    }
  }
}

#[derive(Debug)]
struct AssetBox<T> {
  asset: T,
}

#[cfg(test)]
mod tests {
  use crate::graphics::Image;

  use super::*;

  #[test]
  fn it_should_allocate_an_asset_box() {
    let image = Asset::new(Image {});
    let _ = image.clone();
    let _ = image.clone();
    let _ = image.clone();
    let _ = image.clone();
    let _ = image.clone();
  }
}