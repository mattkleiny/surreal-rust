use surreal::io::VirtualPath;

/// A unique identifier for a [`Resource`] in the application.
///
/// IDs are unique across a single instance of an application, but not
/// necessarily across all applications.
pub type ResourceId = surreal::maths::Guid;

/// Represents a kind of resource in the application, and allows it to be
/// both loaded from, and persisted to, the virtual file system.
pub trait Resource: Sized {
  fn load(path: impl Into<VirtualPath>) -> surreal::Result<Self>;
  fn save(&self, path: impl Into<VirtualPath>) -> surreal::Result<()>;
}

/// Loads [`Resource`] from the virtual file system at the given [`VirtualPath`].
pub fn load_resource<R: Resource>(path: impl Into<VirtualPath>) -> surreal::Result<R> {
  R::load(path.into())
}

/// Saves a [`Resource`] to the virtual file system at the [`VirtualPath`].
pub fn save_resource<R: Resource>(resource: &R, path: impl Into<VirtualPath>) -> surreal::Result<()> {
  resource.save(path.into())
}
