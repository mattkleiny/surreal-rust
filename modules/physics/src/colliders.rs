use super::*;

pub struct Collider2D<'w> {
  id: ColliderId,
  world: &'w dyn PhysicsWorld2D,
}

pub enum ColliderShape {
  Circle(f32),
  Rectangle(Vec2),
}

impl<'w> Collider2D<'w> {
  pub fn new(world: &'w dyn PhysicsWorld2D, shape: ColliderShape) -> Self {
    let id = match shape {
      ColliderShape::Circle(radius) => world.collider_create_circle(ColliderKind::Solid, Vec2::ZERO, radius),
      ColliderShape::Rectangle(_) => todo!(),
    }
    .unwrap();

    Self { id, world }
  }

  pub fn id(&self) -> ColliderId {
    self.id
  }

  pub fn world(&self) -> &dyn PhysicsWorld2D {
    self.world
  }

  pub fn position(&self) -> Vec2 {
    self.world.collider_get_position(self.id).unwrap_or_default()
  }

  pub fn set_position(&self, position: Vec2) {
    self.world.collider_set_position(self.id, position).unwrap();
  }
}

impl<'w> Drop for Collider2D<'w> {
  fn drop(&mut self) {
    self.world.collider_delete(self.id).unwrap();
  }
}
