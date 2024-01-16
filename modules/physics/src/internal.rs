use super::*;

/// The default, home-baked [`PhysicsBackend`].
///
/// This is a simple physics backend that uses a simple Euler integration
/// algorithm to simulate the physics of the game world.
#[derive(Default)]
pub struct InternalPhysicsBackend {
  rigidbodies: ResourceStorage<RigidbodyId, Rigidbody>,
  colliders: ResourceStorage<ColliderId, Collider>,
  effectors: ResourceStorage<EffectorId, Effector>,
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

impl PhysicsBackend for InternalPhysicsBackend {
  fn step(&self, _delta_time: f32) {
    todo!()
  }

  fn reset(&self) {
    self.rigidbodies.clear();
    self.colliders.clear();
  }

  fn rigidbody_create(&self, kind: RigidbodyKind) -> RigidbodyId {
    self.rigidbodies.insert(Rigidbody {
      _kind: kind,
      colliders: Vec::new(),
    })
  }

  fn rigidbody_add_collider(&self, body: RigidbodyId, collider: ColliderId) {
    self.rigidbodies.write(body, |body| {
      body.colliders.push(collider);
    });
  }

  fn rigidbody_remove_collider(&self, body: RigidbodyId, collider: ColliderId) {
    self.rigidbodies.write(body, |body| {
      body.colliders.retain(|c| *c != collider);
    });
  }

  fn rigidbody_delete(&self, body: RigidbodyId) {
    self.rigidbodies.remove(body);
  }

  fn effector_create(&self, kind: EffectorKind) -> EffectorId {
    self.effectors.insert(Effector { _kind: kind })
  }

  fn effector_delete(&self, effector: EffectorId) {
    self.effectors.remove(effector);
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

  fn collider_set_rotation(&self, collider: ColliderId, rotation: Quat) {
    todo!()
  }

  fn collider_set_scale(&self, collider: ColliderId, scale: Vec3) {
    todo!()
  }

  fn collider_delete(&self, collider: ColliderId) {
    self.colliders.remove(collider);
  }
}
