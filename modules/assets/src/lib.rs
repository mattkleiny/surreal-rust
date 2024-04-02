//! Asset management for Surreal.

use std::any::{Any, TypeId};

use common::{Arena, FastHashMap, FileSystemError, ToVirtualPath};
pub use exporters::*;
pub use importers::*;

mod exporters;
mod importers;

common::impl_arena_index!(AssetId, "Identifies an asset in an asset database.");

/// A database for managing assets.
///
/// This struct is the main interface for the asset management system. It
/// provides functionality for loading and unloading assets, and for querying
/// the state of assets.
///
/// It works on flat-file system where all assets are stored on the file system.
/// Assets are loaded from the file system when they are first requested, and
/// are unloaded when they are no longer in use, or when asked to do so.
///
/// Assets can be packed into 'Asset Bundles', which are files which contain
/// many other assets via some 'Asset Bundle Protocol'. Beyond being a composite
/// file, behaviour is identical to the flat-file system.
///
/// Assets are identified by:
/// - A 'path', which is the full path of the asset in the virtual file system.
/// - A 'key', which is the name of the asset within the asset bundle.
/// - A 'guid', which is a globally unique identifier for the asset.
#[derive(Default)]
pub struct AssetDatabase {
  // core asset storage
  assets: Arena<AssetId, AssetState>,

  // lookup tables
  _assets_by_path: FastHashMap<String, AssetId>,
  _assets_by_key: FastHashMap<String, AssetId>,
  _assets_by_guid: FastHashMap<String, AssetId>,

  // importers/exporters
  importers: Vec<Box<dyn UntypedAssetImporter>>,
  exporters: Vec<Box<dyn UntypedAssetExporter>>,
}

/// A possible error when working with the asset database.
#[derive(Debug)]
pub enum AssetDatabaseError {
  InvalidPath,
  InvalidVersion,
  NoImporterFound,
  NoExporterFound,
  FileSystemError(FileSystemError),
  FailedToImport(AssetImportError),
  FailedToExport(AssetExportError),
}

impl From<FileSystemError> for AssetDatabaseError {
  fn from(error: FileSystemError) -> Self {
    Self::FileSystemError(error)
  }
}

impl From<AssetImportError> for AssetDatabaseError {
  fn from(error: AssetImportError) -> Self {
    Self::FailedToImport(error)
  }
}

impl From<AssetExportError> for AssetDatabaseError {
  fn from(error: AssetExportError) -> Self {
    Self::FailedToExport(error)
  }
}

/// Represents the internal state of an asset.
pub enum AssetState {
  Unloaded,
  Orphaned,
  Loaded(Box<dyn Any>),
}

impl AssetDatabase {
  /// Opens the asset database at the given path.
  pub fn open(_path: impl ToVirtualPath) -> Result<Self, AssetDatabaseError> {
    // TODO: make this actually open the database
    Ok(Self::default())
  }

  /// Adds an importer to the database.
  pub fn add_importer(&mut self, importer: impl UntypedAssetImporter + 'static) {
    self.importers.push(Box::new(importer));
  }

  /// Adds an exporter to the database.
  pub fn add_exporter(&mut self, exporter: impl UntypedAssetExporter + 'static) {
    self.exporters.push(Box::new(exporter));
  }

  /// Gets an asset from the database, or loads it from the file system.
  pub fn load<A: 'static>(&mut self, path: impl ToVirtualPath) -> Result<Asset<A>, AssetDatabaseError> {
    let path = path.to_virtual_path();

    for importer in &self.importers {
      if importer.can_import(TypeId::of::<A>(), path.clone()) {
        let mut stream = path.open_input_stream()?;

        let asset = importer.import(&mut stream)?;
        let asset_id = self.assets.insert(AssetState::Loaded(asset));

        return Ok(Asset::from_id(asset_id));
      }
    }

    Err(AssetDatabaseError::NoImporterFound)
  }

  /// Exports an asset to the file system.
  pub fn export<A: 'static>(&mut self, asset: &A, path: impl ToVirtualPath) -> Result<(), AssetDatabaseError> {
    let path = path.to_virtual_path();

    for exporter in &self.exporters {
      if exporter.can_export(TypeId::of::<A>(), path.clone()) {
        let mut stream = path.open_output_stream()?;

        exporter.export(asset, &mut stream)?;

        return Ok(());
      }
    }

    Err(AssetDatabaseError::NoExporterFound)
  }
}

/// A reference to an asset in the database.
///
/// This struct is a reference to an asset in the database. It is used to
/// reference an asset, and to query the state of the asset.
///
/// Note that a reference to an asset does not guarantee that the asset is
/// loaded. If the asset is not loaded, then the reference will be invalid.
///
/// This struct is a 'thin' wrapper around the asset, and is cheap to copy.
pub struct Asset<A> {
  id: AssetId,
  kind: std::marker::PhantomData<A>,
}

impl<A> Asset<A> {
  /// Creates a new asset reference from an asset ID.
  pub const fn from_id(id: AssetId) -> Self {
    Self {
      id,
      kind: std::marker::PhantomData,
    }
  }
}

impl<A> std::ops::Deref for Asset<A> {
  type Target = A;

  fn deref(&self) -> &Self::Target {
    todo!()
  }
}

/// A bundle of assets.
///
/// This struct represents a bundle of assets. It is used to load and unload
/// assets from an archive or compressed means.
///
/// Bundles are hierarchical, and can contain other bundles, and the top-level
/// flat file system is a kind of 'bundle' which contains all other bundles.
pub struct AssetBundle {}

/// A trait for asset bundle codecs.
///
/// This trait is implemented by asset bundle codecs, which are responsible
/// for packing assets into asset bundles, and unpacking them later.
pub trait AssetBundleCodec {}
