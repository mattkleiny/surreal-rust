//! Resource management for Surreal

use surreal_core::io::VirtualPath;

/// A unique identifier for a resource in the application.
pub type ResourceId = surreal_core::maths::Guid;

/// Represents a kind of resource in the application, and allows it to be
/// both loaded and persisted to/from the virtual file system.
pub trait Resource: Sized {
  fn load<'a>(path: impl Into<VirtualPath<'a>>) -> surreal_core::Result<Self>;
  fn save<'a>(&self, path: impl Into<VirtualPath<'a>>) -> surreal_core::Result<()>;
}

/// Loads [`Resource`] from the virtual file system.
pub fn load_resource<'a, R: Resource>(path: impl Into<VirtualPath<'a>>) -> surreal_core::Result<R> {
  R::load(path.into())
}

/// Saves a [`Resource`] to the virtual file system.
pub fn save_resource<'a, R: Resource>(resource: &R, path: impl Into<VirtualPath<'a>>) -> surreal_core::Result<()> {
  resource.save(path.into())
}
