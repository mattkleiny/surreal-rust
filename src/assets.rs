//! A simple asset management system with support for hot file reloading.

use std::any::{Any, TypeId};
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use anyhow::Ok;

use crate::io::{AsVirtualPath, VirtualPath};

// TODO: asset hot loading and dependent asset reloads (shader program includes, for example)

/// A handle to an asset in the asset system.
///
/// Handles are cheap to clone and can be passed around the application freely.
pub struct Handle<A> {
  id: AssetId,
  asset: Rc<A>,
}

impl<A> Clone for Handle<A> {
  fn clone(&self) -> Self {
    Self {
      id: self.id.clone(),
      asset: self.asset.clone(),
    }
  }
}

impl<A> Deref for Handle<A> {
  type Target = A;

  fn deref(&self) -> &Self::Target {
    self.asset.deref()
  }
}

/// A id for an asset in the asset manager cache.
#[derive(Clone, Eq, PartialEq, Hash)]
struct AssetId(TypeId, String);

/// Represents an asset that can be loaded from the filesystem.
pub trait Asset: 'static + Any + Sized {
  type Loader: AssetLoader<Self>;
  
  /// Loads this asset from the given path.
  fn load(assets: &AssetManager, path: &str) -> crate::Result<Handle<Self>>{
    assets.load_asset(path)
  }
}

/// A loader for a particular asset type `A`.
pub trait AssetLoader<A : Asset>: 'static {
  fn load(&self, context: &AssetContext) -> crate::Result<A>;
}

/// context state for [`AssetLoader`] operations.
pub struct AssetContext<'a> {
  /// The path of the asset being loaded.
  pub path: VirtualPath<'a>,
  manager: &'a AssetManager,
}

impl<'a> AssetContext<'a> {
  /// Loads a dependent asset from the given path.
  pub fn load_asset<A: Asset>(&self, path: VirtualPath) -> crate::Result<Handle<A>> {
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
///
/// We hide some complexities with lifetimes by dynamically borrowing the asset manager
/// state on a per-request basis.
struct AssetManagerState {
  loaders: HashMap<TypeId, Box<dyn Any>>,
  cache: HashMap<AssetId, Box<dyn Any>>,
}

impl AssetManager {
  /// Creates a new asset manager.
  pub fn new() -> Self {
    Self {
      state: Rc::new(UnsafeCell::new(AssetManagerState {
        loaders: HashMap::new(),
        cache: HashMap::new(),
      })),
    }
  }

  /// Adds a new [`AssetLoader`] to the manager.
  pub fn add_loader<A: Asset, L: AssetLoader<A>>(&mut self, loader: L) {
    let state = unsafe { &mut *self.state.get() };

    state.loaders.insert(TypeId::of::<A>(), Box::new(loader));
  }

  /// Attempts to load an asset from the given path.
  ///
  /// * If the asset is not found, or if the loader for the asset type is not registered,
  /// then an error is returned.
  /// * If the asset is found, but the loader is not registered, then an error is returned.
  /// * If the asset is found and the loader is registered, then the asset is loaded and returned.
  pub fn load_asset<A: Asset>(&self, path: impl AsVirtualPath) -> crate::Result<Handle<A>> {
    let state = unsafe { &mut *self.state.get() };
    let path = path.as_virtual_path();
    let id = AssetId(TypeId::of::<A>(), path.to_string());

    match state.cache.get(&id) {
      Some(asset) => {
        log::trace!(
          "Using cached asset {} from {}",
          std::any::type_name::<A>(),
          path.as_virtual_path()
        );

        let handle = asset
          .downcast_ref::<Handle<A>>()
          .expect("Should not be possible")
          .to_owned();

        Ok(handle)
      }
      None => {
        log::trace!(
          "Loading asset {} from {}",
          std::any::type_name::<A>(),
          path.as_virtual_path()
        );

        let state = unsafe { &mut *self.state.get() };

        let loader = state
          .loaders
          .get(&TypeId::of::<A>())
          .and_then(|it| it.downcast_ref::<A::Loader>())
          .ok_or_else(|| {
            anyhow::anyhow!(
              "Could not result loader for asset {:?}",
              std::any::type_name::<A>()
            )
          })?;

        // persist loaded assets into cache
        let context = AssetContext {
          path,
          manager: self,
        };

        let asset = loader.load(&context)?;
        let handle = Handle {
          id,
          asset: Rc::new(asset),
        };

        state
          .cache
          .insert(handle.id.clone(), Box::new(handle.clone()));

        Ok(handle)
      }
    }
  }
}
