use std::sync::RwLock;

use common::{Arena, FastHashSet, vec2};

use super::*;

/// A 2d physics world.
#[derive(Default)]
pub struct SimplexWorld2D {
  settings: RwLock<Settings>,
  bodies: RwLock<Arena<BodyId, Body>>,
  colliders: RwLock<Arena<ColliderId, Collider>>,
  effectors: RwLock<Arena<EffectorId, Effector>>,
}

/// Internal settings for a physics world.
struct Settings {
  gravity: Vec2,
}

/// A body in the 2d physics world.
struct Body {
  position: Vec2,
  rotation: f32,
  scale: Vec2,
  velocity: Vec2,
  angular_velocity: Vec2,
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

impl Default for Settings {
  fn default() -> Self {
    Self {
      gravity: vec2(0., -9.8),
    }
  }
}

impl PhysicsWorld for SimplexWorld2D {
  fn step(&self, delta_time: f32) {
    let settings = self.settings.read().unwrap();
    let gravity = settings.gravity;

    self.integrate_bodies(delta_time, gravity);
  }

  fn reset(&self) {
    self.bodies.write().unwrap().clear();
    self.colliders.write().unwrap().clear();
    self.effectors.write().unwrap().clear();
  }
}

impl SimplexWorld2D {
  /// Integrates all bodies in the world.
  fn integrate_bodies(&self, delta_time: f32, gravity: Vec2) {
    let mut bodies = self.bodies.write().unwrap();

    for body in bodies.iter_mut() {
      body.velocity += gravity * delta_time;
      body.position += body.velocity;
    }
  }
}

#[allow(unused_variables)]
impl PhysicsWorld2D for SimplexWorld2D {
  fn body_create(&self, kind: BodyKind, initial_position: Vec2) -> BodyId {
    let mut bodies = self.bodies.write().unwrap();

    bodies.insert(Body {
      position: initial_position,
      rotation: 0.0,
      scale: Vec2::ONE,
      velocity: Vec2::ZERO,
      angular_velocity: Vec2::ZERO,
      kind,
      colliders: FastHashSet::default(),
    })
  }

  fn body_add_collider(&self, body: BodyId, collider: ColliderId) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.colliders.insert(collider);
    }
  }

  fn body_remove_collider(&self, body: BodyId, collider: ColliderId) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.colliders.remove(&collider);
    }
  }

  fn body_set_position(&self, body: BodyId, position: Vec2) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.position = position;
    }
  }

  fn body_get_position(&self, body: BodyId) -> Vec2 {
    let bodies = self.bodies.read().unwrap();

    if let Some(body) = bodies.get(body) {
      body.position
    } else {
      Vec2::ZERO
    }
  }

  fn body_set_rotation(&self, body: BodyId, rotation: f32) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.rotation = rotation;
    }
  }

  fn body_get_rotation(&self, body: BodyId) -> f32 {
    let bodies = self.bodies.read().unwrap();

    if let Some(body) = bodies.get(body) {
      body.rotation
    } else {
      0.0
    }
  }

  fn body_set_scale(&self, body: BodyId, scale: Vec2) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.scale = scale;
    }
  }

  fn body_get_scale(&self, body: BodyId) -> Vec2 {
    let bodies = self.bodies.read().unwrap();

    if let Some(body) = bodies.get(body) {
      body.scale
    } else {
      Vec2::ONE
    }
  }

  fn body_set_velocity(&self, body: BodyId, velocity: Vec2) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.velocity = velocity;
    }
  }

  fn body_get_velocity(&self, body: BodyId) -> Vec2 {
    let bodies = self.bodies.read().unwrap();

    if let Some(body) = bodies.get(body) {
      body.velocity
    } else {
      Vec2::ZERO
    }
  }

  fn body_set_angular_velocity(&self, body: BodyId, velocity: Vec2) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.angular_velocity = velocity;
    }
  }

  fn body_get_angular_velocity(&self, body: BodyId) -> Vec2 {
    let bodies = self.bodies.read().unwrap();

    if let Some(body) = bodies.get(body) {
      body.angular_velocity
    } else {
      Vec2::ZERO
    }
  }

  fn body_delete(&self, body: BodyId) {
    let mut bodies = self.bodies.write().unwrap();

    bodies.remove(body);
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
