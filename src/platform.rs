//! Platform abstractions and utilities.

pub use desktop::*;
pub use headless::*;
pub use io::*;
pub use memory::*;

use crate::audio::AudioDevice;
use crate::graphics::GraphicsDevice;
use crate::input::InputDevice;
use crate::timing::DeltaTime;

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
  type AudioDevice: AudioDevice;
  type GraphicsDevice: GraphicsDevice;
  type InputDevice: InputDevice;

  /// Builds the host for the platform.
  fn build(&self) -> Result<Self::Host>;

  /// Runs a main loop, executing the given callback inside of the given platform.
  fn execute<C>(&self, mut callback: C)
    where Self: Sized, C: FnMut(&mut Self::Host, DeltaTime) -> () {
    let mut host = self
        .build()
        .expect("Failed to build the platform host!");

    while !host.is_closing() {
      host.tick(&mut callback);
    }
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