use std::sync::RwLock;

use common::Arena;

use super::*;

/// A 3D physics world.
#[derive(Default)]
pub struct PhysicsWorld3D {
  colliders: RwLock<Arena<ColliderId, Collider>>,
  bodies: RwLock<Arena<BodyId, Body>>,
}

/// A 3D collider.
struct Collider {}

/// A 3D physics body.
struct Body {}

#[allow(unused_variables)]
impl PhysicsWorld for PhysicsWorld3D {
  type Vector = Real3;

  fn tick(&self, _delta: f32) {
    // no-op
  }

  fn collider_create(&self) -> Result<ColliderId, ColliderError> {
    let mut colliders = self.colliders.write().expect("Failed to lock colliders");

    Ok(colliders.insert(Collider {}))
  }

  fn collider_get_position(&self, id: ColliderId) -> Result<Self::Vector, ColliderError> {
    todo!()
  }

  fn collider_set_position(&self, id: ColliderId, position: Self::Vector) -> Result<(), ColliderError> {
    todo!()
  }

  fn collider_delete(&self, id: ColliderId) -> Result<(), ColliderError> {
    let mut colliders = self.colliders.write().expect("Failed to lock colliders");

    colliders.remove(id).ok_or(ColliderError::InvalidId(id))?;

    Ok(())
  }

  fn body_create(&self) -> Result<BodyId, BodyError> {
    let mut bodies = self.bodies.write().expect("Failed to lock bodies");

    Ok(bodies.insert(Body {}))
  }

  fn body_get_position(&self, id: BodyId) -> Result<Self::Vector, BodyError> {
    todo!()
  }

  fn body_set_position(&self, id: BodyId, position: Self::Vector) -> Result<(), BodyError> {
    todo!()
  }

  fn body_get_velocity(&self, id: BodyId) -> Result<Self::Vector, BodyError> {
    todo!()
  }

  fn body_set_velocity(&self, id: BodyId, velocity: Self::Vector) -> Result<(), BodyError> {
    todo!()
  }

  fn body_delete(&self, id: BodyId) -> Result<(), BodyError> {
    let mut bodies = self.bodies.write().expect("Failed to lock bodies");

    bodies.remove(id).ok_or(BodyError::InvalidId(id))?;

    Ok(())
  }
}
