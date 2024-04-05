use super::*;

mod world2d;
mod world3d;

/// The default, home-baked [`PhysicsBackend`].
///
/// This is a simple physics backend that uses a simple Euler integration
/// algorithm to simulate the physics of the game world.
#[derive(Default)]
pub struct HomebakedPhysicsBackend {}

#[allow(unused_variables)]
impl PhysicsBackend for HomebakedPhysicsBackend {
  fn create_world_2d(&self) -> Box<dyn PhysicsWorld2D> {
    Box::<world2d::HomebakedWorld2D>::default()
  }

  fn create_world_3d(&self) -> Box<dyn PhysicsWorld3D> {
    Box::<world3d::HomebakedWorld3D>::default()
  }
}
