//! Platform abstractions and utilities.

/// An abstraction over the selected backend for the system.
pub trait Platform {
  type Host;
  type Error;

  /// Builds the host for the platform.
  fn build(&self) -> Result<Self::Host, Self::Error>;
}

/// An abstraction over a 'host' in a particular platform.
pub trait Host<P: Platform> {
  /// Ticks the host by a single frame, updating any platform systems and
  /// advancing the game simulation via the given callback.
  fn tick<C>(&mut self, callback: C)
    where C: FnMut(&mut Self, f32) -> ();
}