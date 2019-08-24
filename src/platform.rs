//! Platform abstractions and utilities.

pub use desktop::*;
pub use headless::*;
pub use io::*;
pub use memory::*;

use crate::audio::AudioDevice;
use crate::graphics::GraphicsDevice;
use crate::input::InputDevice;

use super::*;

mod desktop;
mod headless;
mod io;
mod memory;

/// An abstraction over the selected backend for the system.
pub trait Platform {
  type Host: Host;
  type Allocator: Allocator;
  type FileSystem: FileSystem;

  /// Builds the host for the platform.
  fn build(&self) -> Result<Self::Host>;

  /// Runs a main loop, executing the given callback inside of the given platform.
  fn execute<C>(&self, mut callback: C)
    where Self: Sized, C: FnMut(&mut Self::Host, f64) -> () {
    let mut host = self
        .build()
        .expect("Failed to build the platform host!");

    while !host.is_closing() {
      host.tick(&mut callback);
    }
  }
}

/// An abstraction over a 'host' in a particular platform.
pub trait Host: AudioDevice + GraphicsDevice + InputDevice {
  fn width(&self) -> u32;
  fn height(&self) -> u32;
  fn is_closing(&self) -> bool;

  /// Ticks the host by a single frame, updating any platform systems and
  /// advancing the game simulation via the given callback.
  fn tick<C>(&mut self, callback: C)
    where C: FnMut(&mut Self, f64) -> ();

  /// Exits the host, terminating the core loop.
  fn exit(&mut self);
}