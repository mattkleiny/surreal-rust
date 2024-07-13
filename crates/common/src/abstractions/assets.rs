//! Asset management for the engine

/// An error that can occur when loading an asset
#[derive(Debug)]
pub enum AssetError {
  /// The asset could not be found
  NotFound,
  /// The asset could not be loaded
  LoadFailed,
  /// The asset is not of the expected type
  TypeMismatch,
}

/// Represents an asset that can be loaded and used by the engine
pub trait Asset {}

/// An entry in the asset manager
enum AssetEntry {
  Unloaded,
  Loaded(Box<dyn Asset>),
}

/// A manager for assets
#[derive(Default)]
pub struct AssetManager {}
