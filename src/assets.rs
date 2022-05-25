//! A simple asset management system with support for hot file reloading.

use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

use crate::io::{AsVirtualPath, VirtualPath};

pub trait Loadable {
  fn from_path(path: impl AsVirtualPath) -> Self;
}

/// The internal state for an `Asset`.
#[allow(dead_code)]
enum AssetState<T> {
  NotReady,
  Ready(T), // TODO: implement me
}

/// An opaque pointer to a shared asset reference in an `AssetManager`.
pub struct Asset<T> {
  state: Rc<AssetState<T>>,
}

impl<T> Asset<T> {
  /// Accesses the asset's data.
  pub fn get(&self) -> Option<&T> {
    match self.state.as_ref() {
      AssetState::Ready(value) => Some(value),
      _ => None,
    }
  }
}

/// A manager for assets.
///
/// The manager is responsible for loading assets from the virtual file system,
/// and providing access to them.
///
/// The manager is also responsible for keeping track of asset dependencies,
/// and automatically reloading assets when they are modified.
pub struct AssetManager {
  loaders: Vec<Box<dyn Any>>,
  assets: HashMap<String, Box<dyn Any>>,
}

impl AssetManager {
  /// Creates a new asset manager.
  pub fn new() -> Self {
    Self {
      loaders: Vec::new(),
      assets: HashMap::new(),
    }
  }

  /// Adds a new asset loader to the manager.
  pub fn add_loader<A>(&mut self, loader: impl AssetLoader<A> + 'static) {
    self.loaders.push(Box::new(loader));
  }

  /// Attempts to load an asset from the given path.
  pub fn load_asset<A: 'static>(&mut self, path: impl AsVirtualPath) -> crate::Result<Asset<A>> {
    let key = path.as_virtual_path().to_string();

    let state = self.assets
      .entry(key)
      .or_insert_with(|| {
        // TODO: kick off loading process for this asset?
        Box::new(Rc::new(AssetState::<A>::NotReady))
      })
      .downcast_ref::<Rc<AssetState<A>>>()
      .expect("Failed to access asset state");

    Ok(Asset { state: state.clone() })
  }
}

/// Allows loading an asset from the virtual file system.
pub trait AssetLoader<A> {
  /// Determines if the given path is an asset that can be loaded.
  fn can_load(&self, _context: &AssetLoadContext) -> bool { true }

  /// Loads the asset from the given path.
  fn load(&self, context: &AssetLoadContext) -> crate::Result<A>;
}

/// A context for asset operations.
///
/// The context allows other components to refer back to the asset pipeline.
pub struct AssetLoadContext<'a> {
  /// The path of the asset being loaded.
  pub path: VirtualPath<'a>,
  _manager: &'a AssetManager,
}

impl<'a> AssetLoadContext<'a> {
  /// Loads a dependent asset from the given path.
  pub fn load_asset<A>(&self, _path: VirtualPath) -> crate::Result<A> {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn assets_should_load() {
    let mut manager = AssetManager::new();

    let asset: Asset<String> = manager.load_asset("test.txt").expect("Failed to load asset");

    if let Some(string) = asset.get() {
      println!("Ready {:}", string);
    } else {
      println!("Not ready");
    }
  }
}