//! Asset server, database and build pipelines for use in editor tooling.
//!
//! The asset system maintains a flat-file database of all assets in the project,
//! with support for transient imports via [`AssetImporter`] behaviours. This
//! allows projects to participate in the asset system by writing data in bespoke
//! formats and importing them into a central system.
//!
//! The output of the asset database is put through a build pipeline to produce
//! artifacts and [`AssetBundle`]s for consumption by the game at runtime.

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hasher;

use serde::{Deserialize, Serialize};

use surreal::io::{Deserializable, InputStream, Serializable, VirtualPath};
use surreal::utilities::Type;

use super::Resource;

/// A unique identifier for an asset.
pub type AssetId = surreal::maths::Guid;

/// A server for managing assets in the project.
///
/// The asset server is responsible for managing the asset database, providing
/// access to assets in the project, managing asset importers and triggering
/// imports when assets are modified, and for invoking the build pipeline.
pub trait AssetServer {
  // file management
  fn create_folder(&mut self, path: &str) -> surreal::Result<()>;
  fn create_asset(&mut self, path: &str) -> surreal::Result<()>;
  fn import_asset(&mut self, path: &str) -> surreal::Result<()>;
  fn load_all_assets(&mut self, path: &str) -> surreal::Result<()>;
  fn load_first_asset(&mut self, path: &str) -> surreal::Result<()>;
  fn load_main_asset(&mut self, path: &str) -> surreal::Result<()>;
  fn copy_asset(&mut self, source: &str, destination: &str) -> surreal::Result<()>;
  fn rename_asset(&mut self, source: &str, destination: &str) -> surreal::Result<()>;
  fn delete_asset(&mut self, path: &str) -> surreal::Result<()>;
  fn delete_assets(&mut self, paths: &[&str]) -> surreal::Result<()>;
  fn refresh_all(&mut self) -> surreal::Result<()>;

  // importer management
  fn register_importer(&mut self, importer: Box<dyn AssetImporter>) -> surreal::Result<()>;
  fn unregister_importer(&mut self, importer: Box<dyn AssetImporter>) -> surreal::Result<()>;

  // build pipeline management
  fn build_asset_bundle<B: AssetBundle>(&mut self, path: &str, manifest: &AssetManifest) -> surreal::Result<()>;
}

/// The singleton asset database facade; this is the central point of access for all
/// asset-related operations.
///
/// The asset database is responsible for maintaining a flat-file database of all
/// assets in the project, and for managing the import of assets from the file system.
///
/// See [`AssetServer`], [`AssetImporter`] and [`AssetBundle`] for more details.
#[derive(Default, Debug)]
pub struct AssetDatabase {
  metadata: HashMap<String, AssetMetadata>,
}

impl AssetDatabase {
  /// Builds an [`AssetDatabase`] from an existing set of [`AssetMetadata`].
  pub fn from_metadata(metadata: impl Iterator<Item = (String, AssetMetadata)>) -> Self {
    Self {
      metadata: metadata.collect(),
    }
  }

  /// Creates an [`AssetHash`] for the asset at the given [`VirtualPath`] and remembers it.
  pub fn rehash(&mut self, path: impl Into<VirtualPath>) -> surreal::Result<AssetHash> {
    let path = path.into();

    let mut stream = path.open_input_stream()?;
    let hash = AssetHash::from_stream(&mut stream)?;

    self.metadata.entry(path.to_string()).and_modify(|entry| {
      entry.hash = hash;
    });

    Ok(hash)
  }
}

/// An importer for assets.
///
/// Importers are responsible for importing assets from the file system into the
/// asset database. Importers are registered with the asset database and are
/// invoked when assets are modified.
///
/// The output of the importer is cached in the asset database, and is used to
/// determine whether an asset needs to be re-imported.
pub trait AssetImporter {
  /// Determines if the importer can import the given asset.
  fn can_handle(&self, path: &VirtualPath) -> bool;
}

// TODO: import into in-memory cache, and expand on the file system
// TODO: use object ids to look-up instances depending on access patterns
// TODO:

/// A bundle of assets.
///
/// Bundles are responsible for composing assets into a form that can be consumed
/// by the game at runtime. Bundles are used by the [`AssetServer`] to pack assets
/// into central files for distribution.
pub trait AssetBundle {}

/// A unique hash of an asset.
///
/// This is used to determine whether an asset has been modified since it was last
/// imported. If the hash of an asset has changed, the asset will be re-imported.
///
/// The hash is calculated by hashing the contents of the asset file.
#[derive(Serialize, Deserialize, Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct AssetHash(u64);

impl AssetHash {
  /// Creates an [`AssetHash`] from an existing [`Resource`].
  pub fn from_resource(resource: &impl Resource) -> surreal::Result<Self> {
    Ok(Self::from_bytes(&resource.to_binary()?))
  }

  /// Creates an [`AssetHash`] from the given raw slice of bytes.
  pub fn from_bytes(bytes: &[u8]) -> Self {
    let mut hasher = fxhash::FxHasher::default();

    hasher.write(bytes);

    Self(hasher.finish())
  }

  /// Creates a new [`AssetHash`] from a given [`InputStream`].
  pub fn from_stream(stream: &mut impl InputStream) -> surreal::Result<Self> {
    let mut buffer = [0; 1024];
    let mut hasher = fxhash::FxHasher::default();

    loop {
      let read = stream.read(&mut buffer)?;

      if read == 0 {
        break;
      }

      hasher.write(&buffer[..read]);
    }

    Ok(Self(hasher.finish()))
  }
}

impl From<u64> for AssetHash {
  #[inline(always)]
  fn from(value: u64) -> Self {
    Self(value)
  }
}

impl Into<u64> for AssetHash {
  #[inline(always)]
  fn into(self) -> u64 {
    self.0
  }
}

/// Serializable metadata for an asset.
///
/// This is used to store metadata about an asset in the asset database on disk.
/// This includes the asset's unique identifier, the import options for the asset,
/// and the hash of the asset.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetMetadata {
  pub id: AssetId,
  pub hash: AssetHash,
  pub assets: Vec<AssetTypeMetadata>,
}

/// Describes the kinds of assets that are present at a particular path.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetTypeMetadata {
  pub offset: u16,
  pub kind: Type,
}

/// A manifest of assets.
///
/// Manifests are used to describe the contents of an asset bundle. They are
/// used by the [`AssetServer`] to determine which assets are contained in a
/// bundle, and to determine whether a bundle needs to be rebuilt.
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct AssetManifest {
  assets: HashMap<String, AssetMetadata>,
}

/// A builder pattern for [`AssetManifest`]s.
#[must_use]
#[derive(Default)]
pub struct AssetManifestBuilder {
  manifest: AssetManifest,
}

impl AssetManifestBuilder {
  /// Adds an existing asset to the [`AssetManifest`].
  ///
  /// If the asset does not exist, or the metadata cannot be found, it will be ignored.
  pub fn add_asset(mut self, path: impl Into<VirtualPath>) -> Self {
    let asset_path = path.into();
    let metadata_path = asset_path.change_extension("meta");

    if let Ok(metadata) = AssetMetadata::from_yaml_file(metadata_path) {
      self.manifest.assets.insert(asset_path.to_string(), metadata);
    }

    self
  }

  /// Builds the resultant [`AssetManifest`].
  pub fn build(self) -> AssetManifest {
    self.manifest
  }
}

/// A '.pak' file; a compressed bundle of assets.
///
/// This file format is a binary encoded stream of assets, efficiently packed
/// for distribution. The format is designed to be read-only, and is not intended
/// to be modified at runtime.
#[derive(Default)]
pub struct PakBundle;

impl AssetBundle for PakBundle {}

#[cfg(test)]
mod tests {
  use surreal::macros::Object;

  use super::*;

  #[derive(Object)]
  struct SpriteResource;

  #[test]
  fn asset_manifest_should_serialize_to_yaml() {
    let manifest = AssetManifestBuilder::default()
      .add_asset("local://../assets/sprites/bunny.png")
      .build();

    let yaml = manifest.to_yaml().unwrap();

    println!("{}", yaml);

    let manifest = AssetManifest::from_yaml(&yaml).unwrap();

    println!("{:#?}", manifest);
  }
}
