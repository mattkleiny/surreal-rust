//! Asset management for Surreal.

use std::{
  any::{Any, TypeId},
  future::Future,
};

pub use exporters::*;
pub use importers::*;

use crate::{FastHashMap, FileSystemError, Guid, InputStream, StreamError, StringName, ToVirtualPath, VirtualPath};

/// Represents a reference to an asset that can either be loaded or unloaded.
///
/// The asset reference is a zero-cost abstraction that is used to reference
/// assets in a way that allows the asset to be loaded and unloaded without the
/// need to change the reference.
///
/// In order to retrieve the underlying asset data, the asset reference must be
/// de-referenced. This will either return a reference to the asset data if the
/// asset is loaded, or panic if the asset is not loaded.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Asset<T> {
  asset_id: AssetId,
  _marker: std::marker::PhantomData<T>,
}

/// The underlying asset identifier.
///
/// If the asset is not loaded, the asset identifier will be `None`, and
/// attempting to de-reference the asset reference will panic.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AssetId {
  Path(VirtualPath),
  Name(StringName),
  Guid(Guid),
}

/// Represents the internal state of an asset.
pub enum AssetState {
  Unloaded,
  Orphaned,
  Loaded(Box<dyn Any>),
}

impl<T> Asset<T> {
  /// Creates a new asset reference from a name.
  pub fn from_name(name: impl Into<StringName>) -> Self {
    Self {
      asset_id: AssetId::Name(name.into()),
      _marker: std::marker::PhantomData,
    }
  }

  /// Creates a new asset reference from a file path.
  pub fn from_path(path: impl Into<VirtualPath>) -> Self {
    Self {
      asset_id: AssetId::Path(path.into()),
      _marker: std::marker::PhantomData,
    }
  }

  /// Creates a new asset reference from a GUID.
  pub fn from_guid(guid: Guid) -> Self {
    Self {
      asset_id: AssetId::Guid(guid),
      _marker: std::marker::PhantomData,
    }
  }

  /// Attempts to get a reference to the asset data.
  pub async fn get(&self, server: &impl AssetServer) -> Option<&T> {
    server.resolve(&self.asset_id).await.map(|_data| todo!())
  }
}

/// A server capable of loading and unloading assets.
pub trait AssetServer {
  /// Resolves the asset data for the given asset identifier.
  fn resolve(&self, id: &AssetId) -> impl Future<Output = Option<&AssetState>>;
}

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
/// - A 'name', which is the name of the asset within the asset bundle.
/// - A 'guid', which is a globally unique identifier for the asset.
#[derive(Default)]
pub struct AssetDatabase {
  // core asset storage
  assets: FastHashMap<AssetId, AssetState>,

  // lookup tables
  _assets_by_path: FastHashMap<String, AssetId>,
  _assets_by_name: FastHashMap<StringName, AssetId>,
  _assets_by_guid: FastHashMap<Guid, AssetId>,

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

impl AssetDatabase {
  /// Opens the asset database at the given path.
  pub fn open(_path: impl ToVirtualPath) -> Result<Self, AssetDatabaseError> {
    // TODO: make this actually open the database
    Ok(Self::default())
  }

  /// Adds an importer to the database.
  pub fn add_importer(&mut self, importer: Box<dyn UntypedAssetImporter>) {
    self.importers.push(importer);
  }

  /// Adds an exporter to the database.
  pub fn add_exporter(&mut self, exporter: Box<dyn UntypedAssetExporter>) {
    self.exporters.push(exporter);
  }

  /// Gets an asset from the database, or loads it from the file system.
  pub fn load<A: 'static>(&mut self, path: impl ToVirtualPath) -> Result<Asset<A>, AssetDatabaseError> {
    let path = path.to_virtual_path();
    let type_id = TypeId::of::<A>();

    for importer in &self.importers {
      if importer.can_import(type_id, path.clone()) {
        let mut stream = path.open_input_stream()?;

        let asset = importer.import(&mut stream)?;
        let asset_id = AssetId::Path(path.clone());
        let _asset_state = self.assets.insert(asset_id, AssetState::Loaded(asset));

        return Ok(Asset::from_path(path));
      }
    }

    Err(AssetDatabaseError::NoImporterFound)
  }

  /// Exports an asset to the file system.
  pub fn export<A: 'static>(&mut self, asset: &A, path: impl ToVirtualPath) -> Result<(), AssetDatabaseError> {
    let path = path.to_virtual_path();
    let type_id = TypeId::of::<A>();

    for exporter in &self.exporters {
      if exporter.can_export(type_id, path.clone()) {
        let mut stream = path.open_output_stream()?;

        exporter.export(asset, &mut stream)?;

        return Ok(());
      }
    }

    Err(AssetDatabaseError::NoExporterFound)
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

mod exporters {
  use super::*;
  use crate::OutputStream;

  /// An error that can occur when exporting an asset.
  #[derive(Debug)]
  pub enum AssetExportError {
    FileSystemError(FileSystemError),
    StreamError(StreamError),
  }

  impl From<FileSystemError> for AssetExportError {
    fn from(error: FileSystemError) -> Self {
      Self::FileSystemError(error)
    }
  }

  impl From<StreamError> for AssetExportError {
    fn from(error: StreamError) -> Self {
      Self::StreamError(error)
    }
  }

  /// Exports assets to a specific format.
  pub trait AssetExporter: Send + Sync + 'static {
    type Asset: ?Sized;

    /// Returns whether this export can export the given asset type and path.
    fn can_export(&self, _path: VirtualPath) -> bool {
      true
    }

    /// Exports an asset to the given stream.
    fn export(&self, asset: &Self::Asset, stream: &mut dyn OutputStream) -> Result<(), AssetExportError>;
  }

  /// A trait for exporting assets without knowing the asset type.
  pub trait UntypedAssetExporter: Send + Sync + 'static {
    fn can_export(&self, asset_type: TypeId, path: VirtualPath) -> bool;
    fn export(&self, asset: &dyn Any, stream: &mut dyn OutputStream) -> Result<(), AssetExportError>;
  }

  /// Allow any typed asset exporter to be used as an untyped asset exporter.
  impl<A: 'static, T: AssetExporter<Asset = A>> UntypedAssetExporter for T {
    fn can_export(&self, asset_type: TypeId, path: VirtualPath) -> bool {
      asset_type == TypeId::of::<A>() && self.can_export(path)
    }

    fn export(&self, asset: &dyn Any, stream: &mut dyn OutputStream) -> Result<(), AssetExportError> {
      self.export(asset.downcast_ref::<A>().unwrap(), stream)
    }
  }
}

mod importers {
  use super::*;

  /// An error that can occur when exporting an asset.
  #[derive(Debug)]
  pub enum AssetImportError {
    FileSystemError(FileSystemError),
    StreamError(StreamError),
  }

  impl From<FileSystemError> for AssetImportError {
    fn from(error: FileSystemError) -> Self {
      Self::FileSystemError(error)
    }
  }

  impl From<StreamError> for AssetImportError {
    fn from(error: StreamError) -> Self {
      Self::StreamError(error)
    }
  }

  /// Imports assets from a specific format.
  pub trait AssetImporter: Send + Sync + 'static {
    type Asset: Sized;

    /// Returns whether this importer can import the given asset type and path.
    fn can_import(&self, _path: VirtualPath) -> bool {
      true
    }

    /// Imports an asset from the given stream.
    fn import(&self, stream: &mut dyn InputStream) -> Result<Self::Asset, AssetImportError>;
  }

  /// A trait for importing assets without knowing the asset type.
  pub trait UntypedAssetImporter: Send + Sync + 'static {
    fn can_import(&self, asset_type: TypeId, path: VirtualPath) -> bool;
    fn import(&self, stream: &mut dyn InputStream) -> Result<Box<dyn Any>, AssetImportError>;
  }

  /// Allow any typed asset importer to be used as an untyped asset importer.
  impl<A: 'static, T: AssetImporter<Asset = A>> UntypedAssetImporter for T {
    fn can_import(&self, asset_type: TypeId, path: VirtualPath) -> bool {
      asset_type == TypeId::of::<A>() && self.can_import(path)
    }

    fn import(&self, stream: &mut dyn InputStream) -> Result<Box<dyn Any>, AssetImportError> {
      Ok(Box::new(self.import(stream)?))
    }
  }
}
