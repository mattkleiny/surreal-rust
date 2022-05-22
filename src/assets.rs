//! A simple asset management system with support for hot file reloading.

use crate::io::VirtualPath;

/// Represents a fallible result in the asset subsystem.
pub type AssetResult<T> = anyhow::Result<T>;

/// An opaque handle to an asset in the asset system.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct AssetHandle(usize);

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