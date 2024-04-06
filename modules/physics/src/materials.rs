use super::*;

pub struct Material2D<'w> {
  id: MaterialId,
  world: &'w dyn PhysicsWorld2D,
  owns_id: bool,
}

impl<'w> Material2D<'w> {
  pub(crate) fn from_id(world: &'w dyn PhysicsWorld2D, id: MaterialId) -> Self {
    Self {
      id,
      world,
      owns_id: false,
    }
  }

  pub fn new(world: &'w dyn PhysicsWorld2D) -> Self {
    let id = world.material_create().unwrap();

    Self {
      id,
      world,
      owns_id: true,
    }
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
    if self.owns_id {
      self.world.material_delete(self.id).unwrap();
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_create_a_valid_material() {
    let engine = PhysicsEngine::homebaked();
    let world = engine.create_world_2d();

    let mut material = Material2D::new(world.as_ref());

    material.set_friction(0.5);

    assert_eq!(material.friction(), 0.5);

    material.set_restitution(0.5);

    assert_eq!(material.restitution(), 0.5);
  }
}
