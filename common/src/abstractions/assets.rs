use std::future::Future;

use crate::{Guid, StringName, VirtualPath};

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
pub struct AssetRef<T> {
  asset_id: AssetId,
  _marker: std::marker::PhantomData<T>,
}

/// The underlying asset identifier.
///
/// If the asset is not loaded, the asset identifier will be `None`, and
/// attempting to de-reference the asset reference will panic.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AssetId {
  Name(StringName),
  Path(VirtualPath),
  Guid(Guid),
}

/// Internal asset data.
enum AssetData {
  Unloaded,
  Pending,
  Loaded(Box<dyn std::any::Any>),
}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for AssetRef<T> {
  fn serialize<S: serde::Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
    todo!()
  }
}

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for AssetRef<T> {
  fn deserialize<D: serde::Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
    todo!()
  }
}

/// A server capable of loading and unloading assets.
pub trait AssetServer {
  /// Resolves the asset data for the given asset identifier.
  fn resolve(&self, id: &AssetId) -> impl Future<Output = Option<&AssetData>>;
}

impl<T> AssetRef<T> {
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
    server.resolve(&self.asset_id).await.map(|data| unsafe {
      // TODO: find a way to do this without unsafe
      &*(data as *const AssetData as *const T)
    })
  }
}
