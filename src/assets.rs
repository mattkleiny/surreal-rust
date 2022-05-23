//! A simple asset management system with support for hot file reloading.

use crate::io::{AsVirtualPath, VirtualPath};

/// Represents a fallible result in the asset subsystem.
pub type AssetResult<T> = anyhow::Result<T>;

/// A manager for assets.
///
/// The manager is responsible for loading assets from the virtual file system,
/// and providing access to them.
///
/// The manager is also responsible for keeping track of asset dependencies,
/// and automatically reloading assets when they are modified.
pub struct AssetManager {}

impl AssetManager {
  /// Creates a new asset manager.
  pub fn new() -> Self {
    Self {}
  }

  /// Adds a new asset loader to the manager.
  pub fn add_loader<A>(&mut self, _loader: impl AssetLoader<A>) {
    todo!()
  }

  /// Attempts to load an asset from the given path.
  pub fn load_asset<A>(&self, _path: impl AsVirtualPath) -> AssetResult<A> {
    todo!()
  }
}

/// A context for asset operations.
///
/// The context allows other components to refer back to the asset pipeline.
pub struct AssetLoadContext<'a> {
  /// The path of the asset being loaded.
  pub path: VirtualPath<'a>,
  manager: &'a AssetManager,
}

impl<'a> AssetLoadContext<'a> {
  /// Loads a dependent asset from the given path.
  pub fn load_asset<A>(&self, path: VirtualPath) -> AssetResult<A> {
    self.manager.load_asset(path)
  }
}

/// Allows loading an asset from the virtual file system.
pub trait AssetLoader<A> {
  /// Determines if the given path is an asset that can be loaded.
  fn can_load(&self, _context: &AssetLoadContext) -> bool { true }

  /// Loads the asset from the given path.
  fn load(&self, context: &AssetLoadContext) -> AssetResult<A>;
}
