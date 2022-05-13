//! A simple asset management system with support for hot file reloading.
//!
//! Assets are managed through an opaque `AssetHandle`. A handle is simply an ID.
//! Internally the asset manager controls asset states via a simple state machine.

use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

/// Represents a fallible result in the asset subsystem.
pub type AssetResult<T> = anyhow::Result<T>;

/// An opaque handle to an asset in the asset system.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
      AssetState::Unloaded => panic!("Asset is not loaded!")
    }
  }
}

impl<T> DerefMut for Asset<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    let state = unsafe { &mut *self.state.get() };

    match state {
      AssetState::Loaded(value) => value,
      AssetState::Unloaded => panic!("Asset is not loaded!")
    }
  }
}
