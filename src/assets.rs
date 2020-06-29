//! An asset management system.

use std::sync::Arc;
use std::ops::Deref;

/// A manager for assets.
pub struct AssetManager {}

/// A reference to an asset.
#[derive(Clone)]
pub struct Asset<T> {
  asset: Arc<AssetBox<T>>,
}

impl<T> Deref for Asset<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.asset.asset
  }
}

struct AssetBox<T> {
  asset: T,
  is_ready: bool,
}

impl<T> Asset<T> {
  pub fn new(asset: T) -> Self {
    Self {
      asset: Arc::new(AssetBox {
        asset,
        is_ready: true,
      })
    }
  }

  pub fn is_ready(&self) -> bool {
    self.asset.is_ready
  }
}

#[cfg(test)]
mod tests {
  use crate::graphics::Image;

  use super::*;

  #[test]
  fn it_should_allocate_an_asset_box() {
    let image = Asset::new(Image {});

    let pixels = image.get_pixels();
  }
}