//! A physics engine for Surreal.

use crate::{collections::ResourceStorage, maths::DVec2};

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
  /// Advances the physics simulation by the given amount of time.
  fn step(&self, delta_time: f32);

  /// Resets the entire physics simulation.
  fn clear(&self);
}

/// The default, home-baked [`PhysicsBackend`].
///
/// This is a simple physics backend that uses a simple Euler integration
/// algorithm to simulate the physics of the game world.
#[derive(Default)]
struct DefaultPhysicsBackend {
  colliders: ResourceStorage<ColliderId, Collider>,
  rigidbodies: ResourceStorage<BodyId, RigidBody>,
}

/// A collider in the physics simulation.
enum Collider {
  Circle { radius: f64 },
  Rectangle { half_extents: DVec2 },
}

/// A rigid body in the physics simulation.
enum RigidBody {
  Static,
  Kinematic {
    velocity: DVec2,
    angular_velocity: f64,
  },
  Dynamic {
    velocity: DVec2,
    angular_velocity: f64,
  },
}

impl PhysicsBackend for DefaultPhysicsBackend {
  fn step(&self, delta_time: f32) {
    println!("Physics step: {}", delta_time);
  }

  fn clear(&self) {
    self.colliders.clear();
    self.rigidbodies.clear();
  }
}
