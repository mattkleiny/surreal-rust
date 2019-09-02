//! Platform abstractions and utilities.

use crate::graphics::GraphicsDevice;

pub mod desktop;
pub mod memory;

/// An abstraction over the selected backend for the system.
pub trait Platform: Sized {
  type Host: Host<Self>;
  type GraphicsDevice: GraphicsDevice;
  type Error;

  /// Builds the host for the platform.
  fn build(&self) -> Result<Self::Host, Self::Error>;
}

/// An abstraction over a 'host' in a particular platform.
pub trait Host<P: Platform>: Sized {
  fn graphics_device(&self) -> &P::GraphicsDevice;
  fn is_closing(&self) -> bool;

  /// Ticks the host by a single frame, updating any platform systems and
  /// advancing the game simulation via the given callback.
  fn tick<C>(&mut self, callback: C)
    where C: FnMut(&mut Self, f32) -> ();

  /// Runs a main loop, executing the given callback inside of the given platform.
  fn execute<C>(&mut self, mut callback: C)
    where C: FnMut(&mut Self, f32) {
    while !self.is_closing() {
      self.tick(&mut callback);
    }
  }

  /// Exits the host, terminating the entities loop.
  fn exit(&mut self);
}