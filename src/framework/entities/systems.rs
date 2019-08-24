//! Commonly used ECS systems.

use super::*;

/// A system for 2d physics.
pub struct PhysicsSystem2d;

impl<'a> System<'a> for PhysicsSystem2d {
  type SystemData = (
    Read<'a, DeltaTime>,
    WriteStorage<'a, Transform2d>
  );

  fn run(&mut self, data: Self::SystemData) {
    let (delta_time, mut transform) = data;
    let _delta_time = delta_time.0;

    for transform in (&mut transform).join() {
      let _position = transform.position;
      let _rotation = transform.rotation;
    }
  }
}