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

/// Internal asset data.
struct AssetData {}

/// The underlying asset identifier.
///
/// If the asset is not loaded, the asset identifier will be `None`, and
/// attempting to de-reference the asset reference will panic.
#[derive(Clone, Debug)]
pub enum AssetId {
  None,
  Name(StringName),
  Path(VirtualPath),
  Guid(Guid),
}

/// A server capable of loading and unloading assets.
pub trait AssetServer {
  /// Resolves the asset data for the given asset identifier.
  fn resolve(&self, asset_id: &AssetId) -> impl Future<Output = Option<&AssetData>>;

  /// Mutably resolves the asset data for the given asset identifier.
  fn resolve_mut(&self, asset_id: &AssetId) -> impl Future<Output = Option<&mut AssetData>>;
}

impl<T> Default for AssetRef<T> {
  fn default() -> Self {
    Self {
      asset_id: AssetId::None,
      _marker: std::marker::PhantomData,
    }
  }
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

  /// Determines whether the asset reference is valid.
  pub fn is_valid(&self) -> bool {
    !matches!(self.asset_id, AssetId::None)
  }

  /// Attempts to get a reference to the asset data.
  pub async fn get(&self, server: &impl AssetServer) -> Option<&T> {
    server.resolve(&self.asset_id).await.map(|data| unsafe {
      // TODO: find a way to do this without unsafe
      &*(data as *const AssetData as *const T)
    })
  }

  /// Attempts to get a mutable reference to the asset data.
  pub async fn get_mut(&mut self, server: &impl AssetServer) -> Option<&mut T> {
    server.resolve_mut(&self.asset_id).await.map(|data| unsafe {
      // TODO: find a way to do this without unsafe
      &mut *(data as *mut AssetData as *mut T)
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Sprite {}

  struct AssetDatabase {}

  impl AssetServer for AssetDatabase {
    async fn resolve(&self, _asset_id: &AssetId) -> Option<&AssetData> {
      None
    }

    async fn resolve_mut(&self, _asset_id: &AssetId) -> Option<&mut AssetData> {
      None
    }
  }

  #[test]
  fn asset_ref_should_construct() {
    use crate::BlockableFuture;

    let server = AssetDatabase {};
    let player_sprite = AssetRef::<Sprite>::from_path("local://sprites/player.png");
    let sprite = player_sprite.get(&server).block();

    assert!(sprite.is_none());
  }
}
