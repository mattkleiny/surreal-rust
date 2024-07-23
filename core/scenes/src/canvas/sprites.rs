use graphics::TextureRegion;

use crate::{SceneComponent, SceneContext};

/// A component that renders a sprite.
pub struct SpriteComponent {
  pub region: TextureRegion,
}

/// A service for rendering sprites.
pub trait SpriteRenderer {
  fn render_sprite(&self, region: &TextureRegion);
}

impl SceneComponent for SpriteComponent {
  fn on_render(&mut self, context: &SceneContext) {
    if let Some(renderer) = context.resolve::<dyn SpriteRenderer>() {
      renderer.render_sprite(&self.region);
    }
  }
}
