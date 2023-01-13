//! Rendering abstractions and pipelines.
//!
//! This is a series of components designed to make it simpler to build more complex render
//! pipelines than using the 'material', 'mesh', 'render targets' etc do alone.

use std::any::TypeId;

use super::*;
use crate::{collections::FastHashMap, utilities::Object};

/// A context for rendering operations.
///
/// A context contains the state useful for a particular kind of rendering operation, and also
/// exposes some basic lifecycle methods. It's lazily constructed upon first use and remains
/// alive until the [`Renderer`] is dropped.
pub trait RenderContext: Object {
  fn on_begin_with(&mut self) {}
  fn on_end_with(&mut self) {}
  fn on_begin_frame(&mut self) {}
  fn on_end_frame(&mut self) {}
}

/// Describes how to build a [`RenderContext`] .
///
/// A descriptor is a factory for a render context, and contain configuration and shared data
/// that is usable in the creation of the context itself.
pub trait RenderContextDescriptor {
  /// The type of [`RenderContext`] that will be created by this descriptor.
  type Context: RenderContext;

  /// Creates the associated [`RenderContext`].
  fn create(&self, graphics: &GraphicsServer) -> Self::Context;
}

/// Allows an object to be rendered via a [`Renderer`].
///
/// Requires that the manager is configured for the associated context.
pub trait Renderable<C: RenderContext> {
  /// Renders this object via the associated [`RenderContext`].
  fn render(&self, context: &mut C);
}

/// A manager for [`RenderContext`]s.
///
/// A [`RenderContext`] encodes all of the required details and lifecycle for textures, materials,
/// render targets, shaders, necessary in a single invocation of some rendering state.
pub struct Renderer {
  graphics: GraphicsServer,
  contexts: FastHashMap<TypeId, Box<dyn RenderContext>>,
}

impl Renderer {
  /// Creates a new render manager.
  pub fn new(graphics: &GraphicsServer) -> Self {
    Self {
      graphics: graphics.clone(),
      contexts: FastHashMap::default(),
    }
  }

  /// Configures the manager with the given [`RenderContext`].
  pub fn add_context<C: RenderContext>(&mut self, context: C) {
    let key = TypeId::of::<C>();
    let value = Box::new(context);

    self.contexts.insert(key, value);
  }

  /// Configures the manager with the given [`RenderContextDescriptor`].
  pub fn add_descriptor<D: RenderContextDescriptor>(&mut self, descriptor: D) {
    let key = TypeId::of::<D::Context>();
    let value = Box::new(descriptor.create(&self.graphics));

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
  /// If the context cannot be acquired, the body will not be run.
  pub fn with<C: RenderContext>(&mut self, body: impl FnOnce(&mut C)) {
    if let Some(context) = self.contexts.get_mut(&TypeId::of::<C>()) {
      let context = context.as_any_mut().downcast_mut::<C>().unwrap();

      context.on_begin_with();
      body(context);
      context.on_end_with();
    }
  }

  /// Renders the given [`Renderable`] via the associated context.
  pub fn render<R: Renderable<C>, C: RenderContext>(&mut self, renderable: &R) {
    self.with(|context| {
      renderable.render(context);
    });
  }

  /// Releases the given [`RenderContext`] from the manager.
  ///
  /// If it existed, the context will be dropped.
  pub fn release<C: RenderContext>(&mut self) {
    self.contexts.remove(&TypeId::of::<C>());
  }

  /// Clears all [`RenderContext`]s from the manager, resetting it to a clean state.
  pub fn reset(&mut self) {
    self.contexts.clear();
  }
}
