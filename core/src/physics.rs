//! A physics engine for Surreal.

use crate::collections::ResourceStorage;

crate::impl_server!(PhysicsServer, PhysicsBackend);

crate::impl_rid!(ColliderId);
crate::impl_rid!(BodyId);

impl PhysicsServer {
  /// Creates a new physics server with the default physics backend.
  pub fn default() -> Self {
    Self::new(DefaultPhysicsBackend::default())
  }
}

/// A trait for physics backends.
///
/// This trait is implemented by physics backends, which are responsible for
/// simulating the physics of the game world.
///
/// The physics backend is responsible for simulating the physics of the game
/// world. It is responsible for updating the position and orientation of
/// physical objects, and for detecting collisions between objects.
pub trait PhysicsBackend {
  fn step(&mut self, delta_time: f32);
}

/// The default, home-baked [`PhysicsBackend`].
///
/// This is a simple physics backend that uses a simple Euler integration
/// algorithm to simulate the physics of the game world.
#[derive(Default)]
struct DefaultPhysicsBackend {
  _colliders: ResourceStorage<ColliderId, Collider>,
  _bodies: ResourceStorage<BodyId, Body>,
}

struct Collider {}

struct Body {}

impl PhysicsBackend for DefaultPhysicsBackend {
  fn step(&mut self, delta_time: f32) {
    println!("Physics step: {}", delta_time);
  }
}
