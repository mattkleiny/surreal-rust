use common::vec2;
use graphics::{SpriteBatch, SpriteOptions, TextureRegion};

use crate::{SceneComponent, SceneContext};

/// A component that renders a sprite.
pub struct SpriteComponent {
  pub region: TextureRegion,
}

impl SceneComponent for SpriteComponent {
  fn on_render(&mut self, context: &SceneContext) {
    if let Some(batch) = context.services.resolve_mut::<SpriteBatch>() {
      batch.draw_sprite(&self.region, &SpriteOptions {
        position: vec2(0., 0.),
        ..SpriteOptions::default()
      });
    }
  }
}
