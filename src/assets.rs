//! A simple asset management system with support for hot file reloading.

use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

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
}

/// The internal state of an asset.
enum AssetState<T> {
  Ready(T),
  NotReady,
}

impl<T> Asset<T> {
  fn new(value: T) -> Self {
    Self {
      state: UnsafeCell::new(AssetState::Ready(value)),
    }
  }

  pub fn load(path: VirtualPath) -> AssetResult<Asset<T>> where T: Loadable {
    let result = T::load(path)?;
    let asset = Self::new(result);

    Ok(asset)
  }
}

impl<T> Deref for Asset<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    let state = unsafe { &*self.state.get() };

    match state {
      AssetState::Ready(value) => &value,
      AssetState::NotReady => panic!("Asset is not ready!")
    }
  }
}

impl<T> DerefMut for Asset<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    let state = unsafe { &mut *self.state.get() };

    match state {
      AssetState::Ready(value) => value,
      AssetState::NotReady => panic!("Asset is not ready!")
    }
  }
}

/// Permits loading an asset from disk.
pub trait Loadable: Sized {
  fn load(path: VirtualPath) -> AssetResult<Self>;
}

/// Permits hot-loading an asset as it changes on disk.
pub trait Reloadable: Loadable {
  fn reload(&mut self, path: VirtualPath) -> AssetResult<()> {
    Ok(*self = Self::load(path)?)
  }
}

#[cfg(test)]
mod tests {
  use crate::graphics::Image;

  use super::*;

  #[test]
  fn it_should_load_an_image() {
    let image: Asset<Image> = Asset::load(VirtualPath::parse("test.png")).unwrap();
    let pixels = image.as_slice();

    for pixel in pixels {
      println!("{:?}", pixel);
    }
  }
}