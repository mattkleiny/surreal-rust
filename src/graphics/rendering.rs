use crate::collections::AnyMap;
use crate::graphics::GraphicsServer;

/// A renderer is responsible for rendering a scene.
///
/// The render manages a set of [`RenderContext`] s which include all the required details for
/// textures, materials, render targets, shaders, etc.
///
/// Each context can be acquired and utilized via the `with`  method. If the context has not been
/// used before it will be initialized.
pub struct RenderManager {
  server: GraphicsServer,
  contexts: AnyMap,
}

/// Describes how to build a [`RenderContext`] .
///
/// A descriptor is a simple factory for a [`RenderContext`]s, and contain states that is usable in
/// the creation of the context itself.
///
/// Passing a descriptor to the [`Renderer`]  will only result in the creation of a context if it doesn't
/// already exist.
pub trait RenderContextDescriptor {
  /// The type of context that will be created by this descriptor.
  type Context: RenderContext;

  fn create(&self, server: &GraphicsServer) -> Self::Context;
}

/// A context for rendering operations.
pub trait RenderContext: Sized + 'static {
  fn on_initialize(&mut self) {}
  fn on_before_with(&mut self) {}
  fn on_after_with(&mut self) {}
}

/// Allows an object to be rendered via a [`RenderManager`].
pub trait Renderable<C> where C: RenderContext {
  /// Renders this object via the associated [`RenderContext`].
  fn render(&self, context: &mut C);
}

impl RenderManager {
  /// Creates a new renderer.
  pub fn new(server: &GraphicsServer) -> Self {
    Self {
      server: server.clone(),
      contexts: AnyMap::new(),
    }
  }

  pub fn configure<D>(&mut self, descriptor: D) where D: RenderContextDescriptor {
    self.contexts.insert(descriptor.create(&self.server));
  }

  pub fn render<R, C>(&mut self, renderable: &R) where R: Renderable<C>, C: RenderContext {
    self.with(|context| {
      renderable.render(context);
    });
  }

  // TODO: recursive access to contexts?
  pub fn with<C>(&mut self, body: impl FnOnce(&mut C) -> ()) where C: RenderContext {
    match self.contexts.get_mut::<C>() {
      Some(context) => {
        context.on_before_with();
        body(context);
        context.on_after_with();
      }
      // TODO: handle this more elegantly?
      None => panic!("A render context is not available: {}", std::any::type_name::<C>()),
    }
  }

  pub fn release<C>(&mut self) where C: RenderContext {
    self.contexts.remove::<C>();
  }

  pub fn reset(&mut self) {
    self.contexts.clear();
  }
}

#[cfg(test)]
mod tests {
  use crate::graphics::*;

  struct ExampleDescriptor {
    pub tolerance: f32,
  }

  #[allow(dead_code)]
  struct ExampleContext {
    pub texture: Texture,
    pub target1: Texture,
    pub target2: Texture,
    pub tolerance: f32,
  }

  impl RenderContextDescriptor for ExampleDescriptor {
    type Context = ExampleContext;

    fn create(&self, server: &GraphicsServer) -> Self::Context {
      Self::Context {
        texture: Texture::new(server),
        target1: Texture::new(server),
        target2: Texture::new(server),
        tolerance: self.tolerance,
      }
    }
  }

  impl RenderContext for ExampleContext {}

  #[test]
  fn render_manager_should_create_and_manage_contexts() {
    let server = HeadlessGraphicsBackend::new();
    let mut renderer = RenderManager::new(&server);

    renderer.configure(ExampleDescriptor { tolerance: 0.1 });

    renderer.with(|context: &mut ExampleContext| {
      context.tolerance = 10.;
    });

    renderer.with(|context: &mut ExampleContext| {
      assert_eq!(context.tolerance, 10.);
    });
  }
}