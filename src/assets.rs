//! A simple asset management system with support for hot file reloading.

use std::any::{Any, TypeId};
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::io::{AsVirtualPath, VirtualPath};

// TODO: use RefCell for this instead and return asset handles to callers to manage lifetimes.
// TODO: asset hot loading and dependent asset reloads (shader program includes, for example)
// TODO: cache assets between invocations?

/// A type of asset that can be persisted in the asset manager.
pub trait Asset: Sized + 'static {
  /// The associated loader for this asset.
  type Loader: AssetLoader<Self>;
}

/// Allows loading an asset from the virtual file system.
pub trait AssetLoader<A>: 'static {
  /// Loads the asset from the given path.
  fn load(&self, context: &AssetContext) -> crate::Result<A>;
}

/// A context for asset operations.
pub struct AssetContext<'a> {
  /// The path of the asset being loaded.
  pub path: VirtualPath<'a>,

  manager: &'a AssetManager,
}

impl<'a> AssetContext<'a> {
  /// Loads a dependent asset from the given path.
  pub fn load_asset<A: Asset>(&self, path: VirtualPath) -> crate::Result<A> {
    self.manager.load_asset(path)
  }
}

/// An id for an asset in the asset manager.
#[derive(Clone, Eq, PartialEq, Hash)]
struct AssetId {
  path: String,
  type_id: TypeId,
}

/// A manager for assets.
///
/// The manager is responsible for loading assets from the virtual file system,
/// and providing access to them.
///
/// The manager is also responsible for keeping track of asset dependencies,
/// and automatically reloading assets when they are modified.
pub struct AssetManager {
  state: Rc<UnsafeCell<AssetManagerState>>,
}

/// The internal state for the asset manager.
struct AssetManagerState {
  loaders: HashMap<TypeId, Box<dyn Any>>,
}

impl Default for AssetManager {
  fn default() -> Self {
    Self::new()
  }
}

impl AssetManager {
  /// Creates a new asset manager.
  pub fn new() -> Self {
    Self {
      state: Rc::new(UnsafeCell::new(AssetManagerState {
        loaders: HashMap::new(),
      })),
    }
  }

  /// Adds a new asset loader to the manager.
  pub fn add_loader<A: Asset, L: AssetLoader<A>>(&mut self, loader: L) {
    let state = unsafe { &mut *self.state.get() };

    state.loaders.insert(TypeId::of::<A>(), Box::new(loader));
  }

  /// Attempts to load an asset from the given path.
  pub fn load_asset<A: Asset>(&self, path: impl AsVirtualPath) -> crate::Result<A> {
    let state = unsafe { &mut *self.state.get() };
    let path = path.as_virtual_path();

    let loader = state
      .loaders
      .get(&TypeId::of::<A>())
      .and_then(|it| it.downcast_ref::<A::Loader>())
      .ok_or_else(|| anyhow::anyhow!(
        "Could not result loader for asset {:?}",
        std::any::type_name::<A>()
      ))?;

    let context = AssetContext {
      path,
      manager: self,
    };

    loader.load(&context)
  }
}
