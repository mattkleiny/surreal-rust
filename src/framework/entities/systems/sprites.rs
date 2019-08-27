use super::*;

/// A system for sprite rendering.
pub struct SpriteSystem;

impl<'a> System<'a> for SpriteSystem {
  type SystemData = (
    Read<'a, DeltaTime>,
    ReadStorage<'a, Render>
  );

  fn run(&mut self, data: Self::SystemData) {
    let (delta_time, mut render) = data;
    let _delta_time = delta_time.0;

    for _render in (&mut render).join() {
      // TODO: if let AssetRef::Sprite(_) = render
    }
  }
}