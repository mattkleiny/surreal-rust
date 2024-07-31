//! Physics engine for Surreal.

use common::{Vec2, Vec3, Vector};

mod engine;

common::impl_arena_index!(pub ColliderId, "Identifies a collider.");
common::impl_arena_index!(pub BodyId, "Identifies a physics body.");

common::impl_server!(PhysicsServer by PhysicsBackend default engine::RustPhysicsBackend);

// Floating point precision used by the physics engine.
pub type Real = f32;
pub type Real2 = Vec2;
pub type Real3 = Vec3;

/// Gets the physics server instance.
#[inline(always)]
pub fn physics() -> &'static dyn PhysicsBackend {
  PhysicsServer::instance()
}

/// A possible error when interacting with physics worlds.
#[derive(Debug)]
pub enum WorldError {
  CreationFailed,
}

/// An error that can occur when interacting with colliders.
#[derive(Debug)]
pub enum ColliderError {
  CreationFailed,
  InvalidId(ColliderId),
  NullPointer,
}

/// An error that can occur when interacting with physics bodies.
#[derive(Debug)]
pub enum BodyError {
  CreationFailed,
  InvalidId(BodyId),
  NullPointer,
}

/// An abstraction on top of the underlying physics API.
///
/// This is a mid-level abstraction that makes use of 'opaque' resource IDs to
/// hide away implementation details and lifetimes. The backend forms the
/// foundation of higher-level abstractions that make it simpler to build
/// graphics programs.
pub trait PhysicsBackend {
  fn create_world_2d(&self) -> Result<Box<PhysicsWorld2D>, WorldError>;
  fn create_world_3d(&self) -> Result<Box<PhysicsWorld3D>, WorldError>;
}

pub type PhysicsWorld2D = dyn PhysicsWorld<Vector = Real2>;
pub type PhysicsWorld3D = dyn PhysicsWorld<Vector = Real3>;

/// A physics world that contains all the physics bodies and colliders.
///
/// This is the main entry point for interacting with the physics engine.
pub trait PhysicsWorld {
  type Vector: Vector;

  /// Steps the physics simulation by the given delta time.
  fn tick(&self, delta: f32);

  // colliders
  fn collider_create(&self) -> Result<ColliderId, ColliderError>;
  fn collider_get_position(&self, id: ColliderId) -> Result<Self::Vector, ColliderError>;
  fn collider_set_position(&self, id: ColliderId, position: Self::Vector) -> Result<(), ColliderError>;
  fn collider_delete(&self, id: ColliderId) -> Result<(), ColliderError>;

  // bodies
  fn body_create(&self) -> Result<BodyId, BodyError>;
  fn body_get_position(&self, id: BodyId) -> Result<Self::Vector, BodyError>;
  fn body_set_position(&self, id: BodyId, position: Self::Vector) -> Result<(), BodyError>;
  fn body_get_velocity(&self, id: BodyId) -> Result<Self::Vector, BodyError>;
  fn body_set_velocity(&self, id: BodyId, velocity: Self::Vector) -> Result<(), BodyError>;
  fn body_delete(&self, id: BodyId) -> Result<(), BodyError>;
}
