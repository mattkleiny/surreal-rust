//! A simple asset management system with support for hot file reloading.

use std::cell::UnsafeCell;

use crate::collections::ArenaIndex;

pub type AssetResult<T> = std::result::Result<T, Error>;

/// A handle to an asset in the asset manager.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AssetHandle(ArenaIndex);

/// A shared pointer to an asset, with support for interior hot-reloading.
///
/// Asset loading might also be deferred via an async mechanism.
///
/// This asset can have it's contents asset updated at any time, permitting hot reload.
/// Each time the asset is borrowed, the most up-to-date content is returned.
pub struct Asset<T> {
  state: UnsafeCell<AssetState<T>>,
  manager: *mut AssetManager,
}

/// The internal state of an asset.
enum AssetState<T> {
  Ready(T),
  NotReady,
}

impl<T> Asset<T> {
  pub fn load(path: impl AsRef<str>) -> AssetResult<Asset<T>> where T: Loadable {
    unimplemented!()
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

impl<T> std::ops::Drop for Asset<T> {
  fn drop(&mut self) {
    unimplemented!()
  }
}

/// A manager for assets.
///
/// Assets are cached centrally by the manager, so accessing the same path
/// twice will always result in the same asset being returned.
pub struct AssetManager {}

impl AssetManager {
  pub fn load<T>(&mut self, path: impl AsRef<str>) -> AssetResult<Asset<T>> where T: Loadable {
    unimplemented!()
  }
}

/// Permits loading an asset from disk.
pub trait Loadable: Sized {
  fn load(path: impl AsRef<str>) -> AssetResult<Self>;
}

/// Permits hot-loading an asset as it changes on disk.
pub trait Reloadable: Loadable {
  fn reload(&mut self, path: impl AsRef<str>) -> AssetResult<()>;
}

#[derive(Debug)]
pub enum Error {
  General,
  IO(std::io::Error),
}

impl From<Error> for crate::Error {
  fn from(error: Error) -> Self {
    Self::Asset(error)
  }
}

impl From<std::io::Error> for Error {
  fn from(error: std::io::Error) -> Self {
    Self::IO(error)
  }
}