use core::io::VirtualPath;

/// A unique identifier for a [`Resource`] in the application.
///
/// IDs are unique across a single instance of an application, but not necessarily
/// across all applications.
pub type ResourceId = core::maths::Guid;

/// Represents a kind of resource in the application, and allows it to be
/// both loaded and persisted to/from the virtual file system.
pub trait Resource: Sized {
  fn load(path: impl Into<VirtualPath>) -> core::Result<Self>;
  fn save(&self, path: impl Into<VirtualPath>) -> core::Result<()>;
}

/// Loads [`Resource`] from the virtual file system.
pub fn load_resource<R: Resource>(path: impl Into<VirtualPath>) -> core::Result<R> {
  R::load(path.into())
}

/// Saves a [`Resource`] to the virtual file system.
pub fn save_resource<R: Resource>(resource: &R, path: impl Into<VirtualPath>) -> core::Result<()> {
  resource.save(path.into())
}
