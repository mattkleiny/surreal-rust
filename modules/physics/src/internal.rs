use super::*;

/// The default, home-baked [`PhysicsBackend`].
///
/// This is a simple physics backend that uses a simple Euler integration
/// algorithm to simulate the physics of the game world.
#[derive(Default)]
pub struct InternalPhysicsBackend {}

#[allow(unused_variables)]
impl PhysicsBackend for InternalPhysicsBackend {
  fn create_world_2d(&self) -> Box<dyn PhysicsWorld2D> {
    Box::new(dim2::InternalWorld2D::default())
  }

  fn create_world_3d(&self) -> Box<dyn PhysicsWorld3D> {
    todo!()
  }
}

mod dim2 {
  use std::sync::RwLock;

  use common::{Arena, FastHashSet};

  use super::*;

  /// The 2d physics world implementation.
  #[derive(Default)]
  pub struct InternalWorld2D {
    bodies: RwLock<Arena<BodyId, Body>>,
    colliders: RwLock<Arena<ColliderId, Collider>>,
    effectors: RwLock<Arena<EffectorId, Effector>>,
  }

  /// A body in the 2d physics world.
  struct Body {
    position: Vec2,
    rotation: f32,
    scale: Vec2,
    velocity: Vec2,
    kind: BodyKind,
    colliders: FastHashSet<ColliderId>,
  }

  /// A collider in the 2d physics world.
  struct Collider {
    position: Vec2,
    rotation: f32,
    scale: Vec2,
    shape: ColliderShape,
    kind: ColliderKind,
    bodies: FastHashSet<BodyId>,
  }

  /// A shape for a 2d collider.
  enum ColliderShape {
    Sphere { radius: f32 },
    Box { size: Vec2 },
    Capsule { radius: f32, height: f32 },
    Cylinder { radius: f32, height: f32 },
    Cone { radius: f32, height: f32 },
    ConvexHull { vertices: Vec<Vec2> },
    TriangleMesh { vertices: Vec<Vec2>, indices: Vec<u32> },
    HeightField { size: Vec2, heights: Vec<f32> },
  }

  /// An effector in the 2d physics world.
  struct Effector {
    position: Vec2,
    rotation: f32,
    scale: Vec2,
    kind: EffectorKind,
  }

  /// The shape of a 2d effector.
  enum EffectorShape {
    Sphere { radius: f32 },
    Box { size: Vec2 },
    Capsule { radius: f32, height: f32 },
    Cylinder { radius: f32, height: f32 },
  }

  impl PhysicsWorld for InternalWorld2D {
    fn step(&self, delta_time: f32) {
      let gravity = Vec2::new(0.0, -9.81);
      let mut bodies = self.bodies.write().unwrap();

      // integrate forces
      for (_, body) in bodies.enumerate_mut() {
        body.velocity += gravity * delta_time;
        body.position += body.velocity;
      }
    }

    fn reset(&self) {
      self.bodies.write().unwrap().clear();
      self.colliders.write().unwrap().clear();
      self.effectors.write().unwrap().clear();
    }
  }

  #[allow(unused_variables)]
  impl PhysicsWorld2D for InternalWorld2D {
    fn body_create(&self, kind: BodyKind, initial_position: Vec2) -> BodyId {
      todo!()
    }

    fn body_add_collider(&self, body: BodyId, collider: ColliderId) {
      todo!()
    }

    fn body_remove_collider(&self, body: BodyId, collider: ColliderId) {
      todo!()
    }

    fn body_set_position(&self, body: BodyId, position: Vec2) {
      todo!()
    }

    fn body_get_position(&self, body: BodyId) -> Vec2 {
      todo!()
    }

    fn body_set_rotation(&self, body: BodyId, rotation: f32) {
      todo!()
    }

    fn body_get_rotation(&self, body: BodyId) -> f32 {
      todo!()
    }

    fn body_set_scale(&self, body: BodyId, scale: Vec2) {
      todo!()
    }

    fn body_get_scale(&self, body: BodyId) -> Vec2 {
      todo!()
    }

    fn body_set_velocity(&self, body: BodyId, velocity: Vec2) {
      todo!()
    }

    fn body_get_velocity(&self, body: BodyId) -> Vec2 {
      todo!()
    }

    fn body_set_angular_velocity(&self, body: BodyId, velocity: Vec2) {
      todo!()
    }

    fn body_get_angular_velocity(&self, body: BodyId) -> Vec2 {
      todo!()
    }

    fn body_delete(&self, body: BodyId) {
      todo!()
    }

    fn collider_create_circle(&self, kind: ColliderKind, initial_position: Vec2, radius: f32) -> ColliderId {
      todo!()
    }

    fn collider_create_rectangle(&self, kind: ColliderKind, initial_position: Vec2, size: Vec2) -> ColliderId {
      todo!()
    }

    fn collider_create_triangle_mesh(
      &self,
      kind: ColliderKind,
      initial_position: Vec2,
      vertices: &[Vec2],
      indices: &[u32],
    ) -> ColliderId {
      todo!()
    }

    fn collider_create_height_field(
      &self,
      kind: ColliderKind,
      initial_position: Vec2,
      size: Vec2,
      heights: &[f32],
    ) -> ColliderId {
      todo!()
    }

    fn collider_get_kind(&self, collider: ColliderId) -> ColliderKind {
      todo!()
    }

    fn collider_set_position(&self, collider: ColliderId, position: Vec2) {
      todo!()
    }

    fn collider_get_position(&self, collider: ColliderId) -> Vec2 {
      todo!()
    }

    fn collider_set_rotation(&self, collider: ColliderId, rotation: f32) {
      todo!()
    }

    fn collider_get_rotation(&self, collider: ColliderId) -> f32 {
      todo!()
    }

    fn collider_set_scale(&self, collider: ColliderId, scale: Vec2) {
      todo!()
    }

    fn collider_get_scale(&self, collider: ColliderId) -> Vec2 {
      todo!()
    }

    fn collider_delete(&self, collider: ColliderId) {
      todo!()
    }

    fn effector_create_sphere(&self, kind: EffectorKind, initial_position: Vec2, radius: f32) -> EffectorId {
      todo!()
    }

    fn effector_create_box(&self, kind: EffectorKind, initial_position: Vec2, size: Vec2) -> EffectorId {
      todo!()
    }

    fn effector_create_capsule(
      &self,
      kind: EffectorKind,
      initial_position: Vec2,
      radius: f32,
      height: f32,
    ) -> EffectorId {
      todo!()
    }

    fn effector_create_cylinder(
      &self,
      kind: EffectorKind,
      initial_position: Vec2,
      radius: f32,
      height: f32,
    ) -> EffectorId {
      todo!()
    }

    fn effector_get_kind(&self, effector: EffectorId) -> EffectorKind {
      todo!()
    }

    fn effector_set_position(&self, effector: EffectorId, position: Vec2) {
      todo!()
    }

    fn effector_get_position(&self, effector: EffectorId) -> Vec2 {
      todo!()
    }

    fn effector_set_rotation(&self, effector: EffectorId, rotation: f32) {
      todo!()
    }

    fn effector_get_rotation(&self, effector: EffectorId) -> f32 {
      todo!()
    }

    fn effector_set_scale(&self, effector: EffectorId, scale: Vec2) {
      todo!()
    }

    fn effector_get_scale(&self, effector: EffectorId) -> Vec2 {
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
}

mod dim3 {
  /// A 3d physics world.
  struct InternalWorld3D {}
}
