//! Physics engine for Surreal.

#![allow(dead_code)]

use common::collections::ResourceArena;
use common::maths::{Quat, Vec3};

mod internal;

common::impl_rid!(ColliderId);
common::impl_rid!(RigidbodyId);
common::impl_rid!(EffectorId);

common::impl_server!(PhysicsEngine, PhysicsBackend);

impl PhysicsEngine {
  /// Creates a new [`PhysicsEngine`] with the internal backend.
  pub fn internal() -> Self {
    Self::new(internal::InternalPhysicsBackend::default())
  }

  /// Creates a new [`PhysicsEngine`] with the Bullet backend.
  pub fn bullet() -> Self {
    todo!()
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

  // rigid-bodies
  fn rigidbody_create(&self, kind: RigidbodyKind, initial_position: Vec3) -> RigidbodyId;
  fn rigidbody_add_collider(&self, body: RigidbodyId, collider: ColliderId);
  fn rigidbody_remove_collider(&self, body: RigidbodyId, collider: ColliderId);
  fn rigidbody_set_position(&self, body: RigidbodyId, position: Vec3);
  fn rigidbody_get_position(&self, body: RigidbodyId) -> Vec3;
  fn rigidbody_set_rotation(&self, body: RigidbodyId, rotation: Quat);
  fn rigidbody_get_rotation(&self, body: RigidbodyId) -> Quat;
  fn rigidbody_set_scale(&self, body: RigidbodyId, scale: Vec3);
  fn rigidbody_get_scale(&self, body: RigidbodyId) -> Vec3;
  fn rigidbody_set_velocity(&self, body: RigidbodyId, velocity: Vec3);
  fn rigidbody_get_velocity(&self, body: RigidbodyId) -> Vec3;
  fn rigidbody_set_angular_velocity(&self, body: RigidbodyId, velocity: Vec3);
  fn rigidbody_get_angular_velocity(&self, body: RigidbodyId) -> Vec3;
  fn rigidbody_delete(&self, body: RigidbodyId);

  // colliders
  fn collider_create_sphere(&self, initial_position: Vec3, radius: f32) -> ColliderId;
  fn collider_create_box(&self, initial_position: Vec3, size: Vec3) -> ColliderId;
  fn collider_create_capsule(&self, initial_position: Vec3, radius: f32, height: f32) -> ColliderId;
  fn collider_create_cylinder(&self, initial_position: Vec3, radius: f32, height: f32) -> ColliderId;
  fn collider_create_cone(&self, initial_position: Vec3, radius: f32, height: f32) -> ColliderId;
  fn collider_create_convex_hull(&self, initial_position: Vec3, vertices: &[Vec3]) -> ColliderId;
  fn collider_create_triangle_mesh(&self, initial_position: Vec3, vertices: &[Vec3], indices: &[u32]) -> ColliderId;
  fn collider_create_height_field(&self, initial_position: Vec3, size: Vec3, heights: &[f32]) -> ColliderId;
  fn collider_get_kind(&self, collider: ColliderId) -> ColliderKind;
  fn collider_set_position(&self, collider: ColliderId, position: Vec3);
  fn collider_get_position(&self, collider: ColliderId) -> Vec3;
  fn collider_set_rotation(&self, collider: ColliderId, rotation: Quat);
  fn collider_get_rotation(&self, collider: ColliderId) -> Quat;
  fn collider_set_scale(&self, collider: ColliderId, scale: Vec3);
  fn collider_get_scale(&self, collider: ColliderId) -> Vec3;
  fn collider_delete(&self, collider: ColliderId);

  // effectors
  fn effector_create_wind(&self, initial_position: Vec3) -> EffectorId;
  fn effector_create_gravity(&self, initial_position: Vec3) -> EffectorId;
  fn effector_get_kind(&self, effector: EffectorId) -> EffectorKind;
  fn effector_set_position(&self, effector: EffectorId, position: Vec3);
  fn effector_get_position(&self, effector: EffectorId) -> Vec3;
  fn effector_set_rotation(&self, effector: EffectorId, rotation: Quat);
  fn effector_get_rotation(&self, effector: EffectorId) -> Quat;
  fn effector_set_scale(&self, effector: EffectorId, scale: Vec3);
  fn effector_get_scale(&self, effector: EffectorId) -> Vec3;
  fn effector_set_strength(&self, effector: EffectorId, strength: f32);
  fn effector_get_strength(&self, effector: EffectorId) -> f32;
  fn effector_delete(&self, effector: EffectorId);
}
