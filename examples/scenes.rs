//! A simple example of Surreal scenes

use surreal::{prelude::*, prototype::*};

const WIDTH: f32 = 1920.;
const HEIGHT: f32 = 1080.;

fn main() {
  EngineBuilder::default()
    .with_title("Scenes")
    .with_size((WIDTH as u32, HEIGHT as u32))
    .start_scene(|_, assets, renderer| {
      let sprite = Texture::load(assets, "assets/sprites/bunny.png")?;

      renderer.add_descriptor(SpriteContextDescriptor {
        projection_view: Mat4::orthographic_rh_gl(
          -WIDTH / 2.,
          WIDTH / 2.,
          HEIGHT / 2.,
          -HEIGHT / 2.,
          0.,
          100.,
        ),
        ..Default::default()
      });

      let scene = SceneGraph::new(
        SceneNodeBuilder::default()
          .with_name("Bunny")
          .with_component(SpriteComponent { sprite })
          .build(),
      );

      Ok(scene)
    })
    .expect("An unexpected error occurred");
}

struct SpriteComponent {
  sprite: Handle<Texture>,
}

impl Object for SpriteComponent {
  #[inline(always)]
  fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
    self
  }

  #[inline(always)]
  fn as_any(&self) -> &dyn std::any::Any {
    self
  }

  #[inline(always)]
  fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
    self
  }
}

impl SceneComponent for SpriteComponent {
  fn on_draw(&mut self, context: SceneContext, renderer: &mut Renderer) {
    renderer.with(|sprite: &mut SpriteContext| {
      let position = context.node.local_position();

      sprite.batch.draw_sprite(
        &TextureRegion::from(&self.sprite),
        &SpriteOptions {
          position: vec2(position.x, position.y),
          color: Color32::WHITE,
          ..Default::default()
        },
      );
    });
  }
}
