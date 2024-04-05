use common::profiling;

use super::*;

/// A 3d physics world.
#[derive(Default)]
pub struct InternalPhysicsWorld3D {}

#[allow(unused_variables)]
impl PhysicsWorld for InternalPhysicsWorld3D {
  #[profiling]
  fn step(&self, delta_time: f32) {
    todo!()
  }

  #[profiling]
  fn reset(&self) {
    todo!()
  }
}

#[allow(unused_variables)]
impl PhysicsWorld3D for InternalPhysicsWorld3D {}
