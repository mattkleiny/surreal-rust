//! Asset management abstractions.

use crate::platform::Path;

use super::*;

/// An asset type that may be loaded by the asset system.
pub trait Asset: Sized {}

/// A system for managing asset resources.
pub struct AssetManager;

impl AssetManager {
  pub fn new() -> Self {
    Self {}
  }

  /// Loads an asset from the given path.
  pub fn load<A: Asset>(&mut self, _path: &Path) -> Result<A> {
    unimplemented!()
  }
}