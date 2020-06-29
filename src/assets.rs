//! A simple asset management system with support for hot file reloading.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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
  cell: Arc<Mutex<AssetState<T>>>,
}

/// The internal state of an asset.
enum AssetState<T> {
  Ready(T),
  NotReady,
}

/// Permits loading an object from disk.
pub trait LoadableAsset {
  fn load(path: impl AsRef<Path>, context: &mut impl AssetContext) -> Self;
}

impl<T> Asset<T> {
  pub fn new(asset: T) -> Self {
    Self { cell: Arc::new(Mutex::new(AssetState::Ready(asset))) }
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
    unimplemented!()
  }

  pub fn swap(&mut self, other: T) {
    unimplemented!()
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_should_allocate_an_asset() {
    unimplemented!()
  }

  #[test]
  fn it_should_re_use_old_cache_entries() {
    unimplemented!()
  }
}