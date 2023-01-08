use std::collections::HashMap;

use surreal::io::VirtualPath;
use surreal::maths::FromRandom;
use surreal_editor::{AssetDatabase, AssetHash, AssetId, AssetMetadata, AssetTypeMetadata, Resource};

/// A helper for building up an [`AssetDatabase`] for use in test cases.
///
/// When writing files, use the `memory://` scheme to minimize churn on the file system.
#[must_use]
pub struct AssetDatabaseBuilder {
  assets: HashMap<String, AssetMetadata>,
}

impl Default for AssetDatabaseBuilder {
  fn default() -> Self {
    Self { assets: HashMap::new() }
  }
}

impl AssetDatabaseBuilder {
  /// Adds a resource to the [`AssetDatabase`] at the given [`VirtualPath`].
  pub fn with_resource(mut self, path: impl Into<VirtualPath>, resource: impl Resource) -> Self {
    let path = path.into();

    self.assets.insert(
      path.to_string(),
      AssetMetadata {
        id: AssetId::random(),
        hash: AssetHash::from_resource(&resource).expect("Failed to hash asset"),
        assets: vec![AssetTypeMetadata {
          offset: 0,
          kind: resource.get_type(),
        }],
      },
    );

    resource.save(path).expect("Failed to save resource");

    self
  }

  /// Builds the resultant [`AssetDatabase`].
  pub fn build(self) -> AssetDatabase {
    AssetDatabase::from_metadata(self.assets.into_iter())
  }
}
