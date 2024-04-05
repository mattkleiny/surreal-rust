use super::*;

pub struct Effector2D<'w> {
  id: EffectorId,
  world: &'w dyn PhysicsWorld2D,
}

pub enum EffectorShape {
  Box(Vec2),
}

impl<'w> Effector2D<'w> {
  pub fn new(world: &'w dyn PhysicsWorld2D, shape: EffectorShape) -> Self {
    let id = match shape {
      EffectorShape::Box(size) => world.effector_create_box(EffectorKind::Gravity, Vec2::ZERO, size),
    }
    .unwrap();

    Self { id, world }
  }

  pub fn id(&self) -> EffectorId {
    self.id
  }

  pub fn world(&self) -> &dyn PhysicsWorld2D {
    self.world
  }

  pub fn position(&self) -> Vec2 {
    self.world.effector_get_position(self.id).unwrap_or_default()
  }

  pub fn set_position(&self, position: Vec2) {
    self.world.effector_set_position(self.id, position).unwrap();
  }
}

impl<'w> Drop for Effector2D<'w> {
  fn drop(&mut self) {
    self.world.effector_delete(self.id).unwrap();
  }
}
