//! Asset management abstractions.

use std::any::Any;
use std::sync::Arc;

use crate::platform::Path;

use super::*;

/// Represents a reference to some asset in the asset manager.
#[derive(Clone, Debug)]
pub enum AssetRef<'a, T> {
  Ready(Arc<&'a T>),
  Loading,
  NotFound,
}

/// A system for managing asset resources.
pub struct AssetManager<'a> {
  assets: Vec<&'a dyn Any>
}

impl<'a> AssetManager<'a> {
  pub fn new() -> Self {
    Self {
      assets: Vec::new()
    }
  }

  /// Loads an asset from the given path.
  pub fn load<T>(&mut self, _path: &Path) -> Result<AssetRef<'a, T>> {
    unimplemented!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_load_new_assets() {
    let mut manager = AssetManager::new();
    let asset = manager.load::<String>(&Path::new("res://test.json")).unwrap();

    if let AssetRef::Ready(text) = asset {
      println!("{}", text)
    }
  }
}