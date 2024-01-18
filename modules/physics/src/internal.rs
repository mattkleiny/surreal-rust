use super::*;

/// The default, home-baked [`PhysicsBackend`].
///
/// This is a simple physics backend that uses a simple Euler integration
/// algorithm to simulate the physics of the game world.
#[derive(Default)]
pub struct InternalPhysicsBackend {
  rigidbodies: ResourceArena<RigidbodyId, Rigidbody>,
  colliders: ResourceArena<ColliderId, Collider>,
  effectors: ResourceArena<EffectorId, Effector>,
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

#[allow(unused_variables)]
impl PhysicsBackend for InternalPhysicsBackend {
  fn step(&self, delta_time: f32) {
    todo!()
  }

  fn reset(&self) {
    todo!()
  }

  fn rigidbody_create(&self, kind: RigidbodyKind, initial_position: Vec3) -> RigidbodyId {
    todo!()
  }

  fn rigidbody_add_collider(&self, body: RigidbodyId, collider: ColliderId) {
    todo!()
  }

  fn rigidbody_remove_collider(&self, body: RigidbodyId, collider: ColliderId) {
    todo!()
  }

  fn rigidbody_set_position(&self, body: RigidbodyId, position: Vec3) {
    todo!()
  }

  fn rigidbody_get_position(&self, body: RigidbodyId) -> Vec3 {
    todo!()
  }

  fn rigidbody_set_rotation(&self, body: RigidbodyId, rotation: Quat) {
    todo!()
  }

  fn rigidbody_get_rotation(&self, body: RigidbodyId) -> Quat {
    todo!()
  }

  fn rigidbody_set_scale(&self, body: RigidbodyId, scale: Vec3) {
    todo!()
  }

  fn rigidbody_get_scale(&self, body: RigidbodyId) -> Vec3 {
    todo!()
  }

  fn rigidbody_set_velocity(&self, body: RigidbodyId, velocity: Vec3) {
    todo!()
  }

  fn rigidbody_get_velocity(&self, body: RigidbodyId) -> Vec3 {
    todo!()
  }

  fn rigidbody_set_angular_velocity(&self, body: RigidbodyId, velocity: Vec3) {
    todo!()
  }

  fn rigidbody_get_angular_velocity(&self, body: RigidbodyId) -> Vec3 {
    todo!()
  }

  fn rigidbody_delete(&self, body: RigidbodyId) {
    todo!()
  }

  fn collider_create_sphere(&self, initial_position: Vec3, radius: f32) -> ColliderId {
    todo!()
  }

  fn collider_create_box(&self, initial_position: Vec3, size: Vec3) -> ColliderId {
    todo!()
  }

  fn collider_create_capsule(&self, initial_position: Vec3, radius: f32, height: f32) -> ColliderId {
    todo!()
  }

  fn collider_create_cylinder(&self, initial_position: Vec3, radius: f32, height: f32) -> ColliderId {
    todo!()
  }

  fn collider_create_cone(&self, initial_position: Vec3, radius: f32, height: f32) -> ColliderId {
    todo!()
  }

  fn collider_create_convex_hull(&self, initial_position: Vec3, vertices: &[Vec3]) -> ColliderId {
    todo!()
  }

  fn collider_create_triangle_mesh(&self, initial_position: Vec3, vertices: &[Vec3], indices: &[u32]) -> ColliderId {
    todo!()
  }

  fn collider_create_height_field(&self, initial_position: Vec3, size: Vec3, heights: &[f32]) -> ColliderId {
    todo!()
  }

  fn collider_get_kind(&self, collider: ColliderId) -> ColliderKind {
    todo!()
  }

  fn collider_set_position(&self, collider: ColliderId, position: Vec3) {
    todo!()
  }

  fn collider_get_position(&self, collider: ColliderId) -> Vec3 {
    todo!()
  }

  fn collider_set_rotation(&self, collider: ColliderId, rotation: Quat) {
    todo!()
  }

  fn collider_get_rotation(&self, collider: ColliderId) -> Quat {
    todo!()
  }

  fn collider_set_scale(&self, collider: ColliderId, scale: Vec3) {
    todo!()
  }

  fn collider_get_scale(&self, collider: ColliderId) -> Vec3 {
    todo!()
  }

  fn collider_delete(&self, collider: ColliderId) {
    todo!()
  }

  fn effector_create_wind(&self, initial_position: Vec3) -> EffectorId {
    todo!()
  }

  fn effector_create_gravity(&self, initial_position: Vec3) -> EffectorId {
    todo!()
  }

  fn effector_get_kind(&self, effector: EffectorId) -> EffectorKind {
    todo!()
  }

  fn effector_set_position(&self, effector: EffectorId, position: Vec3) {
    todo!()
  }

  fn effector_get_position(&self, effector: EffectorId) -> Vec3 {
    todo!()
  }

  fn effector_set_rotation(&self, effector: EffectorId, rotation: Quat) {
    todo!()
  }

  fn effector_get_rotation(&self, effector: EffectorId) -> Quat {
    todo!()
  }

  fn effector_set_scale(&self, effector: EffectorId, scale: Vec3) {
    todo!()
  }

  fn effector_get_scale(&self, effector: EffectorId) -> Vec3 {
    todo!()
  }

  fn effector_set_strength(&self, effector: EffectorId, strength: f32) {
    todo!()
  }

  fn effector_get_strength(&self, effector: EffectorId) -> f32 {
    todo!()
  }

  fn effector_delete(&self, effector: EffectorId) {
    todo!()
  }
}
