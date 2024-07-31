use std::sync::RwLock;

use common::Arena;

use super::*;

/// A 2D physics world.
#[derive(Default)]
pub struct PhysicsWorld2D {
  colliders: RwLock<Arena<ColliderId, Collider>>,
  bodies: RwLock<Arena<BodyId, Body>>,
}

/// A 2D collider.
struct Collider {
  position: Real2,
  shape: ColliderShape,
}

/// A 2D collider shape.
enum ColliderShape {
  Circle { radius: f32 },
  Rectangle { width: f32, height: f32 },
}

/// A 2D physics body.
struct Body {
  position: Real2,
  velocity: Real2,
  kind: BodyKind,
}

/// A 2D physics body kind.
enum BodyKind {
  Static,
  Dynamic,
}

impl PhysicsWorld for PhysicsWorld2D {
  type Vector = Real2;

  fn tick(&self, _delta: f32) {
    // TODO: Implement physics simulation.
  }

  fn collider_create(&self) -> Result<ColliderId, ColliderError> {
    let mut colliders = self.colliders.write().unwrap();

    Ok(colliders.insert(Collider {
      shape: ColliderShape::Circle { radius: 1.0 },
      position: Real2::ZERO,
    }))
  }

  fn collider_get_position(&self, id: ColliderId) -> Result<Self::Vector, ColliderError> {
    let colliders = self.colliders.read().unwrap();
    let collider = colliders.get(id).ok_or(ColliderError::InvalidId(id))?;

    Ok(collider.position)
  }

  fn collider_set_position(&self, id: ColliderId, position: Self::Vector) -> Result<(), ColliderError> {
    let mut colliders = self.colliders.write().unwrap();
    let collider = colliders.get_mut(id).ok_or(ColliderError::InvalidId(id))?;

    collider.position = position;

    Ok(())
  }

  fn collider_delete(&self, id: ColliderId) -> Result<(), ColliderError> {
    let mut colliders = self.colliders.write().unwrap();

    colliders.remove(id).ok_or(ColliderError::InvalidId(id))?;

    Ok(())
  }

  fn body_create(&self) -> Result<BodyId, BodyError> {
    let mut bodies = self.bodies.write().unwrap();

    Ok(bodies.insert(Body {
      position: Real2::ZERO,
      velocity: Real2::ZERO,
      kind: BodyKind::Dynamic,
    }))
  }

  fn body_get_position(&self, id: BodyId) -> Result<Self::Vector, BodyError> {
    let bodies = self.bodies.read().unwrap();
    let body = bodies.get(id).ok_or(BodyError::InvalidId(id))?;

    Ok(body.position)
  }

  fn body_set_position(&self, id: BodyId, position: Self::Vector) -> Result<(), BodyError> {
    let mut bodies = self.bodies.write().unwrap();
    let body = bodies.get_mut(id).ok_or(BodyError::InvalidId(id))?;

    body.position = position;

    Ok(())
  }

  fn body_get_velocity(&self, id: BodyId) -> Result<Self::Vector, BodyError> {
    let bodies = self.bodies.read().unwrap();
    let body = bodies.get(id).ok_or(BodyError::InvalidId(id))?;

    Ok(body.velocity)
  }

  fn body_set_velocity(&self, id: BodyId, velocity: Self::Vector) -> Result<(), BodyError> {
    let mut bodies = self.bodies.write().unwrap();
    let body = bodies.get_mut(id).ok_or(BodyError::InvalidId(id))?;

    body.velocity = velocity;

    Ok(())
  }

  fn body_delete(&self, id: BodyId) -> Result<(), BodyError> {
    let mut bodies = self.bodies.write().unwrap();

    bodies.remove(id).ok_or(BodyError::InvalidId(id))?;

    Ok(())
  }
}
