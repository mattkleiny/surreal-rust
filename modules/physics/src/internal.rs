use std::sync::RwLock;

use common::Arena;

use super::*;

/// The default, home-baked [`PhysicsBackend`].
///
/// This is a simple physics backend that uses a simple Euler integration
/// algorithm to simulate the physics of the game world.
#[derive(Default)]
pub struct InternalPhysicsBackend {
  bodies: RwLock<Arena<BodyId, Body>>,
  colliders: RwLock<Arena<ColliderId, Collider>>,
  effectors: RwLock<Arena<EffectorId, Effector>>,
}

/// The internal representation of a rigidbody.
struct Body {
  kind: BodyKind,
  position: Vec3,
  rotation: Quat,
  scale: Vec3,
  velocity: Vec3,
  angular_velocity: Vec3,
  colliders: Vec<ColliderId>,
}

/// The internal representation of a collider.
struct Collider {
  kind: ColliderKind,
  shape: ColliderShape,
  position: Vec3,
  rotation: Quat,
  scale: Vec3,
}

/// Internal representation of a collider shape.
enum ColliderShape {
  Sphere { radius: f32 },
  Box { size: Vec3 },
  Capsule { radius: f32, height: f32 },
  Cylinder { radius: f32, height: f32 },
  Cone { radius: f32, height: f32 },
  ConvexHull { vertices: Vec<Vec3> },
  TriangleMesh { vertices: Vec<Vec3>, indices: Vec<u32> },
  HeightField { size: Vec3, heights: Vec<f32> },
}

/// The internal representation of an effector.
struct Effector {
  _kind: EffectorKind,
}

#[allow(unused_variables)]
impl PhysicsBackend for InternalPhysicsBackend {
  fn step(&self, delta_time: f32) {
    let mut bodies = self.bodies.write().unwrap();
    let mut colliders = self.colliders.write().unwrap();

    for body in bodies.iter_mut() {
      body.position += body.velocity * delta_time;
    }

    for collider in colliders.iter_mut() {
      // TODO: collision checks
    }
  }

  fn reset(&self) {
    let mut bodies = self.bodies.write().unwrap();
    let mut colliders = self.colliders.write().unwrap();
    let mut effectors = self.effectors.write().unwrap();

    bodies.clear();
    colliders.clear();
    effectors.clear();
  }

  fn body_create(&self, kind: BodyKind, initial_position: Vec3) -> BodyId {
    let mut bodies = self.bodies.write().unwrap();

    bodies.insert(Body {
      kind,
      position: initial_position,
      rotation: Quat::IDENTITY,
      scale: Vec3::ONE,
      velocity: Vec3::ZERO,
      angular_velocity: Vec3::ZERO,
      colliders: Vec::new(),
    })
  }

  fn body_add_collider(&self, body: BodyId, collider: ColliderId) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.colliders.push(collider);
    }
  }

  fn body_remove_collider(&self, body: BodyId, collider: ColliderId) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.colliders.retain(|&it| it != collider);
    }
  }

  fn body_set_position(&self, body: BodyId, position: Vec3) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.position = position;
    }
  }

  fn body_get_position(&self, body: BodyId) -> Vec3 {
    let bodies = self.bodies.read().unwrap();

    if let Some(body) = bodies.get(body) {
      body.position
    } else {
      Vec3::ZERO
    }
  }

  fn body_set_rotation(&self, body: BodyId, rotation: Quat) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.rotation = rotation;
    }
  }

  fn body_get_rotation(&self, body: BodyId) -> Quat {
    let bodies = self.bodies.read().unwrap();

    if let Some(body) = bodies.get(body) {
      body.rotation
    } else {
      Quat::IDENTITY
    }
  }

  fn body_set_scale(&self, body: BodyId, scale: Vec3) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.scale = scale;
    }
  }

  fn body_get_scale(&self, body: BodyId) -> Vec3 {
    let bodies = self.bodies.read().unwrap();

    if let Some(body) = bodies.get(body) {
      body.scale
    } else {
      Vec3::ONE
    }
  }

  fn body_set_velocity(&self, body: BodyId, velocity: Vec3) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.velocity = velocity;
    }
  }

  fn body_get_velocity(&self, body: BodyId) -> Vec3 {
    let bodies = self.bodies.read().unwrap();

    if let Some(body) = bodies.get(body) {
      body.velocity
    } else {
      Vec3::ZERO
    }
  }

  fn body_set_angular_velocity(&self, body: BodyId, velocity: Vec3) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.angular_velocity = velocity;
    }
  }

  fn body_get_angular_velocity(&self, body: BodyId) -> Vec3 {
    let bodies = self.bodies.read().unwrap();

    if let Some(body) = bodies.get(body) {
      body.angular_velocity
    } else {
      Vec3::ZERO
    }
  }

  fn body_delete(&self, body: BodyId) {
    let mut bodies = self.bodies.write().unwrap();

    bodies.remove(body);
  }

  fn collider_create_sphere(&self, initial_position: Vec3, radius: f32) -> ColliderId {
    let mut colliders = self.colliders.write().unwrap();

    colliders.insert(Collider {
      kind: ColliderKind::Sphere,
      shape: ColliderShape::Sphere { radius },
      position: initial_position,
      rotation: Quat::IDENTITY,
      scale: Vec3::ZERO,
    })
  }

  fn collider_create_box(&self, initial_position: Vec3, size: Vec3) -> ColliderId {
    let mut colliders = self.colliders.write().unwrap();

    colliders.insert(Collider {
      kind: ColliderKind::Box,
      shape: ColliderShape::Box { size },
      position: initial_position,
      rotation: Quat::IDENTITY,
      scale: Vec3::ZERO,
    })
  }

  fn collider_create_capsule(&self, initial_position: Vec3, radius: f32, height: f32) -> ColliderId {
    let mut colliders = self.colliders.write().unwrap();

    colliders.insert(Collider {
      kind: ColliderKind::Capsule,
      shape: ColliderShape::Capsule { radius, height },
      position: initial_position,
      rotation: Quat::IDENTITY,
      scale: Vec3::ZERO,
    })
  }

  fn collider_create_cylinder(&self, initial_position: Vec3, radius: f32, height: f32) -> ColliderId {
    let mut colliders = self.colliders.write().unwrap();

    colliders.insert(Collider {
      kind: ColliderKind::Cylinder,
      shape: ColliderShape::Cylinder { radius, height },
      position: initial_position,
      rotation: Quat::IDENTITY,
      scale: Vec3::ZERO,
    })
  }

  fn collider_create_cone(&self, initial_position: Vec3, radius: f32, height: f32) -> ColliderId {
    let mut colliders = self.colliders.write().unwrap();

    colliders.insert(Collider {
      kind: ColliderKind::Cone,
      shape: ColliderShape::Cone { radius, height },
      position: initial_position,
      rotation: Quat::IDENTITY,
      scale: Vec3::ZERO,
    })
  }

  fn collider_create_convex_hull(&self, initial_position: Vec3, vertices: &[Vec3]) -> ColliderId {
    let mut colliders = self.colliders.write().unwrap();

    colliders.insert(Collider {
      kind: ColliderKind::ConvexHull,
      shape: ColliderShape::ConvexHull {
        vertices: vertices.to_vec(),
      },
      position: initial_position,
      rotation: Quat::IDENTITY,
      scale: Vec3::ZERO,
    })
  }

  fn collider_create_triangle_mesh(&self, initial_position: Vec3, vertices: &[Vec3], indices: &[u32]) -> ColliderId {
    let mut colliders = self.colliders.write().unwrap();

    colliders.insert(Collider {
      kind: ColliderKind::TriangleMesh,
      shape: ColliderShape::TriangleMesh {
        vertices: vertices.to_vec(),
        indices: indices.to_vec(),
      },
      position: initial_position,
      rotation: Quat::IDENTITY,
      scale: Vec3::ZERO,
    })
  }

  fn collider_create_height_field(&self, initial_position: Vec3, size: Vec3, heights: &[f32]) -> ColliderId {
    let mut colliders = self.colliders.write().unwrap();

    colliders.insert(Collider {
      kind: ColliderKind::HeightField,
      shape: ColliderShape::HeightField {
        size,
        heights: heights.to_vec(),
      },
      position: initial_position,
      rotation: Quat::IDENTITY,
      scale: Vec3::ZERO,
    })
  }

  fn collider_get_kind(&self, collider: ColliderId) -> ColliderKind {
    let colliders = self.colliders.read().unwrap();

    if let Some(collider) = colliders.get(collider) {
      collider.kind
    } else {
      ColliderKind::Sphere
    }
  }

  fn collider_set_position(&self, collider: ColliderId, position: Vec3) {
    let mut colliders = self.colliders.write().unwrap();

    if let Some(collider) = colliders.get_mut(collider) {
      collider.position = position;
    }
  }

  fn collider_get_position(&self, collider: ColliderId) -> Vec3 {
    let colliders = self.colliders.read().unwrap();

    if let Some(collider) = colliders.get(collider) {
      collider.position
    } else {
      Vec3::ZERO
    }
  }

  fn collider_set_rotation(&self, collider: ColliderId, rotation: Quat) {
    let mut colliders = self.colliders.write().unwrap();

    if let Some(collider) = colliders.get_mut(collider) {
      collider.rotation = rotation;
    }
  }

  fn collider_get_rotation(&self, collider: ColliderId) -> Quat {
    let colliders = self.colliders.read().unwrap();

    if let Some(collider) = colliders.get(collider) {
      collider.rotation
    } else {
      Quat::IDENTITY
    }
  }

  fn collider_set_scale(&self, collider: ColliderId, scale: Vec3) {
    let mut colliders = self.colliders.write().unwrap();

    if let Some(collider) = colliders.get_mut(collider) {
      collider.scale = scale;
    }
  }

  fn collider_get_scale(&self, collider: ColliderId) -> Vec3 {
    let colliders = self.colliders.read().unwrap();

    if let Some(collider) = colliders.get(collider) {
      collider.scale
    } else {
      Vec3::ZERO
    }
  }

  fn collider_delete(&self, collider: ColliderId) {
    let mut colliders = self.colliders.write().unwrap();

    colliders.remove(collider);
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
    let mut effectors = self.effectors.write().unwrap();

    effectors.remove(effector);
  }
}
