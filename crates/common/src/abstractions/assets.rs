//! Asset management for the engine

use crate::{FromStream, ToVirtualPath};

/// Represents an asset that can be loaded and used by the engine
pub trait Asset {}

/// A manager for assets
#[derive(Default)]
pub struct AssetManager {}

impl AssetManager {
  /// Loads an asset from the given path
  pub fn load<A: Asset + FromStream>(&self, path: impl ToVirtualPath) -> Result<A, AssetError> {
    let path = path.to_virtual_path();
    let mut stream = path.open_input_stream().map_err(|_| AssetError::NotFound)?;

    A::from_stream(&mut stream).map_err(|_| AssetError::LoadFailed)
  }
}

/// An error that can occur when loading an asset
#[derive(Debug)]
pub enum AssetError {
  NotFound,
  LoadFailed,
  TypeMismatch,
}
