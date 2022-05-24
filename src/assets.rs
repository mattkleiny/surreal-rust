//! A simple asset management system with support for hot file reloading.

use std::collections::HashMap;
use std::any::Any;

use crate::io::{AsVirtualPath, VirtualPath};

/// Represents a fallible result in the asset subsystem.
pub type AssetResult<T> = anyhow::Result<T>;

/// An opaque pointer to a shared asset reference in an `AssetManager`.
pub struct Asset<T> {
  state: std::rc::Rc<AssetState<T>>,
}

/// The internal state for an `Asset`.
enum AssetState<T> {
  NotReady,
  Ready(T),
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
  pub fn load_asset<A>(&self, path: impl AsVirtualPath) -> AssetResult<Asset<A>> {
    let key = path.as_virtual_path().to_string();

    todo!()
  }
}

/// Allows loading an asset from the virtual file system.
pub trait AssetLoader<A> {
  /// Determines if the given path is an asset that can be loaded.
  fn can_load(&self, _context: &AssetLoadContext) -> bool { true }

  /// Loads the asset from the given path.
  fn load(&self, context: &AssetLoadContext) -> AssetResult<A>;
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
  pub fn load_asset<A>(&self, _path: VirtualPath) -> AssetResult<A> {
    todo!()
  }
}