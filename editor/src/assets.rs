//! Asset server, database and build pipelines for use in editor tooling.
//!
//! The asset system maintains a flat-file database of all assets in the project,
//! with support for transient imports via [`AssetImporter`] behaviours. This
//! allows projects to participate in the asset system by writing data in bespoke
//! formats and importing them into a central system.
//!
//! The output of the asset database is put through a build pipeline to produce
//! artifacts and [`AssetBundle`]s for consumption by the game at runtime.

use std::collections::HashSet;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use surreal::io::VirtualPath;

use surreal::maths::FromRandom;

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
pub struct AssetDatabase;

impl AssetDatabase {
  pub fn create_folder(_path: &str) -> surreal::Result<()> {
    todo!()
  }

  pub fn create_asset(_path: &str) -> surreal::Result<()> {
    todo!()
  }

  pub fn import_asset(_path: &str) -> surreal::Result<()> {
    todo!()
  }

  pub fn load_all_assets(_path: &str) -> surreal::Result<()> {
    todo!()
  }

  pub fn load_first_asset(_path: &str) -> surreal::Result<()> {
    todo!()
  }

  pub fn load_main_asset(_path: &str) -> surreal::Result<()> {
    todo!()
  }

  pub fn copy_asset(_source: &str, _destination: &str) -> surreal::Result<()> {
    todo!()
  }

  pub fn rename_asset(_source: &str, _destination: &str) -> surreal::Result<()> {
    todo!()
  }

  pub fn delete_asset(_path: &str) -> surreal::Result<()> {
    todo!()
  }

  pub fn delete_assets(_paths: &[&str]) -> surreal::Result<()> {
    todo!()
  }

  pub fn refresh_all(&mut self) -> surreal::Result<()> {
    todo!()
  }

  pub fn register_importer(_importer: Box<dyn AssetImporter>) -> surreal::Result<()> {
    todo!()
  }

  pub fn unregister_importer(_importer: Box<dyn AssetImporter>) -> surreal::Result<()> {
    todo!()
  }

  pub fn build_asset_bundle<B: AssetBundle + Default>(_path: &str, _manifest: &AssetManifest) -> surreal::Result<B> {
    // TODO: implement me
    Ok(B::default())
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
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct AssetHash([u8; 32]);

impl Serialize for AssetHash {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(base64::encode(&self.0).as_str())
  }
}

impl<'de> Deserialize<'de> for AssetHash {
  fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
    todo!()
  }
}

/// Serializable metadata for an asset.
///
/// This is used to store metadata about an asset in the asset database on disk.
/// This includes the asset's unique identifier, the import options for the asset,
/// and the hash of the asset.
#[derive(Serialize, Deserialize)]
pub struct AssetMetadata {
  pub id: AssetId,
  pub options: AssetImportOptions,
  pub hash: AssetHash,
}

/// Options used to import an asset from disk.
#[derive(Serialize, Deserialize)]
pub struct AssetImportOptions {}

/// A manifest of assets.
///
/// Manifests are used to describe the contents of an asset bundle. They are
/// used by the [`AssetServer`] to determine which assets are contained in a
/// bundle, and to determine whether a bundle needs to be rebuilt.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AssetManifest {
  assets: HashSet<AssetManifestEntry>,
}

/// A single entry in an [`AssetManifest`].
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct AssetManifestEntry {
  id: AssetId,
  path: String,
  hash: AssetHash,
}

impl AssetManifestEntry {
  pub fn new(path: String) -> Self {
    // TODO: load from database
    Self {
      id: AssetId::random(),
      path,
      hash: AssetHash::default(),
    }
  }
}

/// A builder pattern for [`AssetManifest`]s.
#[must_use]
pub struct AssetManifestBuilder {
  assets: HashSet<String>,
}

impl AssetManifestBuilder {
  pub fn new() -> Self {
    Self { assets: HashSet::new() }
  }

  pub fn add_asset(mut self, path: &str) -> Self {
    self.assets.insert(path.to_string());
    self
  }

  pub fn build(self) -> AssetManifest {
    AssetManifest {
      assets: self.assets.into_iter().map(|path| AssetManifestEntry::new(path)).collect(),
    }
  }
}

/// A '.pak' file; a compressed bundle of assets.
///
/// This file format is a binary encoded stream of assets, efficiently packed
/// for distribution. The format is designed to be read-only, and is not intended
/// to be modified at runtime.
#[derive(Default)]
pub struct PAK;

impl AssetBundle for PAK {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn asset_database_should_build_a_simple_asset_bundle() {
    let manifest = AssetManifestBuilder::new()
      .add_asset("assets://textures/floor.png")
      .add_asset("assets://textures/wall.png")
      .add_asset("assets://sounds/music.ogg")
      .build();

    AssetDatabase::build_asset_bundle::<PAK>("./test.yaml", &manifest).unwrap();
  }

  #[test]
  fn asset_manifest_should_serialize_to_yaml() {
    let manifest = AssetManifestBuilder::new()
      .add_asset("assets://textures/floor.png")
      .add_asset("assets://textures/wall.png")
      .add_asset("assets://sounds/music.ogg")
      .build();

    println!("{}", surreal::io::Serializable::to_yaml(&manifest).unwrap());
  }
}
