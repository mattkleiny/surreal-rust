//! A home baked physics engine for simple use cases.

use super::*;

mod world2d;
mod world3d;

/// The backend for the homebaked rust-based physics engine.
///
/// Probably not the best implementation, but sure is fun.
#[derive(Default)]
pub struct RustPhysicsBackend;

impl PhysicsBackend for RustPhysicsBackend {
  fn create_world_2d(&self) -> Result<Box<PhysicsWorld2D>, WorldError> {
    Ok(Box::new(world2d::PhysicsWorld2D::default()))
  }

  fn create_world_3d(&self) -> Result<Box<PhysicsWorld3D>, WorldError> {
    Ok(Box::new(world3d::PhysicsWorld3D::default()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_physics_world() {
    let world = physics().create_world_2d().unwrap();
    let collider_id = world.collider_create().unwrap();

    world.tick(0.16);
    world.tick(0.16);

    world.collider_delete(collider_id).unwrap();
  }
}
