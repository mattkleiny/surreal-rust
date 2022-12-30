use surreal::graphics::*;
use surreal::maths::vec2;
use surreal::scene::*;

/// A [`Component`] which renders a sprite in the game world.
pub struct SpriteComponent {
  pub region: TextureRegion,
}

impl Component for SpriteComponent {
  fn name(&self) -> &'static str {
    "SpriteComponent"
  }

  fn on_render(&mut self, node: &mut SceneNode, manager: &mut RenderContextManager) {
    let position = node.local_position();

    manager.with(|context: &mut SpriteContext| {
      context.batch.draw_sprite(
        &self.region,
        &SpriteOptions {
          position: vec2(position.x, position.y),
          ..Default::default()
        },
      );
    });
  }

  fn kind(&self) -> ComponentKind {
    ComponentKind::Renderer
  }
}

/// A [`RenderContext`] for [`SpriteComponent`]s.
///
pub struct SpriteContext {
  batch: SpriteBatch,
  material: Material,
}

impl RenderContext for SpriteContext {
  fn on_begin_frame(&mut self) {
    self.batch.begin(&self.material);
  }

  fn on_end_frame(&mut self) {
    self.batch.flush();
  }
}

#[cfg(test)]
mod tests {
  use surreal::maths::{vec3, Quat};

  use super::*;

  #[test]
  fn sprite_should_render() {
    let graphics = create_test_graphics();
    let texture = Texture::create_colored(&graphics, 1, 1, Color::RED);

    let graph = SceneGraph::new(
      SceneNodeBuilder::default()
        .with_name("Test")
        .with_local_position(vec3(0., 0., 0.))
        .with_local_rotation(Quat::from_rotation_z(std::f32::consts::PI))
        .with_component(SpriteComponent {
          region: TextureRegion::from(texture),
        }),
    );

    println!("{:?}", graph);
  }
}
