//! A simple asset management system with support for hot file reloading.

use std::any::{Any, TypeId};
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use crate::io::{AsVirtualPath, VirtualPath};

// TODO: use RefCell for this instead and return asset handles to callers to manage lifetimes.
// TODO: asset hot loading and dependent asset reloads (shader program includes, for example)
// TODO: cache assets between invocations?

/// Allows loading an asset from the virtual file system.
pub trait AssetLoader {
  type Output;

  /// Loads the asset from the given path.
  fn load(&self, context: &AssetContext) -> crate::Result<Self::Output>;
}

/// A pointer to an asset loader in our asset manager.
///
/// This is a hacky redirection to allow downcasting to the concrete asset loader type.
struct AssetLoaderPtr<A> {
  loader: Box<dyn AssetLoader<Output = A>>,
}

impl<A> Deref for AssetLoaderPtr<A> {
  type Target = dyn AssetLoader<Output = A>;

  fn deref(&self) -> &Self::Target {
    self.loader.deref()
  }
}

/// An id for an asset in the asset manager.
#[derive(Clone, Eq, PartialEq, Hash)]
struct AssetId {
  path: String,
  type_id: TypeId,
}

/// A context for asset operations.
pub struct AssetContext<'a> {
  /// The path of the asset being loaded.
  pub path: VirtualPath<'a>,
  manager: &'a AssetManager,
}

impl<'a> AssetContext<'a> {
  /// Loads a dependent asset from the given path.
  pub fn load_asset<A: Any>(&self, path: VirtualPath) -> crate::Result<A> {
    self.manager.load_asset(path)
  }
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
  pub fn add_loader<L: AssetLoader + 'static>(&mut self, loader: L) {
    let state = unsafe { &mut *self.state.get() };
    let asset_type = TypeId::of::<L::Output>();

    state.loaders.insert(
      asset_type,
      Box::new(AssetLoaderPtr {
        loader: Box::new(loader),
      }),
    );
  }

  /// Attempts to load an asset from the given path.
  pub fn load_asset<A: Any>(&self, path: impl AsVirtualPath) -> crate::Result<A> {
    log::trace!(
      "Loading asset type {} from {}",
      std::any::type_name::<A>(),
      path.as_virtual_path()
    );

    let state = unsafe { &mut *self.state.get() };
    let path = path.as_virtual_path();

    let loader = state
      .loaders
      .get(&TypeId::of::<A>())
      .and_then(|it| it.downcast_ref::<AssetLoaderPtr<A>>())
      .ok_or_else(|| {
        anyhow::anyhow!(
          "Could not result loader for asset {:?}",
          std::any::type_name::<A>()
        )
      })?;

    let context = AssetContext {
      path,
      manager: self,
    };

    loader.load(&context)
  }
}
