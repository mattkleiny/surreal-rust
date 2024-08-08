use crate::{Component, Event};

/// A component that renders a sprite.
pub struct SpriteComponent {}

impl Component for SpriteComponent {
  fn on_event(&self, event: &dyn Event) {}
}
