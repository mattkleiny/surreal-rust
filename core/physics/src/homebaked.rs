//! A home baked physics engine for simple use cases.

use super::*;

/// The backend for the homebaked physics engine.
#[derive(Default)]
pub struct DefaultPhysicsBackend;

impl PhysicsBackend for DefaultPhysicsBackend {
  fn create_world_2d(&self) -> Result<Box<PhysicsWorld2D>, WorldError> {
    Ok(Box::new(world2d::PhysicsWorld2D::default()))
  }

  fn create_world_3d(&self) -> Result<Box<PhysicsWorld3D>, WorldError> {
    Ok(Box::new(world3d::PhysicsWorld3D::default()))
  }
}

mod world2d {
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
    position: Vec2,
    shape: ColliderShape,
  }

  /// A 2D collider shape.
  enum ColliderShape {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
  }

  /// A 2D physics body.
  struct Body {
    position: Vec2,
    velocity: Vec2,
    kind: BodyKind,
  }

  /// A 2D physics body kind.
  enum BodyKind {
    Static,
    Dynamic,
  }

  impl PhysicsWorld for PhysicsWorld2D {
    type Vector = Vec2;

    fn tick(&self, _delta: f32) {
      // TODO: Implement physics simulation.
    }

    fn collider_create(&self) -> Result<ColliderId, ColliderError> {
      let mut colliders = self.colliders.write().unwrap();

      Ok(colliders.insert(Collider {
        shape: ColliderShape::Circle { radius: 1.0 },
        position: Vec2::ZERO,
      }))
    }

    fn collider_delete(&self, id: ColliderId) -> Result<(), ColliderError> {
      let mut colliders = self.colliders.write().unwrap();

      colliders.remove(id).ok_or(ColliderError::InvalidId(id))?;

      Ok(())
    }

    fn body_create(&self) -> Result<BodyId, BodyError> {
      let mut bodies = self.bodies.write().unwrap();

      Ok(bodies.insert(Body {
        position: Vec2::ZERO,
        velocity: Vec2::ZERO,
        kind: BodyKind::Dynamic,
      }))
    }

    fn body_delete(&self, id: BodyId) -> Result<(), BodyError> {
      let mut bodies = self.bodies.write().unwrap();

      bodies.remove(id).ok_or(BodyError::InvalidId(id))?;

      Ok(())
    }
  }
}

mod world3d {
  use super::*;

  /// A 3D physics world.
  #[derive(Default)]
  pub struct PhysicsWorld3D {}

  impl PhysicsWorld for PhysicsWorld3D {
    type Vector = Vec3;

    fn tick(&self, _delta: f32) {
      todo!()
    }

    fn collider_create(&self) -> Result<ColliderId, ColliderError> {
      todo!()
    }

    fn collider_delete(&self, _id: ColliderId) -> Result<(), ColliderError> {
      todo!()
    }

    fn body_create(&self) -> Result<BodyId, BodyError> {
      todo!()
    }

    fn body_delete(&self, _id: BodyId) -> Result<(), BodyError> {
      todo!()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_physics_world() {
    let world = DefaultPhysicsBackend.create_world_2d().unwrap();
    let collider_id = world.collider_create().unwrap();

    world.tick(0.16);
    world.tick(0.16);

    world.collider_delete(collider_id).unwrap();
  }
}
