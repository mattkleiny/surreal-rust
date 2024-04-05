use common::profiling;

use super::*;

/// A 3d physics world.
#[derive(Default)]
pub struct HomebakedWorld3D {}

#[allow(unused_variables)]
impl PhysicsWorld for HomebakedWorld3D {
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
impl PhysicsWorld3D for HomebakedWorld3D {}
