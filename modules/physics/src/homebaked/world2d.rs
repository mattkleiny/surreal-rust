use std::sync::RwLock;

use common::{Arena, FastHashSet, profiling, QuadTree, Rectangle, vec2};

use super::*;

/// A 2d physics world.
#[derive(Default)]
pub struct HomebakedWorld2D {
  settings: RwLock<Settings>,
  bodies: RwLock<Arena<BodyId, Body>>,
  colliders: RwLock<Arena<ColliderId, Collider>>,
  effectors: RwLock<Arena<EffectorId, Effector>>,
  joints: RwLock<Arena<JointId, Joint>>,
  materials: RwLock<Arena<MaterialId, Material>>,
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
  material: Option<MaterialId>,
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

/// A single collision event in the 2d world.
struct CollisionEvent {
  source_body: BodyId,
  source_collider: ColliderId,
  target_body: BodyId,
  target_collider: ColliderId,
  position: Vec2,
  normal: Vec2,
  penetration: f32,
  kind: ColliderKind,
}

/// An effector in the 2d physics world.
struct Effector {
  position: Vec2,
  rotation: f32,
  scale: Vec2,
  strength: f32,
  kind: EffectorKind,
  shape: EffectorShape,
  colliders: FastHashSet<ColliderId>,
}

/// The shape of a 2d effector.
enum EffectorShape {
  Sphere { radius: f32 },
  Box { size: Vec2 },
  Capsule { radius: f32, height: f32 },
  Cylinder { radius: f32, height: f32 },
}

/// A joint in the 2d physics world.
struct Joint {
  body_a: Option<BodyId>,
  body_b: Option<BodyId>,
  anchor: Vec2,
}

/// A physics material for a 2d body.
struct Material {
  friction: f32,
  restitution: f32,
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      gravity: vec2(0., -9.8),
    }
  }
}

impl Collider {
  /// Computes the bounding rectangle for a collider.
  ///
  /// This is used for broadphase collision detection; the
  /// rectangle is the best fit for the collider shape.
  fn compute_bounding_rectangle(&self) -> Rectangle {
    todo!()
  }
}

impl PhysicsWorld for HomebakedWorld2D {
  fn step(&self, delta_time: f32) {
    let settings = self.settings.read().unwrap();

    self.integrate_bodies(delta_time, &settings);

    let collisions = self.broadphase_collision_detection();
    let effectors = self.broadphase_effector_detection();

    for event in self.detect_collisions(collisions) {
      self.integrate_collisions(event);
    }

    self.integrate_effectors(delta_time, &settings, effectors);
  }

  fn reset(&self) {
    self.bodies.write().unwrap().clear();
    self.colliders.write().unwrap().clear();
    self.effectors.write().unwrap().clear();
  }
}

impl HomebakedWorld2D {
  #[profiling]
  fn integrate_bodies(&self, delta_time: f32, settings: &Settings) {
    let mut bodies = self.bodies.write().unwrap();

    for body in bodies.iter_mut() {
      body.velocity += settings.gravity * delta_time;
      body.position += body.velocity;
    }
  }

  #[profiling]
  fn detect_collisions(&self, _body_tree: QuadTree<BodyId>) -> Vec<CollisionEvent> {
    todo!()
  }

  #[profiling]
  fn integrate_collisions(&self, _collision_event: CollisionEvent) {
    todo!()
  }

  #[profiling]
  fn integrate_effectors(&self, delta_time: f32, _settings: &Settings, effector_tree: QuadTree<EffectorId>) {
    let mut bodies = self.bodies.write().unwrap();
    let effectors = self.effectors.write().unwrap();

    for body in bodies.iter_mut() {
      for (effector_id, effector) in effectors.enumerate() {
        if effector_tree.contains_in_bounds(effector_id, body.position) {
          let direction = effector.position - body.position;
          let distance = direction.length();
          let strength = effector.strength / distance.powi(2);

          body.velocity += direction.normalize() * strength * delta_time;
        }
      }
    }
  }

  #[profiling]
  fn broadphase_collision_detection(&self) -> QuadTree<BodyId> {
    let mut results = QuadTree::default();

    let bodies = self.bodies.read().unwrap();
    let colliders = self.colliders.read().unwrap();

    for (body_id, body) in bodies.enumerate() {
      let mut bounding_rectangle = Rectangle::default();

      for collider in body.colliders.iter().flat_map(|id| colliders.get(*id)) {
        bounding_rectangle.extend(&collider.compute_bounding_rectangle());
      }

      results.insert(body_id, bounding_rectangle)
    }

    results
  }

  #[profiling]
  fn broadphase_effector_detection(&self) -> QuadTree<EffectorId> {
    let mut results = QuadTree::default();

    let effectors = self.effectors.read().unwrap();
    let colliders = self.colliders.read().unwrap();

    for (effector_id, effector) in effectors.enumerate() {
      let mut bounding_rectangle = Rectangle::default();

      for collider in effector.colliders.iter().flat_map(|id| colliders.get(*id)) {
        bounding_rectangle.extend(&collider.compute_bounding_rectangle());
      }

      results.insert(effector_id, bounding_rectangle)
    }

    results
  }
}

#[allow(unused_variables)]
impl PhysicsWorld2D for HomebakedWorld2D {
  #[profiling]
  fn set_gravity(&self, gravity: Vec2) {
    let mut settings = self.settings.write().unwrap();

    settings.gravity = gravity;
  }

  #[profiling]
  fn get_gravity(&self) -> Vec2 {
    let settings = self.settings.read().unwrap();

    settings.gravity
  }

  #[profiling]
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
      material: None,
    })
  }

  #[profiling]
  fn body_add_collider(&self, body: BodyId, collider: ColliderId) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.colliders.insert(collider);
    }
  }

  #[profiling]
  fn body_remove_collider(&self, body: BodyId, collider: ColliderId) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.colliders.remove(&collider);
    }
  }

  #[profiling]
  fn body_set_position(&self, body: BodyId, position: Vec2) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.position = position;
    }
  }

  #[profiling]
  fn body_get_position(&self, body: BodyId) -> Vec2 {
    let bodies = self.bodies.read().unwrap();

    bodies.get(body).map_or(Vec2::ZERO, |it| it.position)
  }

  #[profiling]
  fn body_set_rotation(&self, body: BodyId, rotation: f32) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.rotation = rotation;
    }
  }

  #[profiling]
  fn body_get_rotation(&self, body: BodyId) -> f32 {
    let bodies = self.bodies.read().unwrap();

    bodies.get(body).map_or(0., |it| it.rotation)
  }

  #[profiling]
  fn body_set_scale(&self, body: BodyId, scale: Vec2) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.scale = scale;
    }
  }

  #[profiling]
  fn body_get_scale(&self, body: BodyId) -> Vec2 {
    let bodies = self.bodies.read().unwrap();

    bodies.get(body).map_or(Vec2::ONE, |it| it.scale)
  }

  #[profiling]
  fn body_set_velocity(&self, body: BodyId, velocity: Vec2) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.velocity = velocity;
    }
  }

  #[profiling]
  fn body_get_velocity(&self, body: BodyId) -> Vec2 {
    let bodies = self.bodies.read().unwrap();

    bodies.get(body).map_or(Vec2::ZERO, |it| it.velocity)
  }

  #[profiling]
  fn body_set_angular_velocity(&self, body: BodyId, velocity: Vec2) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.angular_velocity = velocity;
    }
  }

  #[profiling]
  fn body_get_angular_velocity(&self, body: BodyId) -> Vec2 {
    let bodies = self.bodies.read().unwrap();

    bodies.get(body).map_or(Vec2::ZERO, |it| it.angular_velocity)
  }

  #[profiling]
  fn body_set_material(&self, body: BodyId, material: Option<MaterialId>) {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.material = material;
    }
  }

  #[profiling]
  fn body_get_material(&self, body: BodyId) -> Option<MaterialId> {
    let bodies = self.bodies.read().unwrap();

    bodies.get(body).and_then(|it| it.material)
  }

  #[profiling]
  fn body_delete(&self, body: BodyId) {
    let mut bodies = self.bodies.write().unwrap();

    bodies.remove(body);
  }

  #[profiling]
  fn collider_create_circle(&self, kind: ColliderKind, initial_position: Vec2, radius: f32) -> ColliderId {
    let mut colliders = self.colliders.write().unwrap();

    colliders.insert(Collider {
      position: initial_position,
      rotation: 0.0,
      scale: Vec2::ONE,
      shape: ColliderShape::Sphere { radius },
      kind,
      bodies: FastHashSet::default(),
    })
  }

  #[profiling]
  fn collider_create_rectangle(&self, kind: ColliderKind, initial_position: Vec2, size: Vec2) -> ColliderId {
    let mut colliders = self.colliders.write().unwrap();

    colliders.insert(Collider {
      position: initial_position,
      rotation: 0.0,
      scale: Vec2::ONE,
      shape: ColliderShape::Box { size },
      kind,
      bodies: FastHashSet::default(),
    })
  }

  #[profiling]
  fn collider_create_triangle_mesh(
    &self,
    kind: ColliderKind,
    initial_position: Vec2,
    vertices: &[Vec2],
    indices: &[u32],
  ) -> ColliderId {
    let mut colliders = self.colliders.write().unwrap();

    colliders.insert(Collider {
      position: initial_position,
      rotation: 0.0,
      scale: Vec2::ONE,
      shape: ColliderShape::TriangleMesh {
        vertices: vertices.to_vec(),
        indices: indices.to_vec(),
      },
      kind,
      bodies: FastHashSet::default(),
    })
  }

  #[profiling]
  fn collider_create_height_field(
    &self,
    kind: ColliderKind,
    initial_position: Vec2,
    size: Vec2,
    heights: &[f32],
  ) -> ColliderId {
    let mut colliders = self.colliders.write().unwrap();

    colliders.insert(Collider {
      position: initial_position,
      rotation: 0.0,
      scale: Vec2::ONE,
      shape: ColliderShape::HeightField {
        size,
        heights: heights.to_vec(),
      },
      kind,
      bodies: FastHashSet::default(),
    })
  }

  #[profiling]
  fn collider_get_kind(&self, collider: ColliderId) -> Option<ColliderKind> {
    let colliders = self.colliders.read().unwrap();

    colliders.get(collider).map(|collider| collider.kind)
  }

  #[profiling]
  fn collider_set_position(&self, collider: ColliderId, position: Vec2) {
    let mut colliders = self.colliders.write().unwrap();

    if let Some(collider) = colliders.get_mut(collider) {
      collider.position = position;
    }
  }

  #[profiling]
  fn collider_get_position(&self, collider: ColliderId) -> Vec2 {
    let colliders = self.colliders.read().unwrap();

    colliders.get(collider).map_or(Vec2::ZERO, |it| it.position)
  }

  #[profiling]
  fn collider_set_rotation(&self, collider: ColliderId, rotation: f32) {
    let mut colliders = self.colliders.write().unwrap();

    if let Some(collider) = colliders.get_mut(collider) {
      collider.rotation = rotation;
    }
  }

  #[profiling]
  fn collider_get_rotation(&self, collider: ColliderId) -> f32 {
    let colliders = self.colliders.read().unwrap();

    colliders.get(collider).map_or(0., |it| it.rotation)
  }

  #[profiling]
  fn collider_set_scale(&self, collider: ColliderId, scale: Vec2) {
    let mut colliders = self.colliders.write().unwrap();

    if let Some(collider) = colliders.get_mut(collider) {
      collider.scale = scale;
    }
  }

  #[profiling]
  fn collider_get_scale(&self, collider: ColliderId) -> Vec2 {
    let colliders = self.colliders.read().unwrap();

    colliders.get(collider).map_or(Vec2::ONE, |it| it.scale)
  }

  #[profiling]
  fn collider_delete(&self, collider: ColliderId) {
    let mut colliders = self.colliders.write().unwrap();
    let mut bodies = self.bodies.write().unwrap();

    for body in bodies.iter_mut() {
      body.colliders.remove(&collider);
    }

    colliders.remove(collider);
  }

  #[profiling]
  fn effector_create_sphere(&self, kind: EffectorKind, initial_position: Vec2, radius: f32) -> EffectorId {
    let mut effectors = self.effectors.write().unwrap();

    effectors.insert(Effector {
      position: initial_position,
      rotation: 0.0,
      scale: Vec2::ONE,
      strength: 1.0,
      kind,
      shape: EffectorShape::Sphere { radius },
      colliders: FastHashSet::default(),
    })
  }

  #[profiling]
  fn effector_create_box(&self, kind: EffectorKind, initial_position: Vec2, size: Vec2) -> EffectorId {
    let mut effectors = self.effectors.write().unwrap();

    effectors.insert(Effector {
      position: initial_position,
      rotation: 0.0,
      scale: Vec2::ONE,
      strength: 1.0,
      kind,
      shape: EffectorShape::Box { size },
      colliders: FastHashSet::default(),
    })
  }

  #[profiling]
  fn effector_create_capsule(
    &self,
    kind: EffectorKind,
    initial_position: Vec2,
    radius: f32,
    height: f32,
  ) -> EffectorId {
    let mut effectors = self.effectors.write().unwrap();

    effectors.insert(Effector {
      position: initial_position,
      rotation: 0.0,
      scale: Vec2::ONE,
      strength: 1.0,
      kind,
      shape: EffectorShape::Capsule { radius, height },
      colliders: FastHashSet::default(),
    })
  }

  #[profiling]
  fn effector_create_cylinder(
    &self,
    kind: EffectorKind,
    initial_position: Vec2,
    radius: f32,
    height: f32,
  ) -> EffectorId {
    let mut effectors = self.effectors.write().unwrap();

    effectors.insert(Effector {
      position: initial_position,
      rotation: 0.0,
      scale: Vec2::ONE,
      strength: 1.0,
      kind,
      shape: EffectorShape::Cylinder { radius, height },
      colliders: FastHashSet::default(),
    })
  }

  #[profiling]
  fn effector_get_kind(&self, effector: EffectorId) -> EffectorKind {
    let effectors = self.effectors.read().unwrap();

    effectors.get(effector).map_or(EffectorKind::Gravity, |it| it.kind)
  }

  #[profiling]
  fn effector_set_position(&self, effector: EffectorId, position: Vec2) {
    let mut effectors = self.effectors.write().unwrap();

    if let Some(effector) = effectors.get_mut(effector) {
      effector.position = position;
    }
  }

  #[profiling]
  fn effector_get_position(&self, effector: EffectorId) -> Vec2 {
    let effectors = self.effectors.read().unwrap();

    effectors.get(effector).map_or(Vec2::ZERO, |it| it.position)
  }

  #[profiling]
  fn effector_set_rotation(&self, effector: EffectorId, rotation: f32) {
    let mut effectors = self.effectors.write().unwrap();

    if let Some(effector) = effectors.get_mut(effector) {
      effector.rotation = rotation;
    }
  }

  #[profiling]
  fn effector_get_rotation(&self, effector: EffectorId) -> f32 {
    let effectors = self.effectors.read().unwrap();

    effectors.get(effector).map_or(0., |it| it.rotation)
  }

  #[profiling]
  fn effector_set_scale(&self, effector: EffectorId, scale: Vec2) {
    let mut effectors = self.effectors.write().unwrap();

    if let Some(effector) = effectors.get_mut(effector) {
      effector.scale = scale;
    }
  }

  #[profiling]
  fn effector_get_scale(&self, effector: EffectorId) -> Vec2 {
    let effectors = self.effectors.read().unwrap();

    effectors.get(effector).map_or(Vec2::ONE, |it| it.scale)
  }

  #[profiling]
  fn effector_set_strength(&self, effector: EffectorId, strength: f32) {
    let mut effectors = self.effectors.write().unwrap();

    if let Some(effector) = effectors.get_mut(effector) {
      effector.strength = strength;
    }
  }

  #[profiling]
  fn effector_get_strength(&self, effector: EffectorId) -> f32 {
    let effectors = self.effectors.read().unwrap();

    effectors.get(effector).map_or(0., |it| it.strength)
  }

  #[profiling]
  fn effector_delete(&self, effector: EffectorId) {
    let mut effectors = self.effectors.write().unwrap();

    effectors.remove(effector);
  }

  #[profiling]
  fn joint_create(&self) -> JointId {
    let mut joints = self.joints.write().unwrap();
    let settings = self.settings.read().unwrap();

    joints.insert(Joint {
      body_a: None,
      body_b: None,
      anchor: Vec2::ZERO,
    })
  }

  #[profiling]
  fn joint_attach(&self, joint: JointId, body_a: BodyId, body_b: BodyId) {
    let mut joints = self.joints.write().unwrap();

    if let Some(joint) = joints.get_mut(joint) {
      joint.body_a = Some(body_a);
      joint.body_b = Some(body_b);
    }
  }

  #[profiling]
  fn joint_detach(&self, joint: JointId) {
    let mut joints = self.joints.write().unwrap();

    if let Some(joint) = joints.get_mut(joint) {
      joint.body_a = None;
      joint.body_b = None;
    }
  }

  #[profiling]
  fn joint_get_bodies(&self, joint: JointId) -> (BodyId, BodyId) {
    let joints = self.joints.read().unwrap();

    joints.get(joint).map_or((BodyId::default(), BodyId::default()), |it| {
      (it.body_a.unwrap_or_default(), it.body_b.unwrap_or_default())
    })
  }

  #[profiling]
  fn joint_set_anchor(&self, joint: JointId, anchor: Vec2) {
    let mut joints = self.joints.write().unwrap();

    if let Some(joint) = joints.get_mut(joint) {
      joint.anchor = anchor;
    }
  }

  #[profiling]
  fn joint_get_anchor(&self, joint: JointId) -> Vec2 {
    let joints = self.joints.read().unwrap();

    joints.get(joint).map_or(Vec2::ZERO, |it| it.anchor)
  }

  #[profiling]
  fn joint_delete(&self, joint: JointId) {
    let mut joints = self.joints.write().unwrap();

    joints.remove(joint);
  }

  #[profiling]
  fn material_create(&self) -> MaterialId {
    let mut materials = self.materials.write().unwrap();

    materials.insert(Material {
      friction: 0.5,
      restitution: 0.5,
    })
  }

  #[profiling]
  fn material_set_friction(&self, material: MaterialId, friction: f32) {
    let mut materials = self.materials.write().unwrap();

    if let Some(material) = materials.get_mut(material) {
      material.friction = friction;
    }
  }

  #[profiling]
  fn material_get_friction(&self, material: MaterialId) -> f32 {
    let materials = self.materials.read().unwrap();

    materials.get(material).map_or(0.5, |it| it.friction)
  }

  #[profiling]
  fn material_set_restitution(&self, material: MaterialId, restitution: f32) {
    let mut materials = self.materials.write().unwrap();

    if let Some(material) = materials.get_mut(material) {
      material.restitution = restitution;
    }
  }

  #[profiling]
  fn material_get_restitution(&self, material: MaterialId) -> f32 {
    let materials = self.materials.read().unwrap();

    materials.get(material).map_or(0.5, |it| it.restitution)
  }

  #[profiling]
  fn material_delete(&self, material: MaterialId) {
    let mut materials = self.materials.write().unwrap();

    materials.remove(material);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_read_and_write_position() {
    let world = HomebakedWorld2D::default();
    let body_id = world.body_create(BodyKind::Kinematic, Vec2::ZERO);

    world.body_set_position(body_id, vec2(1., 2.));

    assert_eq!(world.body_get_position(body_id), vec2(1., 2.));

    world.body_delete(body_id);

    assert_eq!(world.body_get_position(body_id), Vec2::ZERO);
  }

  #[test]
  fn it_should_read_and_write_velocity() {
    let world = HomebakedWorld2D::default();
    let body_id = world.body_create(BodyKind::Kinematic, Vec2::ZERO);

    world.body_set_velocity(body_id, vec2(1., 2.));

    assert_eq!(world.body_get_velocity(body_id), vec2(1., 2.));

    world.body_delete(body_id);

    assert_eq!(world.body_get_velocity(body_id), Vec2::ZERO);
  }

  #[test]
  fn it_should_create_a_simple_joint() {
    let world = HomebakedWorld2D::default();
    let body1 = world.body_create(BodyKind::Kinematic, Vec2::ZERO);
    let body2 = world.body_create(BodyKind::Kinematic, Vec2::ZERO);

    let joint = world.joint_create();

    world.joint_attach(joint, body1, body2);

    assert_eq!(world.joint_get_bodies(joint), (body1, body2));
  }
}
