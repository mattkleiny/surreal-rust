//! Physics engine for Surreal.

#![allow(dead_code)]

use common::Vec2;

mod bodies;
mod colliders;
mod effectors;
mod internal;
mod joints;
mod materials;

pub use bodies::*;
pub use colliders::*;
pub use effectors::*;
pub use joints::*;
pub use materials::*;

common::impl_arena_index!(ColliderId, "Identifies a collider.");
common::impl_arena_index!(BodyId, "Identifies a physics body.");
common::impl_arena_index!(EffectorId, "Identifies an effector.");
common::impl_arena_index!(JointId, "Identifies a joint.");
common::impl_arena_index!(MaterialId, "Identifies a physics material.");

common::impl_server!(PhysicsEngine, PhysicsBackend);

impl PhysicsEngine {
  /// Creates a new [`PhysicsEngine`] with the home-baked backend.
  pub fn homebaked() -> Self {
    Self::new(internal::InternalPhysicsBackend::default())
  }
}

/// Possible kinds of physics bodies.
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

/// A possible error when interacting with bodies.
#[derive(Debug)]
pub enum BodyError {
  InvalidId(BodyId),
}

/// A possible error when interacting with colliders.
#[derive(Debug)]
pub enum ColliderError {
  InvalidId(ColliderId),
}

/// A possible error when interacting with effectors.
#[derive(Debug)]
pub enum EffectorError {
  InvalidId(EffectorId),
}

/// A possible error when interacting with joints.
#[derive(Debug)]
pub enum JointError {
  InvalidId(JointId),
}

/// A possible error when interacting with materials.
#[derive(Debug)]
pub enum MaterialError {
  InvalidId(MaterialId),
}

/// A trait for physics backends.
///
/// This trait is implemented by physics backends, which are responsible for
/// simulating the physics of the game world.
///
/// The physics world abstraction is split into two traits, [`PhysicsWorld2D`]
/// and [`PhysicsWorld3D`], which represent 2D and 3D physics worlds, each of
/// which have their own APIs for creating and manipulating physics objects.
pub trait PhysicsBackend {
  fn create_world_2d(&self) -> Box<dyn PhysicsWorld2D>;
  fn create_world_3d(&self) -> Box<dyn PhysicsWorld3D>;
}

/// Represents a world of physics.
///
/// This trait is implemented by physics worlds, which are responsible for
/// simulating the physics of the game world.
///
/// A world needs to be created and updated by calling the `step` method with
/// the time since the last frame. The physics engine will then simulate the
/// physics of the world for that amount of time and integrate the results in
/// the physics objects.
pub trait PhysicsWorld {
  fn step(&self, delta_time: f32);
  fn reset(&self);
}

/// A world of 2D physics.
#[rustfmt::skip]
pub trait PhysicsWorld2D: PhysicsWorld {
  // settings
  fn set_gravity(&self, gravity: Vec2);
  fn get_gravity(&self) -> Vec2;

  // bodies
  fn body_create(&self, kind: BodyKind, initial_position: Vec2) -> Result<BodyId, BodyError>;
  fn body_add_collider(&self, body: BodyId, collider: ColliderId) -> Result<(), BodyError>;
  fn body_remove_collider(&self, body: BodyId, collider: ColliderId) -> Result<(), BodyError>;
  fn body_set_kind(&self, body: BodyId, kind: BodyKind) -> Result<(), BodyError>;
  fn body_get_kind(&self, body: BodyId) -> Option<BodyKind>;
  fn body_set_position(&self, body: BodyId, position: Vec2) -> Result<(), BodyError>;
  fn body_get_position(&self, body: BodyId) -> Option<Vec2>;
  fn body_set_rotation(&self, body: BodyId, rotation: f32) -> Result<(), BodyError>;
  fn body_get_rotation(&self, body: BodyId) -> Option<f32>;
  fn body_set_scale(&self, body: BodyId, scale: Vec2) -> Result<(), BodyError>;
  fn body_get_scale(&self, body: BodyId) -> Option<Vec2>;
  fn body_set_velocity(&self, body: BodyId, velocity: Vec2) -> Result<(), BodyError>;
  fn body_get_velocity(&self, body: BodyId) -> Option<Vec2>;
  fn body_set_angular_velocity(&self, body: BodyId, velocity: Vec2) -> Result<(), BodyError>;
  fn body_get_angular_velocity(&self, body: BodyId) -> Option<Vec2>;
  fn body_set_material(&self, body: BodyId, material: Option<MaterialId>) -> Result<(), BodyError>;
  fn body_get_material(&self, body: BodyId) -> Option<MaterialId>;
  fn body_delete(&self, body: BodyId) -> Result<(), BodyError>;

  // colliders
  fn collider_create_circle(&self, kind: ColliderKind, initial_position: Vec2, radius: f32) -> Result<ColliderId, ColliderError>;
  fn collider_create_rectangle(&self, kind: ColliderKind, initial_position: Vec2, size: Vec2) -> Result<ColliderId, ColliderError>;
  fn collider_create_triangle_mesh(&self, kind: ColliderKind, initial_position: Vec2, vertices: &[Vec2], indices: &[u32]) -> Result<ColliderId, ColliderError>;
  fn collider_create_height_field(&self, kind: ColliderKind, initial_position: Vec2, size: Vec2, heights: &[f32]) -> Result<ColliderId, ColliderError>;
  fn collider_get_kind(&self, collider: ColliderId) -> Option<ColliderKind>;
  fn collider_set_position(&self, collider: ColliderId, position: Vec2) -> Result<(), ColliderError>;
  fn collider_get_position(&self, collider: ColliderId) -> Option<Vec2>;
  fn collider_set_rotation(&self, collider: ColliderId, rotation: f32) -> Result<(), ColliderError>;
  fn collider_get_rotation(&self, collider: ColliderId) -> Option<f32>;
  fn collider_set_scale(&self, collider: ColliderId, scale: Vec2) -> Result<(), ColliderError>;
  fn collider_get_scale(&self, collider: ColliderId) -> Option<Vec2>;
  fn collider_delete(&self, collider: ColliderId) -> Result<(), ColliderError>;

  // effectors
  fn effector_create_sphere(&self, kind: EffectorKind, initial_position: Vec2, radius: f32) -> Result<EffectorId, EffectorError>;
  fn effector_create_box(&self, kind: EffectorKind, initial_position: Vec2, size: Vec2) -> Result<EffectorId, EffectorError>;
  fn effector_create_capsule(&self, kind: EffectorKind, initial_position: Vec2, radius: f32, height: f32) -> Result<EffectorId, EffectorError>;
  fn effector_create_cylinder(&self, kind: EffectorKind, initial_position: Vec2, radius: f32, height: f32) -> Result<EffectorId, EffectorError>;
  fn effector_get_kind(&self, effector: EffectorId) -> Option<EffectorKind>;
  fn effector_set_position(&self, effector: EffectorId, position: Vec2) -> Result<(), EffectorError>;
  fn effector_get_position(&self, effector: EffectorId) -> Option<Vec2>;
  fn effector_set_rotation(&self, effector: EffectorId, rotation: f32) -> Result<(), EffectorError>;
  fn effector_get_rotation(&self, effector: EffectorId) -> Option<f32>;
  fn effector_set_scale(&self, effector: EffectorId, scale: Vec2) -> Result<(), EffectorError>;
  fn effector_get_scale(&self, effector: EffectorId) -> Option<Vec2>;
  fn effector_set_strength(&self, effector: EffectorId, strength: f32) -> Result<(), EffectorError>;
  fn effector_get_strength(&self, effector: EffectorId) -> Option<f32>;
  fn effector_delete(&self, effector: EffectorId) -> Result<(), EffectorError>;

  // joints
  fn joint_create(&self) -> Result<JointId, JointError>;
  fn joint_attach(&self, joint: JointId, body_a: BodyId, body_b: BodyId) -> Result<(), JointError>;
  fn joint_detach(&self, joint: JointId) -> Result<(), JointError>;
  fn joint_get_bodies(&self, joint: JointId) -> Option<(BodyId, BodyId)>;
  fn joint_set_anchor(&self, joint: JointId, anchor: Vec2) -> Result<(), JointError>;
  fn joint_get_anchor(&self, joint: JointId) -> Option<Vec2>;
  fn joint_delete(&self, joint: JointId) -> Result<(), JointError>;

  // materials
  fn material_create(&self) -> Result<MaterialId, MaterialError>;
  fn material_set_friction(&self, material: MaterialId, friction: f32) -> Result<(), MaterialError>;
  fn material_get_friction(&self, material: MaterialId) -> Option<f32>;
  fn material_set_restitution(&self, material: MaterialId, restitution: f32) -> Result<(), MaterialError>;
  fn material_get_restitution(&self, material: MaterialId) -> Option<f32>;
  fn material_delete(&self, material: MaterialId) -> Result<(), MaterialError>;
}

/// A world of 3D physics.
#[rustfmt::skip]
pub trait PhysicsWorld3D: PhysicsWorld {
}
