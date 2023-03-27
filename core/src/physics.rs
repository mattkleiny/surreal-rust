//! A lightweight physics engine for Surreal.

crate::impl_server!(PhysicsServer, PhysicsBackend);

crate::impl_rid!(ColliderId);
crate::impl_rid!(BodyId);

impl Default for PhysicsServer {
  fn default() -> Self {
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
struct DefaultPhysicsBackend {}

impl PhysicsBackend for DefaultPhysicsBackend {
  fn step(&self, delta_time: f32) {
    println!("Physics step: {}", delta_time);
  }

  fn clear(&self) {
    // no-op
  }
}
