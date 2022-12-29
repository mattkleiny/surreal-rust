//! 2D scene components.

use std::any::Any;

use core::graphics::*;
use core::maths::vec2;
use core::scene::*;
use core::utilities::Object;

/// A [`Component`] which renders a sprite in the game world.
pub struct SpriteComponent<'a> {
  pub region: TextureRegion<'a>,
}

impl<'a> Component for SpriteComponent<'a> {
  fn on_render(&mut self, node: &mut SceneNode, manager: &mut RenderContextManager) {
    let position = node.local_position();

    manager.with(|context: &mut SpriteRenderContext| {
      context.batch.draw_sprite(
        &self.region,
        &SpriteOptions {
          position: vec2(position.x, position.y),
          ..Default::default()
        },
      );
    });
  }

  fn get_kind(&self) -> ComponentKind {
    ComponentKind::Renderer
  }
}

/// A [`RenderContext`] for [`SpriteComponent`]s.
pub struct SpriteRenderContext {
  batch: SpriteBatch,
  material: Material,
}

impl RenderContext for SpriteRenderContext {
  fn on_begin_frame(&mut self) {
    self.batch.begin(&self.material);
  }

  fn on_end_frame(&mut self) {
    self.batch.flush();
  }
}

// TODO: find a way to get rid of this
impl Object for SpriteRenderContext {
  fn as_any(&self) -> &dyn Any {
    self as &dyn Any
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self as &mut dyn Any
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // #[test]
  // fn sprite_should_render() {
  //   let server = GraphicsServer::new(Box::new(HeadlessGraphicsBackend::new()));
  //   let texture = Texture::create_colored(&server, 1, 1, Color::RED);
  //
  //   let node = SceneNodeBuilder::default()
  //     .with_component(SpriteComponent {
  //       region: TextureRegion::from(&texture),
  //     })
  //     .build();
  // }
}
