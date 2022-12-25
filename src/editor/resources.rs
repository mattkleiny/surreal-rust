//! Resource management for Surreal

use crate::io::VirtualPath;

/// A unique identifier for a resource in the application.
pub type ResourceId = crate::maths::Guid;

/// Represents a kind of resource in the application, and allows it to be
/// both loaded and persisted to/from the virtual file system.
pub trait Resource: Sized {
  // fn asset_id(&self) -> ResourceId;
  // fn instance_id(&self) -> ResourceId;

  /// Loads this type of resource from the given path.
  fn load<'a>(path: impl Into<VirtualPath<'a>>) -> crate::Result<Self>;

  /// Saves this resource to the given path.
  fn save<'a>(&self, path: impl Into<VirtualPath<'a>>) -> crate::Result<()>;
}

/// Loads [`Resource`] from the virtual file system.
pub fn load_resource<'a, R: Resource>(path: impl Into<VirtualPath<'a>>) -> crate::Result<R> {
  R::load(path.into())
}

/// Saves a [`Resource`] to the virtual file system.
pub fn save_resource<'a, R: Resource>(resource: &R, path: impl Into<VirtualPath<'a>>) -> crate::Result<()> {
  resource.save(path.into())
}
