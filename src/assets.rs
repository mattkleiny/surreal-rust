//! A simple asset management system with support for hot file reloading.

use crate::io::Path;

pub type AssetResult<T> = std::result::Result<T, Error>;

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
  pub fn load(path: Path, context: &mut impl AssetContext) -> AssetResult<Asset<T>>
    where T: LoadableAsset {
    let asset = T::load(path, context)?;

    Ok(Asset {
      state: AssetState::Ready(asset),
    })
  }

  pub fn get(&self) -> Option<&T> {
    unimplemented!()
  }

  pub fn get_mut(&mut self) -> Option<&mut T> {
    unimplemented!()
  }
}

impl<T> std::ops::Deref for Asset<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.get().expect("This asset has not finished loading!")
  }
}

impl<T> std::ops::DerefMut for Asset<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.get_mut().expect("This asset has not finished loading!")
  }
}

/// Permits loading an object from disk.
pub trait LoadableAsset: Sized {
  fn load(path: Path, context: &mut impl AssetContext) -> AssetResult<Self>;
}

/// Permits hot-loading an asset as it changes on disk.
pub trait ReloadableAsset: LoadableAsset {
  fn on_asset_reload(&mut self, updated_asset: &mut Self);
}

/// A manager for assets.
///
/// Assets are cached centrally by the manager, so accessing the same path
/// twice will always result in the same asset being returned.
pub struct AssetManager {}

/// Context for asset operations.
pub trait AssetContext {
  fn load<T: LoadableAsset>(&mut self, path: Path) -> AssetResult<Asset<T>>;
}

impl AssetContext for AssetManager {
  fn load<T: LoadableAsset>(&mut self, path: Path) -> AssetResult<Asset<T>> {
    unimplemented!()
  }
}

#[derive(Debug)]
pub enum Error {
  UnableToLoadAsset
}

impl From<Error> for crate::Error {
  fn from(error: Error) -> Self {
    Self::Asset(error)
  }
}