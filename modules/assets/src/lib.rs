//! Asset management for Surreal.

use std::any::Any;

pub use exporters::*;
pub use importers::*;

mod exporters;
mod importers;

// TODO: add a small reflection system for reading/writing resource data
// TODO: add a small UI toolkit for building the editor; for each field render

use common::{FastHashMap, ResourceArena, ToVirtualPath};

common::impl_rid!(AssetId, "Identifies an asset in an asset database.");

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
  _assets: ResourceArena<AssetId, AssetState>,

  // lookup tables
  _assets_by_path: FastHashMap<String, AssetId>,
  _assets_by_key: FastHashMap<String, AssetId>,
  _assets_by_guid: FastHashMap<String, AssetId>,
}

/// Represents the internal state of an asset.
pub enum AssetState {
  Unloaded,
  Loaded(Box<dyn Any>),
  Orphaned,
}

impl AssetDatabase {
  /// Gets an asset from the database, or loads it from the file system.
  pub fn load<'a, A>(&self, _path: impl ToVirtualPath) -> Asset<A> {
    todo!()
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
  _id: AssetId,
  _database: *mut AssetDatabase,
  _kind: std::marker::PhantomData<A>,
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
