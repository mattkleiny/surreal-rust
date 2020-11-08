//! A simple asset management system with support for hot file reloading.

use enumflags2::_internal::core::ops::Deref;

use crate::io::Path;

/// Context for asset operations.
pub trait AssetContext {}

/// A manager for assets.
///
/// Assets are cached centrally by the manager, so accessing the same path
/// twice will always result in the same asset being returned.
pub struct AssetManager {}

impl AssetManager {
  pub fn load<T: LoadableAsset>(&mut self, path: Path) -> Asset<T> {
    Asset::load(path, self)
  }
}

impl AssetContext for AssetManager {}

/// A shared pointer to an asset, with support for interior hot-reloading.
///
/// Asset loading might also be deferred via an async mechanism.
///
/// This asset can have it's contents asset updated at any time, permitting hot reload.
/// Each time the asset is borrowed, the most up-to-date content is returned.
pub struct Asset<T> {
  state: AssetState<T>,
}

/// The internal state of an asset.
enum AssetState<T> {
  Ready(T),
  NotReady,
}

impl<T> Asset<T> {
  pub fn load(path: Path, context: &mut impl AssetContext) -> Asset<T>
  where
    T: LoadableAsset,
  {
    Asset {
      state: AssetState::Ready(T::load(path, context)),
    }
  }

  pub fn get(&self) -> Option<&T> {
    unimplemented!()
  }
}

impl<T> Deref for Asset<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.get().expect("This asset has not finished loading!")
  }
}

/// Permits loading an object from disk.
pub trait LoadableAsset {
  fn load(path: Path, context: &mut impl AssetContext) -> Self;
}
