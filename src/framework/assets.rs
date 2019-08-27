//! Asset management abstractions.

use std::any::Any;
use std::sync::Arc;

use crate::platform::Path;

use super::*;

/// Represents a reference to some asset in the asset manager.
#[derive(Clone, Debug)]
pub enum AssetRef<T> {
  Ready(Arc<T>),
  Loading,
  NotFound,
}

/// A system for managing asset resources.
pub struct AssetManager {
  assets: Vec<&'static dyn Any>
}

impl AssetManager {
  pub fn new() -> Self {
    Self {
      assets: Vec::new()
    }
  }

  /// Loads an asset from the given path.
  pub fn load<T>(&mut self, _path: &Path) -> Result<AssetRef<T>> {
    unimplemented!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_load_new_assets() {
    let mut manager = AssetManager::new();

    let path = Path::new("res://tests/simple.json");
    let asset = manager.load::<String>(&path).unwrap();

    if let AssetRef::Ready(text) = asset {
      println!("{}", text)
    }
  }
}