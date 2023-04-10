//! Physics engine for Surreal.
//!
//! This module provides a physics engine for Surreal. The physics engine is
//! lightweight and is written entirely in Rust.

use surreal::collections::ResourceStorage;

surreal::impl_server!(PhysicsEngine, PhysicsBackend);

surreal::impl_rid!(ColliderId);
surreal::impl_rid!(RigidbodyId);
surreal::impl_rid!(EffectorId);

impl PhysicsEngine {
  /// Creates a new physics server with the internal backend.
  pub fn create_internal() -> Self {
    Self::new(InternalPhysicsBackend::default())
  }
}

/// Possible kinds of rigidbodies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RigidbodyKind {
  Static,
  Dynamic,
  Kinematic,
}

/// Possible kinds of colliders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColliderKind {
  Sphere,
  Box,
  Capsule,
  Cylinder,
  Cone,
  ConvexHull,
  TriangleMesh,
  HeightField,
}

/// Possible kinds of effectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectorKind {
  Gravity,
  Wind,
}

/// A trait for physics backends.
///
/// This trait is implemented by physics backends, which are responsible for
/// simulating the physics of the game world.
///
/// The physics backend is responsible for simulating the physics of the game
/// world. It is responsible for updating the position and orientation of
/// physical objects, and for detecting collisions between objects.
pub trait PhysicsBackend {
  /// Advances the physics simulation by the given amount of time.
  fn step(&self, delta_time: f32);

  /// Resets the entire physics simulation.
  fn reset(&self);

  // rigidbodies
  fn rigidbody_create(&self, kind: RigidbodyKind) -> RigidbodyId;
  fn rigidbody_add_collider(&self, body: RigidbodyId, collider: ColliderId);
  fn rigidbody_remove_collider(&self, body: RigidbodyId, collider: ColliderId);
  fn rigidbody_delete(&self, body: RigidbodyId);

  // colliders
  fn collider_create(&self, kind: ColliderKind) -> ColliderId;
  fn collider_delete(&self, collider: ColliderId);

  // effectors
  fn effector_create(&self, kind: EffectorKind) -> EffectorId;
  fn effector_delete(&self, effector: EffectorId);
}

/// The default, home-baked [`PhysicsBackend`].
///
/// This is a simple physics backend that uses a simple Euler integration
/// algorithm to simulate the physics of the game world.
#[derive(Default)]
struct InternalPhysicsBackend {
  rigidbodies: ResourceStorage<RigidbodyId, Rigidbody>,
  colliders: ResourceStorage<ColliderId, Collider>,
  effectors: ResourceStorage<EffectorId, Effector>,
}

/// The internal representation of a rigidbody.
struct Rigidbody {
  _kind: RigidbodyKind,
  colliders: Vec<ColliderId>,
}

/// The internal representation of a collider.
struct Collider {
  _kind: ColliderKind,
}

/// The internal representation of an effector.
struct Effector {
  _kind: EffectorKind,
}

impl PhysicsBackend for InternalPhysicsBackend {
  fn step(&self, _delta_time: f32) {
    todo!()
  }

  fn reset(&self) {
    self.rigidbodies.clear();
    self.colliders.clear();
  }

  fn rigidbody_create(&self, kind: RigidbodyKind) -> RigidbodyId {
    self.rigidbodies.insert(Rigidbody {
      _kind: kind,
      colliders: Vec::new(),
    })
  }

  fn rigidbody_add_collider(&self, body: RigidbodyId, collider: ColliderId) {
    self.rigidbodies.write(body, |body| {
      body.colliders.push(collider);
    });
  }

  fn rigidbody_remove_collider(&self, body: RigidbodyId, collider: ColliderId) {
    self.rigidbodies.write(body, |body| {
      body.colliders.retain(|c| *c != collider);
    });
  }

  fn rigidbody_delete(&self, body: RigidbodyId) {
    self.rigidbodies.remove(body);
  }

  fn collider_create(&self, kind: ColliderKind) -> ColliderId {
    self.colliders.insert(Collider { _kind: kind })
  }

  fn collider_delete(&self, collider: ColliderId) {
    self.colliders.remove(collider);
  }

  fn effector_create(&self, kind: EffectorKind) -> EffectorId {
    self.effectors.insert(Effector { _kind: kind })
  }

  fn effector_delete(&self, effector: EffectorId) {
    self.effectors.remove(effector);
  }
}
