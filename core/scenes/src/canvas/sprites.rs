use super::*;

/// A component that renders a sprite.
pub struct SpriteComponent {}

impl Component for SpriteComponent {
  fn on_attach(&self, node: &Entity) {
    todo!()
  }

  fn on_detach(&self, node: &Entity) {
    todo!()
  }
}

impl EventListener<Tick> for SpriteComponent {
  fn on_event(&self, _event: &mut Tick) {
    todo!()
  }
}

impl EventListener<Draw> for SpriteComponent {
  fn on_event(&self, _event: &mut Draw) {
    todo!()
  }
}
