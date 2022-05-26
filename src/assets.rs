//! A simple asset management system with support for hot file reloading.

use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::io::{AsVirtualPath, VirtualPath};

/// A type of asset that can be persisted in the asset manager.
pub trait Asset: Sized {
  /// The associated loader for this asset.
  type Loader: AssetLoader<Self>;
}

/// Allows loading an asset from the virtual file system.
pub trait AssetLoader<A> {
  /// Loads the asset from the given path.
  fn load(&self, context: &AssetContext) -> crate::Result<A>;
}

/// A context for asset operations.
pub struct AssetContext<'a> {
  /// The path of the asset being loaded.
  pub path: VirtualPath<'a>,
  /// The manager that owns this context.
  pub manager: &'a AssetManager,
}

impl<'a> AssetContext<'a> {
  /// Loads a dependent asset from the given path.
  pub fn load_asset<A: Asset + 'static>(&self, path: VirtualPath) -> crate::Result<A> {
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
  state: Rc<RefCell<AssetManagerState>>,
}

/// The internal state for the asset manager.
struct AssetManagerState {
  loaders: HashMap<TypeId, Box<dyn Any>>,
}

impl AssetManager {
  /// Creates a new asset manager.
  pub fn new() -> Self {
    Self {
      state: Rc::new(RefCell::new(AssetManagerState {
        loaders: HashMap::new(),
      })),
    }
  }

  /// Adds a new asset loader to the manager.
  pub fn add_loader<A: Asset + 'static, L: AssetLoader<A> + 'static>(&mut self, loader: L) {
    let mut state = self.state.borrow_mut();

    state.loaders.insert(TypeId::of::<A>(), Box::new(loader));
  }

  /// Attempts to load an asset from the given path.
  pub fn load_asset<A: Asset + 'static>(&self, path: impl AsVirtualPath) -> crate::Result<A> {
    let state = self.state.borrow();

    let loader = state.loaders
      .get(&TypeId::of::<A>())
      .and_then(|it| it.downcast_ref::<A::Loader>())
      .ok_or(anyhow::anyhow!("Could not result loader for asset {:?}", std::any::type_name::<A>()))?;

    let context = AssetContext {
      path: path.as_virtual_path(),
      manager: self,
    };

    // TODO: persist loaded assets in cache?
    loader.load(&context)
  }
}

#[cfg(test)]
mod tests {
  use crate::graphics::{ImageLoader, Texture, TextureLoader, TextureOptions};
  use crate::prelude::HeadlessGraphicsBackend;

  use super::*;

  #[test]
  fn assets_should_load() {
    let mut manager = AssetManager::new();

    manager.add_loader(ImageLoader {
      format: None
    });

    manager.add_loader(TextureLoader {
      server: HeadlessGraphicsBackend::new(),
      options: TextureOptions::default(),
    });

    let texture: Texture = manager.load_asset("assets/sprites/bunny.png").expect("Failed to load asset");

    println!("Image is {:}x{:} pixels", texture.width(), texture.height());
  }
}