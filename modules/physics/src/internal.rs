use std::sync::RwLock;

use common::{vec3, Arena, QuadTree, Rectangle};

use super::*;

const EARTH_GRAVITY: Vec3 = vec3(0.0, -9.81, 0.0);

/// The default, home-baked [`PhysicsBackend`].
///
/// This is a simple physics backend that uses a simple Euler integration
/// algorithm to simulate the physics of the game world.
#[derive(Default)]
pub struct InternalPhysicsBackend {
  settings: RwLock<Settings>,
  bodies: RwLock<Arena<BodyId, Body>>,
  colliders: RwLock<Arena<ColliderId, Collider>>,
  effectors: RwLock<Arena<EffectorId, Effector>>,
  collision_broadphase: RwLock<QuadTree<ColliderId>>,
  overlapping_collider_pairs: RwLock<Vec<(ColliderId, ColliderId)>>,
}

struct Settings {
  gravity: Vec3,
}

impl Default for Settings {
  fn default() -> Self {
    Self { gravity: EARTH_GRAVITY }
  }
}

struct Body {
  kind: BodyKind,
  position: Vec3,
  rotation: Quat,
  scale: Vec3,
  velocity: Vec3,
  angular_velocity: Vec3,
  colliders: Vec<ColliderId>,
}

struct Collider {
  kind: ColliderKind,
  shape: ColliderShape,
  position: Vec3,
  rotation: Quat,
  scale: Vec3,
}

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

struct Effector {
  kind: EffectorKind,
  shape: EffectorShape,
  position: Vec3,
  rotation: Quat,
  scale: Vec3,
  strength: f32,
}

enum EffectorShape {
  Sphere { radius: f32 },
  Box { size: Vec3 },
  Capsule { radius: f32, height: f32 },
  Cylinder { radius: f32, height: f32 },
}

impl Collider {
  pub fn compute_bounding_rectangle(&self) -> Rectangle {
    todo!()
  }
}

impl InternalPhysicsBackend {
  fn integrate_physical_forces(&self, delta_time: f32) {
    let settings = self.settings.read().unwrap();
    let mut bodies = self.bodies.write().unwrap();

    for body in bodies.iter_mut() {
      body.position += body.velocity * delta_time;
      body.position += settings.gravity * delta_time;
    }
  }

  fn apply_collision_broadphase(&self) {
    let mut tree = self.collision_broadphase.write().unwrap();
    let colliders = self.colliders.read().unwrap();

    tree.clear();

    for (collider_id, collider) in colliders.enumerate() {
      let bounds = collider.compute_bounding_rectangle();

      tree.insert(collider_id, bounds);
    }
  }

  fn apply_collision_narrowphase(&self) {
    let tree = self.collision_broadphase.read().unwrap();
    let colliders = self.colliders.read().unwrap();
    let mut collider_pairs = self.overlapping_collider_pairs.write().unwrap();

    collider_pairs.clear();

    for (source_id, collider) in colliders.enumerate() {
      let bounds = collider.compute_bounding_rectangle();

      for other_id in tree.find_in_bounds(bounds) {
        if source_id == *other_id {
          continue;
        }

        if let Some(other_collider) = colliders.get(*other_id) {
          if collider.position.distance(other_collider.position) < 1.0 {
            collider_pairs.push((source_id, *other_id));
          }
        }
      }
    }
  }
}

#[allow(unused_variables)]
impl PhysicsBackend for InternalPhysicsBackend {
  fn step(&self, delta_time: f32) {
    self.integrate_physical_forces(delta_time);
    self.apply_collision_broadphase();
    self.apply_collision_narrowphase();
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

  fn collider_create_sphere(&self, kind: ColliderKind, initial_position: Vec3, radius: f32) -> ColliderId {
    let mut colliders = self.colliders.write().unwrap();

    colliders.insert(Collider {
      kind,
      shape: ColliderShape::Sphere { radius },
      position: initial_position,
      rotation: Quat::IDENTITY,
      scale: Vec3::ZERO,
    })
  }

  fn collider_create_box(&self, kind: ColliderKind, initial_position: Vec3, size: Vec3) -> ColliderId {
    let mut colliders = self.colliders.write().unwrap();

    colliders.insert(Collider {
      kind,
      shape: ColliderShape::Box { size },
      position: initial_position,
      rotation: Quat::IDENTITY,
      scale: Vec3::ZERO,
    })
  }

  fn collider_create_capsule(
    &self,
    kind: ColliderKind,
    initial_position: Vec3,
    radius: f32,
    height: f32,
  ) -> ColliderId {
    let mut colliders = self.colliders.write().unwrap();

    colliders.insert(Collider {
      kind,
      shape: ColliderShape::Capsule { radius, height },
      position: initial_position,
      rotation: Quat::IDENTITY,
      scale: Vec3::ZERO,
    })
  }

  fn collider_create_cylinder(
    &self,
    kind: ColliderKind,
    initial_position: Vec3,
    radius: f32,
    height: f32,
  ) -> ColliderId {
    let mut colliders = self.colliders.write().unwrap();

    colliders.insert(Collider {
      kind,
      shape: ColliderShape::Cylinder { radius, height },
      position: initial_position,
      rotation: Quat::IDENTITY,
      scale: Vec3::ZERO,
    })
  }

  fn collider_create_cone(&self, kind: ColliderKind, initial_position: Vec3, radius: f32, height: f32) -> ColliderId {
    let mut colliders = self.colliders.write().unwrap();

    colliders.insert(Collider {
      kind,
      shape: ColliderShape::Cone { radius, height },
      position: initial_position,
      rotation: Quat::IDENTITY,
      scale: Vec3::ZERO,
    })
  }

  fn collider_create_convex_hull(&self, kind: ColliderKind, initial_position: Vec3, vertices: &[Vec3]) -> ColliderId {
    let mut colliders = self.colliders.write().unwrap();

    colliders.insert(Collider {
      kind,
      shape: ColliderShape::ConvexHull {
        vertices: vertices.to_vec(),
      },
      position: initial_position,
      rotation: Quat::IDENTITY,
      scale: Vec3::ZERO,
    })
  }

  fn collider_create_triangle_mesh(
    &self,
    kind: ColliderKind,
    initial_position: Vec3,
    vertices: &[Vec3],
    indices: &[u32],
  ) -> ColliderId {
    let mut colliders = self.colliders.write().unwrap();

    colliders.insert(Collider {
      kind,
      shape: ColliderShape::TriangleMesh {
        vertices: vertices.to_vec(),
        indices: indices.to_vec(),
      },
      position: initial_position,
      rotation: Quat::IDENTITY,
      scale: Vec3::ZERO,
    })
  }

  fn collider_create_height_field(
    &self,
    kind: ColliderKind,
    initial_position: Vec3,
    size: Vec3,
    heights: &[f32],
  ) -> ColliderId {
    let mut colliders = self.colliders.write().unwrap();

    colliders.insert(Collider {
      kind,
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
      ColliderKind::Trigger
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

  fn effector_create_sphere(&self, kind: EffectorKind, initial_position: Vec3, radius: f32) -> EffectorId {
    let mut effectors = self.effectors.write().unwrap();

    effectors.insert(Effector {
      kind,
      shape: EffectorShape::Sphere { radius },
      position: initial_position,
      rotation: Quat::IDENTITY,
      scale: Vec3::ZERO,
      strength: 0.0,
    })
  }

  fn effector_create_box(&self, kind: EffectorKind, initial_position: Vec3, size: Vec3) -> EffectorId {
    let mut effectors = self.effectors.write().unwrap();

    effectors.insert(Effector {
      kind,
      shape: EffectorShape::Box { size },
      position: initial_position,
      rotation: Quat::IDENTITY,
      scale: Vec3::ZERO,
      strength: 0.0,
    })
  }

  fn effector_create_capsule(
    &self,
    kind: EffectorKind,
    initial_position: Vec3,
    radius: f32,
    height: f32,
  ) -> EffectorId {
    let mut effectors = self.effectors.write().unwrap();

    effectors.insert(Effector {
      kind,
      shape: EffectorShape::Capsule { radius, height },
      position: initial_position,
      rotation: Quat::IDENTITY,
      scale: Vec3::ZERO,
      strength: 0.0,
    })
  }

  fn effector_create_cylinder(
    &self,
    kind: EffectorKind,
    initial_position: Vec3,
    radius: f32,
    height: f32,
  ) -> EffectorId {
    let mut effectors = self.effectors.write().unwrap();

    effectors.insert(Effector {
      kind,
      shape: EffectorShape::Cylinder { radius, height },
      position: initial_position,
      rotation: Quat::IDENTITY,
      scale: Vec3::ZERO,
      strength: 0.0,
    })
  }

  fn effector_get_kind(&self, effector: EffectorId) -> EffectorKind {
    let effectors = self.effectors.read().unwrap();

    if let Some(effector) = effectors.get(effector) {
      effector.kind
    } else {
      EffectorKind::Gravity
    }
  }

  fn effector_set_position(&self, effector: EffectorId, position: Vec3) {
    let mut effectors = self.effectors.write().unwrap();

    if let Some(effector) = effectors.get_mut(effector) {
      effector.position = position;
    }
  }

  fn effector_get_position(&self, effector: EffectorId) -> Vec3 {
    let effectors = self.effectors.read().unwrap();

    if let Some(effector) = effectors.get(effector) {
      effector.position
    } else {
      Vec3::ZERO
    }
  }

  fn effector_set_rotation(&self, effector: EffectorId, rotation: Quat) {
    let mut effectors = self.effectors.write().unwrap();

    if let Some(effector) = effectors.get_mut(effector) {
      effector.rotation = rotation;
    }
  }

  fn effector_get_rotation(&self, effector: EffectorId) -> Quat {
    let effectors = self.effectors.read().unwrap();

    if let Some(effector) = effectors.get(effector) {
      effector.rotation
    } else {
      Quat::IDENTITY
    }
  }

  fn effector_set_scale(&self, effector: EffectorId, scale: Vec3) {
    let mut effectors = self.effectors.write().unwrap();

    if let Some(effector) = effectors.get_mut(effector) {
      effector.scale = scale;
    }
  }

  fn effector_get_scale(&self, effector: EffectorId) -> Vec3 {
    let effectors = self.effectors.read().unwrap();

    if let Some(effector) = effectors.get(effector) {
      effector.scale
    } else {
      Vec3::ZERO
    }
  }

  fn effector_set_strength(&self, effector: EffectorId, strength: f32) {
    let mut effectors = self.effectors.write().unwrap();

    if let Some(effector) = effectors.get_mut(effector) {
      effector.strength = strength;
    }
  }

  fn effector_get_strength(&self, effector: EffectorId) -> f32 {
    let effectors = self.effectors.read().unwrap();

    if let Some(effector) = effectors.get(effector) {
      effector.strength
    } else {
      0.0
    }
  }

  fn effector_delete(&self, effector: EffectorId) {
    let mut effectors = self.effectors.write().unwrap();

    effectors.remove(effector);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_behaviour() {
    let backend = InternalPhysicsBackend::default();

    let body = backend.body_create(BodyKind::Dynamic, Vec3::ZERO);
    let collider = backend.collider_create_sphere(ColliderKind::Solid, Vec3::ZERO, 1.0);

    backend.body_add_collider(body, collider);

    backend.step(1.0);
    backend.step(1.0);

    let position = backend.body_get_position(body);

    assert_eq!(position, vec3(0.0, -19.62, 0.0));
  }
}
