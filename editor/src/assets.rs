//! Asset server, database and build pipelines for use in editor tooling.
//!
//! The asset system maintains a flat-file database of all assets in the project,
//! with support for transient imports via [`AssetImporter`] behaviours. This
//! allows projects to participate in the asset system by writing data in bespoke
//! formats and importing them into a central system.
//!
//! The output of the asset database is put through a build pipeline to produce
//! artifacts and [`AssetBundle`]s for consumption by the game at runtime.

use std::any::Any;
use std::collections::{BTreeSet, HashMap};
use std::fmt::Debug;
use std::hash::Hasher;

use serde::{Deserialize, Serialize};
use surreal::io::{InputStream, Serializable, VirtualPath};
use surreal::utilities::Type;

use super::Resource;

surreal::impl_guid!(AssetId);

/// A database for assets in a project.
///
/// The asset database is responsible for maintaining a flat-file database of all
/// assets in the project, and for managing the import of assets from the file system.
///
/// See [`AssetImporter`] and [`AssetBundle`] for more details.
#[derive(Default)]
pub struct AssetDatabase {
  _root_path: String,
  manifest: AssetManifest,
  metadata: HashMap<String, AssetMetadata>,
  importers: Vec<Box<dyn AssetImporter>>,
  pending_changes: Vec<AssetDatabaseChange>,
}

/// A change to the asset database.
enum AssetDatabaseChange {
  /// Creates [`AssetMetadata`] for a file at a given path.
  CreateMetadata(String, AssetMetadata),
}

impl AssetDatabase {
  /// Builds an [`AssetDatabase`] from the given root project path.
  pub fn new(path: impl AsRef<str>) -> Self {
    Self {
      _root_path: path.as_ref().to_owned(),
      manifest: AssetManifest::default(),
      metadata: HashMap::new(),
      importers: Vec::new(),
      pending_changes: Vec::new(),
    }
  }

  /// Builds an [`AssetDatabase`] from the given [`AssetManifest`].
  pub fn from_manifest(root_path: impl AsRef<str>, manifest: impl Into<AssetManifest>) -> Self {
    let mut database = Self::new(root_path);
    database.manifest = manifest.into();

    for path in &database.manifest.assets {
      let metadata = AssetMetadata {
        id: AssetId::random(),
        hash: AssetHash::default(),
        assets: Vec::default(),
      };

      let path = VirtualPath::from(path);
      let path = path.change_extension("meta");

      database.pending_changes.push(AssetDatabaseChange::CreateMetadata(path.to_string(), metadata));
    }

    database
  }

  /// Returns the [`AssetManifest`] for the entire database.
  pub fn manifest(&self) -> &AssetManifest {
    &self.manifest
  }

  /// Adds an [`AssetImporter`] with the database.
  pub fn add_importer(&mut self, importer: impl AssetImporter + 'static) {
    self.importers.push(Box::new(importer));
  }

  /// Loads a [`Box`]ed [`Asset`] of the given type from the given [`VirtualPath`].
  pub fn load_asset_boxed<A: Asset>(&mut self, path: impl Into<VirtualPath>) -> surreal::Result<Box<A>> {
    let path = path.into();

    for importer in &self.importers {
      if importer.can_handle(&path) {
        let asset = importer.import(&path)?;
        let asset = asset.into_any().downcast().expect("Failed to downcast asset to expected type");

        return Ok(asset);
      }
    }

    Err(surreal::anyhow!("Asset cannot be imported at path '{}'", path))
  }

  /// Loads an [`Asset`] of the given type from the given [`VirtualPath`].
  pub fn load_asset<A: Asset>(&mut self, path: impl Into<VirtualPath>) -> surreal::Result<A> {
    let boxed = self.load_asset_boxed(path)?;
    let asset = Box::into_inner(boxed);

    Ok(asset)
  }

  /// Creates an [`AssetHash`] for the [`Asset`] at the given [`VirtualPath`] and remembers it.
  pub fn rehash(&mut self, path: impl Into<VirtualPath>) -> surreal::Result<AssetHash> {
    let path = path.into();

    let mut stream = path.open_input_stream()?;
    let hash = AssetHash::from_stream(&mut stream)?;

    self.metadata.entry(path.to_string()).and_modify(|entry| {
      entry.hash = hash;
    });

    Ok(hash)
  }

  /// Saves any pending changes out to disk.
  pub fn flush_changes(&mut self) -> surreal::Result<()> {
    while let Some(change) = self.pending_changes.pop() {
      match change {
        AssetDatabaseChange::CreateMetadata(path, metadata) => {
          metadata.to_yaml_file(VirtualPath::from(&path))?
        }
      }
    }

    Ok(())
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

  /// Imports the asset at the given path.
  fn import(&self, path: &VirtualPath) -> surreal::Result<Box<dyn Asset>>;
}

// TODO: import into in-memory cache, and expand on the file system
// TODO: use object ids to look-up instances depending on access patterns

/// Represents an asset type that can be imported by an [`AssetImporter`].
pub trait Asset: Any {
  /// Converts this asset to a [`Box`] of [`Any`].
  fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

/// Blanket [`Asset`] implementation for sized types.
impl<A: Any + Sized> Asset for A {
  #[inline(always)]
  fn into_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }
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
  assets: BTreeSet<String>,
}

/// A builder pattern for [`AssetManifest`]s.
#[must_use]
#[derive(Default)]
pub struct AssetManifestBuilder {
  manifest: AssetManifest,
}

impl AssetManifestBuilder {
  /// Determines if the given path can be added to the [`AssetManifest`].
  pub fn can_import(&self, path: &VirtualPath) -> bool {
    path.extension() != "meta"
  }

  /// Adds an existing asset to the manifest.
  pub fn add_asset(mut self, path: impl Into<VirtualPath>) -> Self {
    let path = path.into();

    if self.can_import(&path) {
      self.manifest.assets.insert(path.to_string());
    }

    self
  }

  /// Adds all assets that match the given pattern to the manifest.
  pub fn add_assets(mut self, pattern: &str) -> Self {
    let options = glob::MatchOptions {
      case_sensitive: false,
      require_literal_separator: false,
      require_literal_leading_dot: false,
    };

    if let Ok(paths) = glob::glob_with(pattern, options) {
      for path in paths {
        match path {
          Ok(path) if path.is_file() => {
            self = self.add_asset(&VirtualPath::from(path.to_str().unwrap()).to_string());
          }
          Ok(_) => {}
          Err(_) => {}
        }
      }
    }

    self
  }

  /// Builds the resultant [`AssetManifest`].
  pub fn build(self) -> AssetManifest {
    self.manifest
  }
}

impl From<AssetManifestBuilder> for AssetManifest {
  fn from(value: AssetManifestBuilder) -> Self {
    value.build()
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
  use surreal::io::Deserializable;
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
