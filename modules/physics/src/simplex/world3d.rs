use common::profiling;

use super::*;

/// A 3d physics world.
#[derive(Default)]
pub struct SimplexWorld3D {}

#[allow(unused_variables)]
impl PhysicsWorld for SimplexWorld3D {
  #[profiling]
  fn step(&self, delta_time: f32) {
    todo!()
  }

  #[profiling]
  fn reset(&self) {
    todo!()
  }
}

#[allow(unused_variables)]
impl PhysicsWorld3D for SimplexWorld3D {
  #[profiling]
  fn body_create(&self, kind: BodyKind, initial_position: Vec3) -> BodyId {
    todo!()
  }

  #[profiling]
  fn body_add_collider(&self, body: BodyId, collider: ColliderId) {
    todo!()
  }

  #[profiling]
  fn body_remove_collider(&self, body: BodyId, collider: ColliderId) {
    todo!()
  }

  #[profiling]
  fn body_set_position(&self, body: BodyId, position: Vec3) {
    todo!()
  }

  #[profiling]
  fn body_get_position(&self, body: BodyId) -> Vec3 {
    todo!()
  }

  #[profiling]
  fn body_set_rotation(&self, body: BodyId, rotation: Quat) {
    todo!()
  }

  #[profiling]
  fn body_get_rotation(&self, body: BodyId) -> Quat {
    todo!()
  }

  #[profiling]
  fn body_set_scale(&self, body: BodyId, scale: Vec3) {
    todo!()
  }

  #[profiling]
  fn body_get_scale(&self, body: BodyId) -> Vec3 {
    todo!()
  }

  #[profiling]
  fn body_set_velocity(&self, body: BodyId, velocity: Vec3) {
    todo!()
  }

  #[profiling]
  fn body_get_velocity(&self, body: BodyId) -> Vec3 {
    todo!()
  }

  #[profiling]
  fn body_set_angular_velocity(&self, body: BodyId, velocity: Vec3) {
    todo!()
  }

  #[profiling]
  fn body_get_angular_velocity(&self, body: BodyId) -> Vec3 {
    todo!()
  }

  #[profiling]
  fn body_delete(&self, body: BodyId) {
    todo!()
  }

  #[profiling]
  fn collider_create_sphere(&self, kind: ColliderKind, initial_position: Vec3, radius: f32) -> ColliderId {
    todo!()
  }

  #[profiling]
  fn collider_create_box(&self, kind: ColliderKind, initial_position: Vec3, size: Vec3) -> ColliderId {
    todo!()
  }

  #[profiling]
  fn collider_create_capsule(
    &self,
    kind: ColliderKind,
    initial_position: Vec3,
    radius: f32,
    height: f32,
  ) -> ColliderId {
    todo!()
  }

  #[profiling]
  fn collider_create_cylinder(
    &self,
    kind: ColliderKind,
    initial_position: Vec3,
    radius: f32,
    height: f32,
  ) -> ColliderId {
    todo!()
  }

  #[profiling]
  fn collider_create_cone(&self, kind: ColliderKind, initial_position: Vec3, radius: f32, height: f32) -> ColliderId {
    todo!()
  }

  #[profiling]
  fn collider_create_convex_hull(&self, kind: ColliderKind, initial_position: Vec3, vertices: &[Vec3]) -> ColliderId {
    todo!()
  }

  #[profiling]
  fn collider_create_triangle_mesh(
    &self,
    kind: ColliderKind,
    initial_position: Vec3,
    vertices: &[Vec3],
    indices: &[u32],
  ) -> ColliderId {
    todo!()
  }

  #[profiling]
  fn collider_create_height_field(
    &self,
    kind: ColliderKind,
    initial_position: Vec3,
    size: Vec3,
    heights: &[f32],
  ) -> ColliderId {
    todo!()
  }

  #[profiling]
  fn collider_get_kind(&self, collider: ColliderId) -> ColliderKind {
    todo!()
  }

  #[profiling]
  fn collider_set_position(&self, collider: ColliderId, position: Vec3) {
    todo!()
  }

  #[profiling]
  fn collider_get_position(&self, collider: ColliderId) -> Vec3 {
    todo!()
  }

  #[profiling]
  fn collider_set_rotation(&self, collider: ColliderId, rotation: Quat) {
    todo!()
  }

  #[profiling]
  fn collider_get_rotation(&self, collider: ColliderId) -> Quat {
    todo!()
  }

  #[profiling]
  fn collider_set_scale(&self, collider: ColliderId, scale: Vec3) {
    todo!()
  }

  #[profiling]
  fn collider_get_scale(&self, collider: ColliderId) -> Vec3 {
    todo!()
  }

  #[profiling]
  fn collider_delete(&self, collider: ColliderId) {
    todo!()
  }

  #[profiling]
  fn effector_create_sphere(&self, kind: EffectorKind, initial_position: Vec3, radius: f32) -> EffectorId {
    todo!()
  }

  #[profiling]
  fn effector_create_box(&self, kind: EffectorKind, initial_position: Vec3, size: Vec3) -> EffectorId {
    todo!()
  }

  #[profiling]
  fn effector_create_capsule(
    &self,
    kind: EffectorKind,
    initial_position: Vec3,
    radius: f32,
    height: f32,
  ) -> EffectorId {
    todo!()
  }

  #[profiling]
  fn effector_create_cylinder(
    &self,
    kind: EffectorKind,
    initial_position: Vec3,
    radius: f32,
    height: f32,
  ) -> EffectorId {
    todo!()
  }

  #[profiling]
  fn effector_get_kind(&self, effector: EffectorId) -> EffectorKind {
    todo!()
  }

  #[profiling]
  fn effector_set_position(&self, effector: EffectorId, position: Vec3) {
    todo!()
  }

  #[profiling]
  fn effector_get_position(&self, effector: EffectorId) -> Vec3 {
    todo!()
  }

  #[profiling]
  fn effector_set_rotation(&self, effector: EffectorId, rotation: Quat) {
    todo!()
  }

  #[profiling]
  fn effector_get_rotation(&self, effector: EffectorId) -> Quat {
    todo!()
  }

  #[profiling]
  fn effector_set_scale(&self, effector: EffectorId, scale: Vec3) {
    todo!()
  }

  #[profiling]
  fn effector_get_scale(&self, effector: EffectorId) -> Vec3 {
    todo!()
  }

  #[profiling]
  fn effector_set_strength(&self, effector: EffectorId, strength: f32) {
    todo!()
  }

  #[profiling]
  fn effector_get_strength(&self, effector: EffectorId) -> f32 {
    todo!()
  }

  #[profiling]
  fn effector_delete(&self, effector: EffectorId) {
    todo!()
  }
}
