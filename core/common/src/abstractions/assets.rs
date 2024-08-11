use std::{
  fmt::{Debug, Formatter},
  ops::Deref,
  sync::Arc,
};

use macros::Singleton;

use crate::{BlockableFuture, FastHashMap, FromStream, Guid, InputStream, OutputStream, ToVirtualPath, VirtualPath};

/// An error that can occur when loading an asset
#[derive(Debug)]
pub enum AssetError {
  InvalidId,
  NotFound,
  LoadFailed,
  TypeMismatch,
}

/// Represents a database that can load and save assets.
#[derive(Singleton)]
pub struct AssetDatabase {
  base_path: VirtualPath,
  asset_map: AssetMetadataMap,
}

impl Default for AssetDatabase {
  fn default() -> Self {
    Self {
      base_path: VirtualPath::new("local://assets"),
      asset_map: AssetMetadataMap::default(),
    }
  }
}

/// A map of assets.
#[derive(Default)]
struct AssetMetadataMap {
  by_guids: FastHashMap<Guid, Arc<AssetMetadata>>,
  by_keys: FastHashMap<String, Arc<AssetMetadata>>,
  by_paths: FastHashMap<VirtualPath, Arc<AssetMetadata>>,
}

impl AssetMetadataMap {
  /// Inserts an asset into the map.
  pub fn insert(&mut self, metadata: AssetMetadata) {
    let metadata = Arc::new(metadata);

    self.by_guids.insert(metadata.guid, metadata.clone());
    self.by_keys.insert(metadata.key.clone(), metadata.clone());
    self.by_paths.insert(metadata.path.clone(), metadata);
  }

  /// Resolves an asset by its ID.
  pub fn resolve(&self, id: &AssetId) -> Option<&AssetMetadata> {
    match id {
      AssetId::None => None,
      AssetId::Guid(guid) => self.by_guids.get(guid).map(|arc| arc.as_ref()),
      AssetId::Key(key) => self.by_keys.get(key).map(|arc| arc.as_ref()),
      AssetId::Path(path) => self.by_paths.get(path).map(|arc| arc.as_ref()),
    }
  }
}

/// Metadata for an asset.
#[derive(Clone)]
struct AssetMetadata {
  guid: Guid,
  key: String,
  path: VirtualPath,
}

impl AssetDatabase {
  /// Returns the instance of the asset database.
  pub fn read_asset(&self, id: &AssetId) -> Result<Box<dyn InputStream>, AssetError> {
    if let Some(metadata) = self.asset_map.resolve(id) {
      metadata.path.open_input_stream().map_err(|_| AssetError::LoadFailed)
    } else {
      Err(AssetError::NotFound)
    }
  }
}

/// A codec for encoding and decoding assets.
pub trait AssetDecoder<A: Sized> {
  fn decode(stream: &mut dyn InputStream) -> Result<A, AssetError> {
    Self::decode_async(stream).block()
  }

  async fn decode_async(stream: &mut dyn InputStream) -> Result<A, AssetError>;
}

/// Represents an asset that can be loaded and resolved.
pub trait Asset: Sized {
  type Decoder: AssetDecoder<Self>;

  fn from_id(id: &AssetId) -> Result<Self, AssetError> {
    Self::from_id_async(id).block()
  }

  async fn from_id_async(id: &AssetId) -> Result<Self, AssetError> {
    let mut stream = AssetDatabase::instance().read_asset(id)?;

    Self::Decoder::decode_async(stream.as_mut()).await
  }

  fn from_guid(guid: Guid) -> Result<Self, AssetError> {
    Self::from_guid_async(guid).block()
  }

  async fn from_guid_async(guid: Guid) -> Result<Self, AssetError> {
    Self::from_id_async(&AssetId::Guid(guid)).await
  }

  fn from_key(key: impl AsRef<str>) -> Result<Self, AssetError> {
    Self::from_key_async(key).block()
  }

  async fn from_key_async(key: impl AsRef<str>) -> Result<Self, AssetError> {
    Self::from_id_async(&AssetId::Key(key.as_ref().to_string())).await
  }

  fn from_path(path: impl ToVirtualPath) -> Result<Self, AssetError> {
    Self::from_path_async(path).block()
  }

  async fn from_path_async(path: impl ToVirtualPath) -> Result<Self, AssetError> {
    Self::from_id_async(&AssetId::Path(path.to_virtual_path())).await
  }
}

/// Represents an asset on the virtual file system.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct AssetRef<T> {
  id: AssetId,
  _marker: std::marker::PhantomData<T>,
}

impl<T> Default for AssetRef<T> {
  fn default() -> Self {
    Self {
      id: AssetId::None,
      _marker: std::marker::PhantomData,
    }
  }
}

/// Possible means of identifying an asset.
#[derive(Default, Clone, Debug, Eq, PartialEq, Hash)]
pub enum AssetId {
  #[default]
  None,
  Guid(Guid),
  Key(String),
  Path(VirtualPath),
}

impl<A: Asset> AssetRef<A> {
  /// Creates an asset from a GUID.
  #[inline]
  pub fn from_id(id: Guid) -> Self {
    Self {
      id: AssetId::Guid(id),
      _marker: std::marker::PhantomData,
    }
  }

  /// Creates an asset from a key.
  #[inline]
  pub fn from_key(key: impl AsRef<str>) -> Self {
    Self {
      id: AssetId::Key(key.as_ref().to_string()),
      _marker: std::marker::PhantomData,
    }
  }

  /// Creates an asset from a virtual path.
  #[inline]
  pub fn from_path(path: impl ToVirtualPath) -> Self {
    Self {
      id: AssetId::Path(path.to_virtual_path()),
      _marker: std::marker::PhantomData,
    }
  }

  /// Resolves the asset from the asset manager.
  pub fn resolve(&self) -> Result<A, AssetError> {
    A::from_id(&self.id)
  }
}

impl<T> Debug for AssetRef<T> {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    write!(formatter, "{:?}", self.id)
  }
}

impl<A: FromStream> Asset for A {
  type Decoder = Self;
}

impl<A: FromStream> AssetDecoder<A> for A {
  async fn decode_async(stream: &mut dyn InputStream) -> Result<A, AssetError> {
    A::from_stream_async(stream).await.map_err(|_| AssetError::LoadFailed)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::FromRandom;

  struct TestAsset;

  impl FromStream for TestAsset {
    async fn from_stream_async(stream: &mut dyn InputStream) -> Result<Self, Self::Error> {
      todo!()
    }
  }

  #[test]
  fn test_asset_operations() {
    let asset = TestAsset::from_guid_async(Guid::random()).block().unwrap();
  }
}
