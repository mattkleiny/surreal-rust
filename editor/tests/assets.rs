#![feature(anonymous_lifetime_in_impl_trait)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use surreal::io::{Deserializable, Serializable, VirtualPath};
use surreal::maths::FromRandom;
use surreal_editor::{AssetDatabase, AssetHash, AssetId, AssetMetadata, Resource};

#[test]
fn resources_with_differences_should_hash_differently() {
  let mut database = AssetDatabaseBuilder::default()
    .with_resource("memory://test1.json", TestResource { value: 1 })
    .with_resource("memory://test2.json", TestResource { value: 2 })
    .build();

  let hash1 = database.rehash("memory://test1.json").unwrap();
  let hash2 = database.rehash("memory://test1.json").unwrap();
  let hash3 = database.rehash("memory://test2.json").unwrap();

  assert_eq!(hash1, hash2);
  assert_ne!(hash1, hash3);
}

/// A helper for building up an [`AssetDatabase`] for use in test cases.
///
/// When writing files, use the `memory://` scheme to minimize churn on the file system.
#[must_use]
struct AssetDatabaseBuilder {
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
      },
    );

    resource.save(path).expect("Failed to save resource");

    self
  }

  /// Builds the resultant [`AssetDatabase`].
  pub fn build(self) -> AssetDatabase {
    AssetDatabase::from_assets(self.assets.into_iter())
  }
}

/// A simple test [`Resource`] for use in the [`AssetDatabase`].
#[derive(Serialize, Deserialize)]
pub struct TestResource {
  value: usize,
}

impl Resource for TestResource {
  fn load(path: impl Into<VirtualPath>) -> surreal::Result<Self> {
    Self::load_from_json(path)
  }

  fn save(&self, path: impl Into<VirtualPath>) -> surreal::Result<()> {
    self.save_to_json(path)
  }
}
