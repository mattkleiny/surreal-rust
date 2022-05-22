//! A simple asset management system with support for hot file reloading.

use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::io::VirtualPath;

/// Represents a fallible result in the asset subsystem.
pub type AssetResult<T> = anyhow::Result<T>;

/// An opaque handle to an asset in the asset system.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct AssetHandle(usize);

/// A shared pointer to an asset, with support for interior hot-reloading.
///
/// Asset loading might also be deferred via an async mechanism.
///
/// This asset can have it's contents asset updated at any time, permitting hot reload.
/// Each time the asset is borrowed, the most up-to-date content is returned.
#[derive(Clone)]
pub struct Asset<T> {
  state: Rc<UnsafeCell<AssetState<T>>>,
}

/// The internal state machine for an asset.
enum AssetState<T> {
  Unloaded,
  Loading,
  Loaded(T),
}

impl<T> Asset<T> {
  fn new(value: T) -> Self {
    Self {
      state: Rc::new(UnsafeCell::new(AssetState::Loaded(value))),
    }
  }
}

impl<T> Deref for Asset<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    let state = unsafe { &*self.state.get() };

    match state {
      AssetState::Loaded(value) => &value,
      _ => panic!("Asset is not loaded!")
    }
  }
}

impl<T> DerefMut for Asset<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    let state = unsafe { &mut *self.state.get() };

    match state {
      AssetState::Loaded(value) => value,
      _ => panic!("Asset is not loaded!")
    }
  }
}

/// A context for asset operations.
pub struct AssetLoadContext<'a> {
  /// The path of the asset being loaded.
  pub path: VirtualPath<'a>,
}

impl<'a> AssetLoadContext<'a> {
  /// Loads a dependent asset from the given path.
  pub fn load_asset<T>(&self, _path: VirtualPath) -> AssetResult<T> {
    todo!()
  }
}

/// Allows loading an asset from the virtual file system.
pub trait AssetLoader {
  /// The type of asset that this loader can load.
  type Asset;

  /// Determines if the given path is an asset that can be loaded.
  fn can_load(&self, context: AssetLoadContext) -> bool;

  /// Loads the asset from the given path.
  fn load(&self, context: AssetLoadContext) -> AssetResult<Self::Asset>;
}