use surreal::io::VirtualPath;
use surreal_editor::{AssetDatabase, Resource};

/// A helper for building up an [`AssetDatabase`] for use in test cases.
///
/// When writing files, use the `memory://` scheme to minimize churn on the file system.
#[must_use]
#[derive(Default)]
pub struct AssetDatabaseBuilder {}

impl AssetDatabaseBuilder {
  /// Adds a resource to the [`AssetDatabase`] at the given [`VirtualPath`].
  pub fn add_resource(self, _path: impl Into<VirtualPath>, _resource: impl Resource) -> Self {
    todo!()
  }

  /// Builds the resultant [`AssetDatabase`].
  pub fn build(self) -> AssetDatabase {
    todo!()
  }
}
