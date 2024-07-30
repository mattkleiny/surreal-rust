use crate::{SceneComponent, SceneContext};

/// A component that renders a sprite.
pub struct SpriteComponent {}

impl SceneComponent for SpriteComponent {
  fn on_render(&mut self, _context: &SceneContext) {
    // TODO: render the sprite
  }
}
