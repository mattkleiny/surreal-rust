//! Asset management abstractions.

use crate::platform::Path;

use super::*;

/// A system for managing asset resources.
pub struct AssetManager;

impl AssetManager {
  pub fn new() -> Self {
    Self {}
  }

  /// Loads an asset from the given path.
  pub fn load<A>(&mut self, _path: &Path) -> Result<A> {
    unimplemented!()
  }
}