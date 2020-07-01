//! A simple asset management system with support for hot file reloading.

use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub use crate::vfs::Path;

/// Context for asset operations.
pub trait AssetContext {
  fn try_get<T>(&self, path: impl AsRef<Path>) -> Option<Asset<T>>;
}

/// A manager for assets.
///
/// Assets are cached centrally by the manager, so accessing the same path
/// twice will always result in the same asset being returned.
pub struct AssetManager {
  asset_cache: HashMap<Path, u16>,
}

impl AssetManager {
  pub fn new() -> Self {
    Self { asset_cache: HashMap::new() }
  }

  /// Loads an asset from the given path, caching the results in the manager.
  ///
  /// If the asset has already been loaded, it will be returned instead of loading again.
  pub fn load<T: LoadableAsset>(&mut self, path: &impl AsRef<Path>) -> Asset<T> {
    Asset::load(path, self)
  }
}

impl AssetContext for AssetManager {
  fn try_get<T>(&self, path: impl AsRef<Path>) -> Option<Asset<T>> {
    None // TODO: implement me
  }
}

/// A shared pointer to an asset, with support for interior hot-reloading.
///
/// Asset loading might also be deferred via an async mechanism.
///
/// This asset can have it's contents asset updated at any time, permitting hot reload.
/// Each time the asset is borrowed, the most up-to-date content is returned.
pub struct Asset<T> {
  cell: Arc<UnsafeCell<AssetState<T>>>,
}

/// The internal state of an asset.
enum AssetState<T> {
  Ready(T),
  NotReady,
}

unsafe impl<T> Send for Asset<T> {}
unsafe impl<T> Sync for Asset<T> {}

impl<T> Asset<T> {
  pub fn new(asset: T) -> Self {
    Self { cell: Arc::new(UnsafeCell::new(AssetState::Ready(asset))) }
  }

  pub fn load(path: &impl AsRef<Path>, context: &mut impl AssetContext) -> Self
    where T: LoadableAsset {
    if let Some(asset) = context.try_get(path) {
      asset
    } else {
      Self::new(T::load(path, context))
    }
  }

  pub fn is_ready(&self) -> bool {
    match unsafe { self.cell.get().as_ref() } {
      Some(state) => match state {
        AssetState::Ready(_) => true,
        AssetState::NotReady => false,
      },
      _ => false
    }
  }

  pub fn swap(&mut self, other: T) {
    unsafe {
      *self.cell.get().as_mut().unwrap() = AssetState::Ready(other);
    }
  }
}

impl<T> Deref for Asset<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    match unsafe { self.cell.get().as_mut().unwrap() } {
      AssetState::Ready(asset) => asset,
      AssetState::NotReady => panic!("This asset is not yet ready!")
    }
  }
}

/// Permits loading an object from disk.
pub trait LoadableAsset {
  fn load(path: &impl AsRef<Path>, context: &mut impl AssetContext) -> Self;
}

#[cfg(test)]
mod tests {
  use crate::assets::Asset;

  #[test]
  fn it_should_allocate_an_asset() {
    let mut asset = Asset::new("Test");

    let reference1 = &asset;
    let reference2 = &asset;

    asset.swap("Test 2");

    assert_eq!(reference1, "Test 2");
    assert_eq!(reference2, "Test 2");
  }

  #[test]
  fn it_should_re_use_old_cache_entries() {
    unimplemented!()
  }
}