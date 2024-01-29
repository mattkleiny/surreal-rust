//! Physics engine for Surreal.

#![allow(dead_code)]

use common::{Quat, Vec3};

mod internal;

common::impl_arena_index!(ColliderId, "Identifies a collider.");
common::impl_arena_index!(BodyId, "Identifies a physics body.");
common::impl_arena_index!(EffectorId, "Identifies an effector.");

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
pub enum BodyKind {
  Static,
  Dynamic,
  Kinematic,
}

/// Possible kinds of colliders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColliderKind {
  Trigger,
  Solid,
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
#[rustfmt::skip]
pub trait PhysicsBackend {
  // core
  fn step(&self, delta_time: f32);
  fn reset(&self);

  // rigidodies
  fn body_create(&self, kind: BodyKind, initial_position: Vec3) -> BodyId;
  fn body_add_collider(&self, body: BodyId, collider: ColliderId);
  fn body_remove_collider(&self, body: BodyId, collider: ColliderId);
  fn body_set_position(&self, body: BodyId, position: Vec3);
  fn body_get_position(&self, body: BodyId) -> Vec3;
  fn body_set_rotation(&self, body: BodyId, rotation: Quat);
  fn body_get_rotation(&self, body: BodyId) -> Quat;
  fn body_set_scale(&self, body: BodyId, scale: Vec3);
  fn body_get_scale(&self, body: BodyId) -> Vec3;
  fn body_set_velocity(&self, body: BodyId, velocity: Vec3);
  fn body_get_velocity(&self, body: BodyId) -> Vec3;
  fn body_set_angular_velocity(&self, body: BodyId, velocity: Vec3);
  fn body_get_angular_velocity(&self, body: BodyId) -> Vec3;
  fn body_delete(&self, body: BodyId);

  // colliders
  fn collider_create_sphere(&self, kind: ColliderKind, initial_position: Vec3, radius: f32) -> ColliderId;
  fn collider_create_box(&self, kind: ColliderKind, initial_position: Vec3, size: Vec3) -> ColliderId;
  fn collider_create_capsule(&self, kind: ColliderKind, initial_position: Vec3, radius: f32, height: f32) -> ColliderId;
  fn collider_create_cylinder(&self, kind: ColliderKind, initial_position: Vec3, radius: f32, height: f32) -> ColliderId;
  fn collider_create_cone(&self, kind: ColliderKind, initial_position: Vec3, radius: f32, height: f32) -> ColliderId;
  fn collider_create_convex_hull(&self, kind: ColliderKind, initial_position: Vec3, vertices: &[Vec3]) -> ColliderId;
  fn collider_create_triangle_mesh(&self, kind: ColliderKind, initial_position: Vec3, vertices: &[Vec3], indices: &[u32]) -> ColliderId;
  fn collider_create_height_field(&self, kind: ColliderKind, initial_position: Vec3, size: Vec3, heights: &[f32]) -> ColliderId;
  fn collider_get_kind(&self, collider: ColliderId) -> ColliderKind;
  fn collider_set_position(&self, collider: ColliderId, position: Vec3);
  fn collider_get_position(&self, collider: ColliderId) -> Vec3;
  fn collider_set_rotation(&self, collider: ColliderId, rotation: Quat);
  fn collider_get_rotation(&self, collider: ColliderId) -> Quat;
  fn collider_set_scale(&self, collider: ColliderId, scale: Vec3);
  fn collider_get_scale(&self, collider: ColliderId) -> Vec3;
  fn collider_delete(&self, collider: ColliderId);

  // effectors
  fn effector_create_sphere(&self, kind: EffectorKind, initial_position: Vec3, radius: f32) -> EffectorId;
  fn effector_create_box(&self, kind: EffectorKind, initial_position: Vec3, size: Vec3) -> EffectorId;
  fn effector_create_capsule(&self, kind: EffectorKind, initial_position: Vec3, radius: f32, height: f32) -> EffectorId;
  fn effector_create_cylinder(&self, kind: EffectorKind, initial_position: Vec3, radius: f32, height: f32) -> EffectorId;
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
