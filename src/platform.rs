//! Platform abstractions and utilities.

pub use desktop::*;
pub use headless::*;
pub use io::*;
pub use memory::*;

use crate::timing::DeltaTime;

mod desktop;
mod headless;
mod io;
mod memory;

/// Possible error types for platform construction.
#[derive(Debug)]
pub enum PlatformError {
  Creation(String),
  Unknown,
}

/// An abstraction over the selected backend for the system.
pub trait Platform {
  type Host: Host;

  /// Builds the host for the platform.
  fn build(&self) -> Result<Self::Host, PlatformError>;

  /// Runs a main loop, executing the given callback inside of the given platform.
  fn execute<C>(&self, mut callback: C) -> Result<(), PlatformError>
    where Self: Sized, C: FnMut(&mut Self::Host, DeltaTime) {
    let mut host = self.build()?;

    while !host.is_closing() {
      host.tick(&mut callback);
    }

    Ok(())
  }
}

/// An abstraction over a 'host' in a particular platform.
pub trait Host {
  fn width(&self) -> u32;
  fn height(&self) -> u32;
  fn is_closing(&self) -> bool;

  /// Ticks the host by a single frame, updating any platform systems and
  /// advancing the game simulation via the given callback.
  fn tick<C>(&mut self, callback: C)
    where C: FnMut(&mut Self, DeltaTime) -> ();

  /// Exits the host, terminating the core loop.
  fn exit(&mut self);
}