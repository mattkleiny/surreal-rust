use super::*;

pub struct Joint2D<'w> {
  id: JointId,
  world: &'w dyn PhysicsWorld2D,
}

impl<'w> Joint2D<'w> {
  pub fn new(world: &'w dyn PhysicsWorld2D) -> Self {
    let id = world.joint_create().unwrap();

    Self { id, world }
  }

  pub fn id(&self) -> JointId {
    self.id
  }

  pub fn world(&self) -> &dyn PhysicsWorld2D {
    self.world
  }
}

impl<'w> Drop for Joint2D<'w> {
  fn drop(&mut self) {
    self.world.joint_delete(self.id).unwrap();
  }
}
