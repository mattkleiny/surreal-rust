//! Physics engine for Surreal.
//!
//! This module provides a physics engine for Surreal. The physics engine is
//! lightweight and is written entirely in Rust.

use surreal::collections::ResourceStorage;

mod internal;

surreal::impl_rid!(ColliderId);
surreal::impl_rid!(RigidbodyId);
surreal::impl_rid!(EffectorId);

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

surreal::impl_server!(PhysicsEngine, PhysicsBackend);

impl PhysicsEngine {
  /// Creates a new [`PhysicsEngine`] with the internal backend.
  pub fn create_internal() -> Self {
    Self::new(internal::InternalPhysicsBackend::default())
  }
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

  // rigid-bodies
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
