use super::*;

pub struct Material2D<'w> {
  id: MaterialId,
  world: &'w dyn PhysicsWorld2D,
}

impl<'w> Material2D<'w> {
  pub fn new(world: &'w dyn PhysicsWorld2D) -> Self {
    let id = world.material_create().unwrap();

    Self { id, world }
  }

  pub fn id(&self) -> MaterialId {
    self.id
  }

  pub fn world(&self) -> &dyn PhysicsWorld2D {
    self.world
  }

  pub fn friction(&self) -> f32 {
    self.world.material_get_friction(self.id).unwrap_or_default()
  }

  pub fn set_friction(&mut self, friction: f32) {
    self.world.material_set_friction(self.id, friction).unwrap();
  }

  pub fn restitution(&self) -> f32 {
    self.world.material_get_restitution(self.id).unwrap_or_default()
  }

  pub fn set_restitution(&mut self, restitution: f32) {
    self.world.material_set_restitution(self.id, restitution).unwrap();
  }
}

impl<'w> Drop for Material2D<'w> {
  fn drop(&mut self) {
    self.world.material_delete(self.id).unwrap();
  }
}
