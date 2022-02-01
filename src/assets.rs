//! A simple asset management system with support for hot file reloading.

use std::cell::UnsafeCell;

use crate::io::VirtualPath;

/// Represents a fallible result in the asset subsystem.
pub type AssetResult<T> = anyhow::Result<T>;

/// An opaque handle to an asset in the asset system.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AssetHandle(u64);

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
  pub fn load(path: VirtualPath) -> AssetResult<Asset<T>> where T: Loadable {
    unimplemented!()
  }

  pub fn get(&self) -> Option<&T> {
    unimplemented!()
  }

  pub fn get_mut(&mut self) -> Option<&mut T> {
    unimplemented!()
  }
}

impl<T> std::ops::Drop for Asset<T> {
  fn drop(&mut self) {
    // TODO: reference counting semantics in the asset manager?
    unimplemented!()
  }
}

/// A manager for assets.
///
/// Assets are cached centrally by the manager, so accessing the same path
/// twice will always result in the same asset being returned.
pub struct AssetManager {}

impl AssetManager {
  pub fn new() -> Self {
    Self {}
  }

  pub fn load<T>(&mut self, path: VirtualPath) -> AssetResult<Asset<T>> where T: Loadable {
    unimplemented!()
  }
}

/// Permits loading an asset from disk.
pub trait Loadable: Sized {
  fn load(path: VirtualPath) -> AssetResult<Self>;
}