#![feature(anonymous_lifetime_in_impl_trait)]

use serde::{Deserialize, Serialize};

use surreal::io::{Deserializable, Serializable, VirtualPath};
use surreal::macros::Object;
use surreal_editor::Resource;

mod common;

#[test]
fn resources_with_differences_should_hash_differently() {
  let mut database = common::AssetDatabaseBuilder::default()
    .with_resource("memory://test1.json", TestResource { value: 1 })
    .with_resource("memory://test2.json", TestResource { value: 2 })
    .build();

  let hash1 = database.rehash("memory://test1.json").unwrap();
  let hash2 = database.rehash("memory://test1.json").unwrap();
  let hash3 = database.rehash("memory://test2.json").unwrap();

  assert_eq!(hash1, hash2);
  assert_ne!(hash1, hash3);
}

/// A simple test [`Resource`] for use in the [`AssetDatabase`].
#[derive(Object, Serialize, Deserialize)]
pub struct TestResource {
  value: usize,
}

impl Resource for TestResource {
  fn load(path: impl Into<VirtualPath>) -> surreal::Result<Self> {
    Self::from_json_file(path)
  }

  fn save(&self, path: impl Into<VirtualPath>) -> surreal::Result<()> {
    self.to_json_file(path)
  }
}
