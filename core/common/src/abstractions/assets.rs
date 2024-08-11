use std::fmt::{Debug, Formatter};

use crate::{Graph, GraphNodeId, Guid, ToVirtualPath, VirtualPath};

/// An error that can occur when loading an asset
#[derive(Debug)]
pub enum AssetError {
  NotFound,
  LoadFailed,
  TypeMismatch,
}

/// A context for resolving [`Asset`]s.
pub struct AssetContext {
  /// The current asset being loaded.
  current_node: GraphNodeId,
  /// The graph of dependencies from the root asset being loaded.
  ///
  /// N.B: Some or portions of this graph might already be resolved, depending
  /// on the order of loading.
  dependencies: Graph<AssetId>,
}

/// Represents an asset that can be loaded and resolved.
pub trait Asset {
  /// Resolves the dependencies of the asset.
  fn resolve_dependencies(&self, context: &mut AssetContext);
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
enum AssetId {
  #[default]
  None,
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

  /// Resolves the asset from the asset manager.
  pub fn resolve(&self) -> Result<T, AssetError> {
    todo!()
  }
}

impl<T> Debug for AssetRef<T> {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    write!(formatter, "{:?}", self.id)
  }
}

impl<T> Asset for AssetRef<T> {
  fn resolve_dependencies(&self, context: &mut AssetContext) {
    let node = context.dependencies.add_node(self.id.clone());

    context.dependencies.add_edge(context.current_node, node, 1.0);
  }
}
