use super::*;

pub struct Collider2D<'w> {
  collider_id: ColliderId,
  world: &'w dyn PhysicsWorld2D,
}

impl<'w> Collider2D<'w> {
  pub fn new_rectangle(world: &'w dyn PhysicsWorld2D) -> Self {
    Collider2D {
      world,
      collider_id: world
        .collider_create_rectangle(ColliderKind::Solid, Vec2::ZERO, Vec2::ONE)
        .unwrap(),
    }
  }

  pub fn new_circle(world: &'w dyn PhysicsWorld2D) -> Self {
    Collider2D {
      world,
      collider_id: world
        .collider_create_circle(ColliderKind::Solid, Vec2::ZERO, 1.0)
        .unwrap(),
    }
  }

  pub fn position(&self) -> Vec2 {
    self.world.collider_get_position(self.collider_id).unwrap()
  }

  pub fn set_position(&self, position: Vec2) {
    self.world.collider_set_position(self.collider_id, position).unwrap();
  }

  pub fn rotation(&self) -> f32 {
    self.world.collider_get_rotation(self.collider_id).unwrap()
  }

  pub fn set_rotation(&self, rotation: f32) {
    self.world.collider_set_rotation(self.collider_id, rotation).unwrap();
  }

  pub fn scale(&self) -> Vec2 {
    self.world.collider_get_scale(self.collider_id).unwrap()
  }

  pub fn set_scale(&self, scale: Vec2) {
    self.world.collider_set_scale(self.collider_id, scale).unwrap();
  }
}

impl<'w> Drop for Collider2D<'w> {
  fn drop(&mut self) {
    self.world.collider_delete(self.collider_id).unwrap();
  }
}
