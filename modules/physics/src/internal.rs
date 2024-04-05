use super::*;

mod world2d;
mod world3d;

/// The default, home-baked [`PhysicsBackend`].
///
/// This is a simple physics backend that uses a simple Euler integration
/// algorithm to simulate the physics of the game world.
#[derive(Default)]
pub(crate) struct InternalPhysicsBackend {}

#[allow(unused_variables)]
impl PhysicsBackend for InternalPhysicsBackend {
  fn create_world_2d(&self) -> Box<dyn PhysicsWorld2D> {
    Box::<world2d::InternalPhysicsWorld2D>::default()
  }

  fn create_world_3d(&self) -> Box<dyn PhysicsWorld3D> {
    Box::<world3d::InternalPhysicsWorld3D>::default()
  }
}
