use std::ops::{Deref, DerefMut};

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
enum AssetId {
  None,
  Name(StringName),
  Path(VirtualPath),
  Guid(Guid),
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
  pub const fn from_name(name: StringName) -> Self {
    Self {
      asset_id: AssetId::Name(name),
      _marker: std::marker::PhantomData,
    }
  }

  /// Creates a new asset reference from a file path.
  pub const fn from_path(path: VirtualPath) -> Self {
    Self {
      asset_id: AssetId::Path(path),
      _marker: std::marker::PhantomData,
    }
  }

  /// Creates a new asset reference from a GUID.
  pub const fn from_guid(guid: Guid) -> Self {
    Self {
      asset_id: AssetId::Guid(guid),
      _marker: std::marker::PhantomData,
    }
  }

  /// Determines whether the asset reference is valid.
  pub fn is_valid(&self) -> bool {
    !matches!(self.asset_id, AssetId::None)
  }
}

impl<T> Deref for AssetRef<T> {
  type Target = T;

  /// Dereferences the asset reference to retrieve the underlying asset data.
  fn deref(&self) -> &T {
    if matches!(self.asset_id, AssetId::None) {
      panic!("Attempted to dereference an unloaded asset")
    } else {
      todo!()
    }
  }
}

impl<T> DerefMut for AssetRef<T> {
  /// Mutably dereferences the asset reference to retrieve the underlying asset
  /// data.
  fn deref_mut(&mut self) -> &mut T {
    if matches!(self.asset_id, AssetId::None) {
      panic!("Attempted to dereference an unloaded asset")
    } else {
      todo!()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{FromRandom, ToStringName, ToVirtualPath};

  struct Sprite {}

  #[test]
  fn asset_ref_should_construct() {
    let _sprite = AssetRef::<Sprite>::from_path("local://sprites/test.png".to_virtual_path());
    let _sprite = AssetRef::<Sprite>::from_name("hero_1".to_string_name());
    let _sprite = AssetRef::<Sprite>::from_guid(Guid::random());
  }
}
