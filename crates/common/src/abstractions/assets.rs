//! Asset management for the engine

use std::fmt::{Debug, Formatter};

use macros::Asset;

use crate::{Guid, StringName, ToVirtualPath, VirtualPath};

/// An error that can occur when loading an asset
#[derive(Debug)]
pub enum AssetError {
  NotFound,
  LoadFailed,
  TypeMismatch,
}

/// A context for resolving [`Asset`]s.
pub struct AssetContext {}

/// Represents an asset that can be loaded and resolved.
pub trait Asset {
  /// Resolves the dependencies of the asset.
  fn resolve_dependencies(&self, context: &mut AssetContext);
}

// TODO: remove this once derive macro is more sophisticated
macro_rules! impl_empty_asset {
  ($type:ty) => {
    impl Asset for $type {
      #[inline(always)]
      fn resolve_dependencies(&self, _context: &mut AssetContext) {
        // no-op
      }
    }
  };
}

impl_empty_asset!(());
impl_empty_asset!(bool);
impl_empty_asset!(u8);
impl_empty_asset!(u16);
impl_empty_asset!(u32);
impl_empty_asset!(u64);
impl_empty_asset!(i8);
impl_empty_asset!(i16);
impl_empty_asset!(i32);
impl_empty_asset!(i64);
impl_empty_asset!(f32);
impl_empty_asset!(f64);
impl_empty_asset!(String);
impl_empty_asset!(StringName);
impl_empty_asset!(Guid);
impl_empty_asset!(VirtualPath);

impl<T> Asset for AssetRef<T> {
  fn resolve_dependencies(&self, _context: &mut AssetContext) {
    todo!()
  }
}

/// Represents an asset on the virtual file system.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct AssetRef<T> {
  id: AssetId,
  _marker: std::marker::PhantomData<T>,
}

/// Possible means of identifying an asset.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum AssetId {
  Id(Guid),
  Key(String),
  Path(VirtualPath),
}

impl<T> AssetRef<T> {
  /// Creates an asset from a GUID.
  #[inline]
  pub fn from_id(id: Guid) -> Self {
    Self {
      id: AssetId::Id(id),
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
}

impl<T> Debug for AssetRef<T> {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    write!(formatter, "{:?}", self.id)
  }
}

#[derive(Asset)]
pub struct TestAsset {
  pub id: Guid,
  pub name: String,
  pub example: AssetRef<TestAsset>,
}
