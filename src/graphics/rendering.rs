use crate::collections::AnyMap;
use crate::graphics::GraphicsServer;

/// A renderer is responsible for rendering a scene.
///
/// The render manages a set of 'contexts' which include details for texture, material,
/// render target, shader, etc. Each context can be acquired and utilized via the `acquire_context`
/// method.
pub struct Renderer {
  server: GraphicsServer,
  contexts: AnyMap,
}

/// A context is a set of resources that can be used to render a scene.
///
/// Contexts can contain textures, materials, shaders, etc. Acquiring a context for the first time
/// will result in it's resources being allocated. Instructions can then be emitted to a command
/// buffer to allow evaluation against the graphics server.
pub trait RenderContext: Sized + 'static {
  /// Creates the render context.
  fn create(server: &GraphicsServer) -> Self;
}

impl Renderer {
  /// Creates a new renderer.
  pub fn new(server: &GraphicsServer) -> Self {
    Self {
      server: server.clone(),
      contexts: AnyMap::new(),
    }
  }

  /// Acquires a context for the given type and execute the body against it.
  pub fn with<C>(&mut self, body: impl FnOnce(&mut C) -> ()) where C: RenderContext {
    let context = self.contexts.get_mut::<C>();

    if let Some(context) = context {
      // use the existing context
      body(context);
    } else if let None = context {
      // create a new context
      let mut context = C::create(&self.server);

      body(&mut context);

      self.contexts.insert(context);
    }
  }

  /// Removes the given context from the renderer, allowing it's resources to be reclaimed.
  pub fn release<C>(&mut self) where C: RenderContext {
    self.contexts.remove::<C>();
  }

  /// Removes all contexts from the renderer, allowing all resources to be reclaimed.
  pub fn reset(&mut self) {
    self.contexts.clear();
  }
}

#[cfg(test)]
mod tests {
  use crate::graphics::{Material, Texture};
  use crate::platform::headless::graphics::HeadlessGraphicsBackend;
  use crate::prototype::load_standard_shader;

  use super::*;

  /// Context for signed distance field rendering.
  #[allow(dead_code)]
  struct DistanceFieldContext {
    pub texture: Texture,
    pub target1: Texture,
    pub target2: Texture,
    pub material: Material,
    pub tolerance: f32,
  }

  impl RenderContext for DistanceFieldContext {
    fn create(server: &GraphicsServer) -> Self {
      let shader = load_standard_shader(server);

      Self {
        texture: Texture::new(server),
        target1: Texture::new(server),
        target2: Texture::new(server),
        material: Material::new(server, &shader),
        tolerance: 0.1,
      }
    }
  }

  #[test]
  fn renderer_should_create_and_manage_contexts() {
    let server = HeadlessGraphicsBackend::new();
    let mut renderer = Renderer::new(&server);

    renderer.with(|context: &mut DistanceFieldContext| {
      context.tolerance = 10.;
    });

    renderer.with(|context: &mut DistanceFieldContext| {
      assert_eq!(context.tolerance, 10.);
    });
  }
}