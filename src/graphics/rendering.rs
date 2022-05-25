use crate::collections::AnyMap;
use crate::graphics::GraphicsServer;

/// A renderer is responsible for rendering a scene.
///
/// The render manages a set of [`RenderContext`] s which include all the required details for
/// textures, materials, render targets, shaders, etc.
///
/// Each context can be acquired and utilized via the `with`  method. If the context has not been
/// used before it will be initialized.
pub struct Renderer {
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

impl Renderer {
  /// Creates a new renderer.
  pub fn new(server: &GraphicsServer) -> Self {
    Self {
      server: server.clone(),
      contexts: AnyMap::new(),
    }
  }

  /// Acquires a `RenderContext` for the given descriptor `D` and execute the given body against it.
  ///
  /// If the context has not been used before, or has been freed since, the descriptor will be used
  /// to initialize it anew.
  pub fn with<D>(&mut self, descriptor: &D, body: impl FnOnce(&mut D::Context) -> ()) where D: RenderContextDescriptor {
    match self.contexts.get_mut::<D::Context>() {
      Some(context) => {
        // already exists? just use it
        context.on_before_with();
        body(context);
        context.on_after_with();
      }
      None => {
        // create a new context
        let mut context = descriptor.create(&self.server);

        context.on_initialize();
        context.on_before_with();
        body(&mut context);
        context.on_after_with();

        self.contexts.insert(context);
      }
    }
  }

  /// Removes the given context from the renderer, allowing it's resources to be reclaimed.
  pub fn release<D>(&mut self) where D: RenderContextDescriptor {
    self.contexts.remove::<D::Context>();
  }

  /// Removes all contexts from the renderer, allowing all resources to be reclaimed.
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
  fn renderer_should_create_and_manage_contexts() {
    let server = HeadlessGraphicsBackend::new();
    let mut renderer = Renderer::new(&server);
    let descriptor = ExampleDescriptor { tolerance: 0.1 };

    renderer.with(&descriptor, |context: &mut ExampleContext| {
      context.tolerance = 10.;
    });

    renderer.with(&descriptor, |context: &mut ExampleContext| {
      assert_eq!(context.tolerance, 10.);
    });
  }
}