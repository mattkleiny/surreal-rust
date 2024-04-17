use std::sync::RwLock;

use common::{profiling, vec2, Arena, FastHashSet, QuadTree, Rectangle};

use super::*;

/// A 2d physics world.
#[derive(Default)]
pub struct InternalPhysicsWorld2D {
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
  Circle { radius: f32 },
  Rectangle { size: Vec2 },
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

impl PhysicsWorld for InternalPhysicsWorld2D {
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

impl InternalPhysicsWorld2D {
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
impl PhysicsWorld2D for InternalPhysicsWorld2D {
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
  fn body_create(&self, kind: BodyKind, initial_position: Vec2) -> Result<BodyId, BodyError> {
    let mut bodies = self.bodies.write().unwrap();

    let body_id = bodies.insert(Body {
      position: initial_position,
      rotation: 0.0,
      scale: Vec2::ONE,
      velocity: Vec2::ZERO,
      angular_velocity: Vec2::ZERO,
      kind,
      colliders: FastHashSet::default(),
      material: None,
    });

    Ok(body_id)
  }

  #[profiling]
  fn body_add_collider(&self, body: BodyId, collider: ColliderId) -> Result<(), BodyError> {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.colliders.insert(collider);
      Ok(())
    } else {
      Err(BodyError::InvalidId(body))
    }
  }

  #[profiling]
  fn body_remove_collider(&self, body: BodyId, collider: ColliderId) -> Result<(), BodyError> {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.colliders.remove(&collider);
      Ok(())
    } else {
      Err(BodyError::InvalidId(body))
    }
  }

  #[profiling]
  fn body_set_kind(&self, body: BodyId, kind: BodyKind) -> Result<(), BodyError> {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.kind = kind;
      Ok(())
    } else {
      Err(BodyError::InvalidId(body))
    }
  }

  #[profiling]
  fn body_get_kind(&self, body: BodyId) -> Option<BodyKind> {
    let bodies = self.bodies.read().unwrap();

    bodies.get(body).map(|it| it.kind)
  }

  #[profiling]
  fn body_set_position(&self, body: BodyId, position: Vec2) -> Result<(), BodyError> {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.position = position;
      Ok(())
    } else {
      Err(BodyError::InvalidId(body))
    }
  }

  #[profiling]
  fn body_get_position(&self, body: BodyId) -> Option<Vec2> {
    let bodies = self.bodies.read().unwrap();

    bodies.get(body).map(|it| it.position)
  }

  #[profiling]
  fn body_set_rotation(&self, body: BodyId, rotation: f32) -> Result<(), BodyError> {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.rotation = rotation;
      Ok(())
    } else {
      Err(BodyError::InvalidId(body))
    }
  }

  #[profiling]
  fn body_get_rotation(&self, body: BodyId) -> Option<f32> {
    let bodies = self.bodies.read().unwrap();

    bodies.get(body).map(|it| it.rotation)
  }

  #[profiling]
  fn body_set_scale(&self, body: BodyId, scale: Vec2) -> Result<(), BodyError> {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.scale = scale;
      Ok(())
    } else {
      Err(BodyError::InvalidId(body))
    }
  }

  #[profiling]
  fn body_get_scale(&self, body: BodyId) -> Option<Vec2> {
    let bodies = self.bodies.read().unwrap();

    bodies.get(body).map(|it| it.scale)
  }

  #[profiling]
  fn body_set_velocity(&self, body: BodyId, velocity: Vec2) -> Result<(), BodyError> {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.velocity = velocity;
      Ok(())
    } else {
      Err(BodyError::InvalidId(body))
    }
  }

  #[profiling]
  fn body_get_velocity(&self, body: BodyId) -> Option<Vec2> {
    let bodies = self.bodies.read().unwrap();

    bodies.get(body).map(|it| it.velocity)
  }

  #[profiling]
  fn body_set_angular_velocity(&self, body: BodyId, velocity: Vec2) -> Result<(), BodyError> {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.angular_velocity = velocity;
      Ok(())
    } else {
      Err(BodyError::InvalidId(body))
    }
  }

  #[profiling]
  fn body_get_angular_velocity(&self, body: BodyId) -> Option<Vec2> {
    let bodies = self.bodies.read().unwrap();

    bodies.get(body).map(|it| it.angular_velocity)
  }

  #[profiling]
  fn body_set_material(&self, body: BodyId, material: Option<MaterialId>) -> Result<(), BodyError> {
    let mut bodies = self.bodies.write().unwrap();

    if let Some(body) = bodies.get_mut(body) {
      body.material = material;
      Ok(())
    } else {
      Err(BodyError::InvalidId(body))
    }
  }

  #[profiling]
  fn body_get_material(&self, body: BodyId) -> Option<MaterialId> {
    let bodies = self.bodies.read().unwrap();

    bodies.get(body).and_then(|it| it.material)
  }

  #[profiling]
  fn body_delete(&self, body: BodyId) -> Result<(), BodyError> {
    let mut bodies = self.bodies.write().unwrap();

    if bodies.remove(body).is_some() {
      Ok(())
    } else {
      Err(BodyError::InvalidId(body))
    }
  }

  #[profiling]
  fn collider_create_circle(
    &self,
    kind: ColliderKind,
    initial_position: Vec2,
    radius: f32,
  ) -> Result<ColliderId, ColliderError> {
    let mut colliders = self.colliders.write().unwrap();

    let collider_id = colliders.insert(Collider {
      position: initial_position,
      rotation: 0.0,
      scale: Vec2::ONE,
      shape: ColliderShape::Circle { radius },
      kind,
      bodies: FastHashSet::default(),
    });

    Ok(collider_id)
  }

  #[profiling]
  fn collider_create_rectangle(
    &self,
    kind: ColliderKind,
    initial_position: Vec2,
    size: Vec2,
  ) -> Result<ColliderId, ColliderError> {
    let mut colliders = self.colliders.write().unwrap();

    let collider_id = colliders.insert(Collider {
      position: initial_position,
      rotation: 0.0,
      scale: Vec2::ONE,
      shape: ColliderShape::Rectangle { size },
      kind,
      bodies: FastHashSet::default(),
    });

    Ok(collider_id)
  }

  #[profiling]
  fn collider_create_triangle_mesh(
    &self,
    kind: ColliderKind,
    initial_position: Vec2,
    vertices: &[Vec2],
    indices: &[u32],
  ) -> Result<ColliderId, ColliderError> {
    let mut colliders = self.colliders.write().unwrap();

    let collider_id = colliders.insert(Collider {
      position: initial_position,
      rotation: 0.0,
      scale: Vec2::ONE,
      shape: ColliderShape::TriangleMesh {
        vertices: vertices.to_vec(),
        indices: indices.to_vec(),
      },
      kind,
      bodies: FastHashSet::default(),
    });

    Ok(collider_id)
  }

  #[profiling]
  fn collider_create_height_field(
    &self,
    kind: ColliderKind,
    initial_position: Vec2,
    size: Vec2,
    heights: &[f32],
  ) -> Result<ColliderId, ColliderError> {
    let mut colliders = self.colliders.write().unwrap();

    let collider_id = colliders.insert(Collider {
      position: initial_position,
      rotation: 0.0,
      scale: Vec2::ONE,
      shape: ColliderShape::HeightField {
        size,
        heights: heights.to_vec(),
      },
      kind,
      bodies: FastHashSet::default(),
    });

    Ok(collider_id)
  }

  #[profiling]
  fn collider_get_kind(&self, collider: ColliderId) -> Option<ColliderKind> {
    let colliders = self.colliders.read().unwrap();

    colliders.get(collider).map(|collider| collider.kind)
  }

  #[profiling]
  fn collider_set_position(&self, collider: ColliderId, position: Vec2) -> Result<(), ColliderError> {
    let mut colliders = self.colliders.write().unwrap();

    if let Some(collider) = colliders.get_mut(collider) {
      collider.position = position;
      Ok(())
    } else {
      return Err(ColliderError::InvalidId(collider));
    }
  }

  #[profiling]
  fn collider_get_position(&self, collider: ColliderId) -> Option<Vec2> {
    let colliders = self.colliders.read().unwrap();

    colliders.get(collider).map(|it| it.position)
  }

  #[profiling]
  fn collider_set_rotation(&self, collider: ColliderId, rotation: f32) -> Result<(), ColliderError> {
    let mut colliders = self.colliders.write().unwrap();

    if let Some(collider) = colliders.get_mut(collider) {
      collider.rotation = rotation;
      Ok(())
    } else {
      Err(ColliderError::InvalidId(collider))
    }
  }

  #[profiling]
  fn collider_get_rotation(&self, collider: ColliderId) -> Option<f32> {
    let colliders = self.colliders.read().unwrap();

    colliders.get(collider).map(|it| it.rotation)
  }

  #[profiling]
  fn collider_set_scale(&self, collider: ColliderId, scale: Vec2) -> Result<(), ColliderError> {
    let mut colliders = self.colliders.write().unwrap();

    if let Some(collider) = colliders.get_mut(collider) {
      collider.scale = scale;
      Ok(())
    } else {
      Err(ColliderError::InvalidId(collider))
    }
  }

  #[profiling]
  fn collider_get_scale(&self, collider: ColliderId) -> Option<Vec2> {
    let colliders = self.colliders.read().unwrap();

    colliders.get(collider).map(|it| it.scale)
  }

  #[profiling]
  fn collider_delete(&self, collider: ColliderId) -> Result<(), ColliderError> {
    let mut colliders = self.colliders.write().unwrap();
    let mut bodies = self.bodies.write().unwrap();

    for body in bodies.iter_mut() {
      body.colliders.remove(&collider);
    }

    if colliders.remove(collider).is_some() {
      Ok(())
    } else {
      Err(ColliderError::InvalidId(collider))
    }
  }

  #[profiling]
  fn effector_create_sphere(
    &self,
    kind: EffectorKind,
    initial_position: Vec2,
    radius: f32,
  ) -> Result<EffectorId, EffectorError> {
    let mut effectors = self.effectors.write().unwrap();

    let effector_id = effectors.insert(Effector {
      position: initial_position,
      rotation: 0.0,
      scale: Vec2::ONE,
      strength: 1.0,
      kind,
      shape: EffectorShape::Sphere { radius },
      colliders: FastHashSet::default(),
    });

    Ok(effector_id)
  }

  #[profiling]
  fn effector_create_box(
    &self,
    kind: EffectorKind,
    initial_position: Vec2,
    size: Vec2,
  ) -> Result<EffectorId, EffectorError> {
    let mut effectors = self.effectors.write().unwrap();

    let effector_id = effectors.insert(Effector {
      position: initial_position,
      rotation: 0.0,
      scale: Vec2::ONE,
      strength: 1.0,
      kind,
      shape: EffectorShape::Box { size },
      colliders: FastHashSet::default(),
    });

    Ok(effector_id)
  }

  #[profiling]
  fn effector_create_capsule(
    &self,
    kind: EffectorKind,
    initial_position: Vec2,
    radius: f32,
    height: f32,
  ) -> Result<EffectorId, EffectorError> {
    let mut effectors = self.effectors.write().unwrap();

    let effector_id = effectors.insert(Effector {
      position: initial_position,
      rotation: 0.0,
      scale: Vec2::ONE,
      strength: 1.0,
      kind,
      shape: EffectorShape::Capsule { radius, height },
      colliders: FastHashSet::default(),
    });

    Ok(effector_id)
  }

  #[profiling]
  fn effector_create_cylinder(
    &self,
    kind: EffectorKind,
    initial_position: Vec2,
    radius: f32,
    height: f32,
  ) -> Result<EffectorId, EffectorError> {
    let mut effectors = self.effectors.write().unwrap();

    let effector_id = effectors.insert(Effector {
      position: initial_position,
      rotation: 0.0,
      scale: Vec2::ONE,
      strength: 1.0,
      kind,
      shape: EffectorShape::Cylinder { radius, height },
      colliders: FastHashSet::default(),
    });

    Ok(effector_id)
  }

  #[profiling]
  fn effector_get_kind(&self, effector: EffectorId) -> Option<EffectorKind> {
    let effectors = self.effectors.read().unwrap();

    effectors.get(effector).map(|it| it.kind)
  }

  #[profiling]
  fn effector_set_position(&self, effector: EffectorId, position: Vec2) -> Result<(), EffectorError> {
    let mut effectors = self.effectors.write().unwrap();

    if let Some(effector) = effectors.get_mut(effector) {
      effector.position = position;
      Ok(())
    } else {
      Err(EffectorError::InvalidId(effector))
    }
  }

  #[profiling]
  fn effector_get_position(&self, effector: EffectorId) -> Option<Vec2> {
    let effectors = self.effectors.read().unwrap();

    effectors.get(effector).map(|it| it.position)
  }

  #[profiling]
  fn effector_set_rotation(&self, effector: EffectorId, rotation: f32) -> Result<(), EffectorError> {
    let mut effectors = self.effectors.write().unwrap();

    if let Some(effector) = effectors.get_mut(effector) {
      effector.rotation = rotation;
      Ok(())
    } else {
      Err(EffectorError::InvalidId(effector))
    }
  }

  #[profiling]
  fn effector_get_rotation(&self, effector: EffectorId) -> Option<f32> {
    let effectors = self.effectors.read().unwrap();

    effectors.get(effector).map(|it| it.rotation)
  }

  #[profiling]
  fn effector_set_scale(&self, effector: EffectorId, scale: Vec2) -> Result<(), EffectorError> {
    let mut effectors = self.effectors.write().unwrap();

    if let Some(effector) = effectors.get_mut(effector) {
      effector.scale = scale;
      Ok(())
    } else {
      Err(EffectorError::InvalidId(effector))
    }
  }

  #[profiling]
  fn effector_get_scale(&self, effector: EffectorId) -> Option<Vec2> {
    let effectors = self.effectors.read().unwrap();

    effectors.get(effector).map(|it| it.scale)
  }

  #[profiling]
  fn effector_set_strength(&self, effector: EffectorId, strength: f32) -> Result<(), EffectorError> {
    let mut effectors = self.effectors.write().unwrap();

    if let Some(effector) = effectors.get_mut(effector) {
      effector.strength = strength;
      Ok(())
    } else {
      Err(EffectorError::InvalidId(effector))
    }
  }

  #[profiling]
  fn effector_get_strength(&self, effector: EffectorId) -> Option<f32> {
    let effectors = self.effectors.read().unwrap();

    effectors.get(effector).map(|it| it.strength)
  }

  #[profiling]
  fn effector_delete(&self, effector: EffectorId) -> Result<(), EffectorError> {
    let mut effectors = self.effectors.write().unwrap();

    if effectors.remove(effector).is_some() {
      Ok(())
    } else {
      Err(EffectorError::InvalidId(effector))
    }
  }

  #[profiling]
  fn joint_create(&self) -> Result<JointId, JointError> {
    let mut joints = self.joints.write().unwrap();
    let settings = self.settings.read().unwrap();

    let joint_id = joints.insert(Joint {
      body_a: None,
      body_b: None,
      anchor: Vec2::ZERO,
    });

    Ok(joint_id)
  }

  #[profiling]
  fn joint_attach(&self, joint: JointId, body_a: BodyId, body_b: BodyId) -> Result<(), JointError> {
    let mut joints = self.joints.write().unwrap();

    if let Some(joint) = joints.get_mut(joint) {
      joint.body_a = Some(body_a);
      joint.body_b = Some(body_b);

      Ok(())
    } else {
      Err(JointError::InvalidId(joint))
    }
  }

  #[profiling]
  fn joint_detach(&self, joint: JointId) -> Result<(), JointError> {
    let mut joints = self.joints.write().unwrap();

    if let Some(joint) = joints.get_mut(joint) {
      joint.body_a = None;
      joint.body_b = None;

      Ok(())
    } else {
      Err(JointError::InvalidId(joint))
    }
  }

  #[profiling]
  fn joint_get_bodies(&self, joint: JointId) -> Option<(BodyId, BodyId)> {
    let joints = self.joints.read().unwrap();

    joints
      .get(joint)
      .map(|it| (it.body_a.unwrap_or_default(), it.body_b.unwrap_or_default()))
  }

  #[profiling]
  fn joint_set_anchor(&self, joint: JointId, anchor: Vec2) -> Result<(), JointError> {
    let mut joints = self.joints.write().unwrap();

    if let Some(joint) = joints.get_mut(joint) {
      joint.anchor = anchor;
      Ok(())
    } else {
      Err(JointError::InvalidId(joint))
    }
  }

  #[profiling]
  fn joint_get_anchor(&self, joint: JointId) -> Option<Vec2> {
    let joints = self.joints.read().unwrap();

    joints.get(joint).map(|it| it.anchor)
  }

  #[profiling]
  fn joint_delete(&self, joint: JointId) -> Result<(), JointError> {
    let mut joints = self.joints.write().unwrap();

    if joints.remove(joint).is_some() {
      Ok(())
    } else {
      Err(JointError::InvalidId(joint))
    }
  }

  #[profiling]
  fn material_create(&self) -> Result<MaterialId, MaterialError> {
    let mut materials = self.materials.write().unwrap();

    let material_id = materials.insert(Material {
      friction: 0.5,
      restitution: 0.5,
    });

    Ok(material_id)
  }

  #[profiling]
  fn material_set_friction(&self, material: MaterialId, friction: f32) -> Result<(), MaterialError> {
    let mut materials = self.materials.write().unwrap();

    if let Some(material) = materials.get_mut(material) {
      material.friction = friction;
      Ok(())
    } else {
      Err(MaterialError::InvalidId(material))
    }
  }

  #[profiling]
  fn material_get_friction(&self, material: MaterialId) -> Option<f32> {
    let materials = self.materials.read().unwrap();

    materials.get(material).map(|it| it.friction)
  }

  #[profiling]
  fn material_set_restitution(&self, material: MaterialId, restitution: f32) -> Result<(), MaterialError> {
    let mut materials = self.materials.write().unwrap();

    if let Some(material) = materials.get_mut(material) {
      material.restitution = restitution;
      Ok(())
    } else {
      Err(MaterialError::InvalidId(material))
    }
  }

  #[profiling]
  fn material_get_restitution(&self, material: MaterialId) -> Option<f32> {
    let materials = self.materials.read().unwrap();

    materials.get(material).map(|it| it.restitution)
  }

  #[profiling]
  fn material_delete(&self, material: MaterialId) -> Result<(), MaterialError> {
    let mut materials = self.materials.write().unwrap();

    if materials.remove(material).is_some() {
      Ok(())
    } else {
      Err(MaterialError::InvalidId(material))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_read_and_write_position() {
    let world = InternalPhysicsWorld2D::default();
    let body_id = world.body_create(BodyKind::Kinematic, Vec2::ZERO).unwrap();

    world.body_set_position(body_id, vec2(1., 2.)).unwrap();

    assert_eq!(world.body_get_position(body_id), Some(vec2(1., 2.)));

    world.body_delete(body_id).unwrap();

    assert_eq!(world.body_get_position(body_id), None);
  }

  #[test]
  fn it_should_read_and_write_velocity() {
    let world = InternalPhysicsWorld2D::default();
    let body_id = world.body_create(BodyKind::Kinematic, Vec2::ZERO).unwrap();

    world.body_set_velocity(body_id, vec2(1., 2.)).unwrap();

    assert_eq!(world.body_get_velocity(body_id), Some(vec2(1., 2.)));

    world.body_delete(body_id).unwrap();

    assert_eq!(world.body_get_velocity(body_id), None);
  }

  #[test]
  fn it_should_create_a_simple_joint() {
    let world = InternalPhysicsWorld2D::default();

    let body1 = world.body_create(BodyKind::Kinematic, Vec2::ZERO).unwrap();
    let body2 = world.body_create(BodyKind::Kinematic, Vec2::ZERO).unwrap();
    let joint = world.joint_create().unwrap();

    world.joint_attach(joint, body1, body2).unwrap();

    assert_eq!(world.joint_get_bodies(joint), Some((body1, body2)));
  }

  #[test]
  fn it_should_create_a_simple_material() {
    let world = InternalPhysicsWorld2D::default();

    let material = world.material_create().unwrap();
    let body = world.body_create(BodyKind::Kinematic, Vec2::ZERO).unwrap();

    world.body_set_material(body, Some(material)).unwrap();

    assert_eq!(world.body_get_material(body), Some(material));
  }
}
