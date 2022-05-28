use crate::collections::AnyMap;
use crate::graphics::GraphicsServer;

/// Allows an object to be rendered via a [`RenderManager`].
///
/// Requires that the manager is configured for the associated context.
pub trait Renderable<C> where C: RenderContext {
  /// Renders this object via the associated [`RenderContext`].
  fn render(&self, context: &mut C);
}

/// A context for rendering operations.
///
/// A context contains the state useful for a particular kind of rendering operation, and also
/// exposes some basic lifecycle methods.
pub trait RenderContext: Sized + 'static {
  fn on_initialize(&mut self) {}
  fn on_before_with(&mut self) {}
  fn on_after_with(&mut self) {}
}

/// Describes how to build a [`RenderContext`] .
///
/// A descriptor is a factory for a render context, and contain configuration and shared data
/// that is usable in the creation of the context itself.
pub trait RenderContextDescriptor {
  /// The type of context that will be created by this descriptor.
  type Context: RenderContext;

  /// Creates the associated context.
  fn create(&self, server: &GraphicsServer) -> Self::Context;
}

/// A renderer is responsible for rendering a scene.
///
/// The render manages a set of [`RenderContext`] s which include all the required details for
/// textures, materials, render targets, shaders, etc.
///
/// Each context can be acquired and utilized via the `with` method.
pub struct RenderManager {
  server: GraphicsServer,
  contexts: AnyMap,
}

impl RenderManager {
  /// Creates a new render manager.
  pub fn new(server: &GraphicsServer) -> Self {
    Self {
      server: server.clone(),
      contexts: AnyMap::new(),
    }
  }

  /// Configures the manager with the given context.
  pub fn configure<D>(&mut self, descriptor: D) where D: RenderContextDescriptor {
    self.contexts.insert(descriptor.create(&self.server));
  }

  /// Renders the given object via the associated context.
  pub fn render<R, C>(&mut self, renderable: &R) where R: Renderable<C>, C: RenderContext {
    self.with(|context| {
      renderable.render(context);
    });
  }

  /// Acquires a context for the given descriptor.
  ///
  /// If the context cannot be acquired, the body will not be run.
  pub fn with<C>(&mut self, body: impl FnOnce(&mut C) -> ()) where C: RenderContext {
    if let Some(context) = self.contexts.get_mut::<C>() {
      context.on_before_with();
      body(context);
      context.on_after_with();
    }
  }

  /// Releases the given context from the manager.
  pub fn release<C>(&mut self) where C: RenderContext {
    self.contexts.remove::<C>();
  }

  /// Clears all contexts from the manager, resetting it to a clean state.
  pub fn reset(&mut self) {
    self.contexts.clear();
  }
}
