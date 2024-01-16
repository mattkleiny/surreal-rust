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

  fn collider_create(&self, kind: ColliderKind) -> ColliderId {
    self.colliders.insert(Collider { _kind: kind })
  }

  fn collider_delete(&self, collider: ColliderId) {
    self.colliders.remove(collider);
  }

  fn effector_create(&self, kind: EffectorKind) -> EffectorId {
    self.effectors.insert(Effector { _kind: kind })
  }

  fn effector_delete(&self, effector: EffectorId) {
    self.effectors.remove(effector);
  }
}
