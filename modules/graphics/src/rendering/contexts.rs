//! Rendering contexts and inversion of control.

use std::any::{Any, TypeId};

use common::{reinterpret_cast_mut, FastHashMap};

/// A context for rendering operations.
///
/// A context contains the state useful for a particular kind of rendering
/// operation, and also exposes some basic lifecycle methods. It's lazily
/// constructed upon first use and remains alive until the [`Renderer`] is
/// dropped.
#[allow(unused_variables)]
pub trait RenderContext: Any + Send + Sync {
  fn on_begin_with(&mut self) {}
  fn on_end_with(&mut self) {}
  fn on_begin_frame(&mut self) {}
  fn on_end_frame(&mut self) {}
}

/// A manager for [`RenderContext`]s.
///
/// A [`RenderContext`] encodes all the required details and lifecycle for
/// textures, materials, render targets, shaders, necessary in a single
/// invocation of some rendering state.
pub struct Renderer {
  contexts: FastHashMap<TypeId, Box<dyn RenderContext>>,
}

impl Renderer {
  /// Creates a new render manager.
  pub fn new() -> Self {
    Self {
      contexts: FastHashMap::default(),
    }
  }

  /// Configures the manager with the given [`RenderContext`].
  pub fn add_context<C: RenderContext>(&mut self, context: C) {
    let key = TypeId::of::<C>();
    let value = Box::new(context);

    self.contexts.insert(key, value);
  }

  /// Begins a new frame.
  pub fn begin_frame(&mut self) {
    for context in self.contexts.values_mut() {
      context.on_begin_frame();
    }
  }

  /// Ends the current frame.
  pub fn end_frame(&mut self) {
    for context in self.contexts.values_mut() {
      context.on_end_frame();
    }
  }

  /// Acquires a [`RenderContext`] and executes the body against it.
  ///
  /// If the context cannot be acquired, the body will not be run, and no error
  /// will be returned. This is useful for optional contexts, but can be
  /// confusing if the context is required.
  pub fn with<C: RenderContext>(&mut self, body: impl FnOnce(&mut C)) {
    if let Some(context) = self.contexts.get_mut(&TypeId::of::<C>()) {
      context.on_begin_with();

      body(unsafe { reinterpret_cast_mut(context) });

      context.on_end_with();
    }
  }

  /// Releases the given [`RenderContext`] from the renderer.
  pub fn release<C: RenderContext>(&mut self) {
    self.contexts.remove(&TypeId::of::<C>());
  }

  /// Clears all [`RenderContext`]s from the renderer.
  pub fn reset(&mut self) {
    self.contexts.clear();
  }
}
