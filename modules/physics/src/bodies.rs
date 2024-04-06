use super::*;

pub struct Body2D<'w> {
  id: BodyId,
  world: &'w dyn PhysicsWorld2D,
}

impl<'w> Body2D<'w> {
  pub fn new(world: &'w dyn PhysicsWorld2D, kind: BodyKind) -> Self {
    Self {
      world,
      id: world.body_create(kind, Vec2::ZERO).unwrap(),
    }
  }

  pub fn id(&self) -> BodyId {
    self.id
  }

  pub fn world(&self) -> &dyn PhysicsWorld2D {
    self.world
  }

  pub fn kind(&self) -> BodyKind {
    self.world.body_get_kind(self.id).unwrap_or(BodyKind::Dynamic)
  }

  pub fn set_kind(&mut self, kind: BodyKind) {
    self.world.body_set_kind(self.id, kind).unwrap();
  }

  pub fn material(&self) -> Option<Material2D> {
    self
      .world
      .body_get_material(self.id)
      .map(|id| Material2D::from_id(self.world, id))
  }

  pub fn set_material(&mut self, material: &Material2D) {
    self.world.body_set_material(self.id, Some(material.id())).unwrap();
  }

  pub fn position(&self) -> Vec2 {
    self.world.body_get_position(self.id).unwrap_or_default()
  }

  pub fn set_position(&mut self, position: Vec2) {
    self.world.body_set_position(self.id, position).unwrap();
  }
}

impl<'w> Drop for Body2D<'w> {
  fn drop(&mut self) {
    self.world.body_delete(self.id).unwrap();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let engine = PhysicsEngine::homebaked();
    let world = engine.create_world_2d();

    let mut body = Body2D::new(world.as_ref(), BodyKind::Dynamic);
    let material = Material2D::new(world.as_ref());

    body.set_position(Vec2::ONE);
    body.set_material(&material);

    assert_eq!(body.position(), Vec2::ONE);

    assert!(body.material().is_some());
  }
}
