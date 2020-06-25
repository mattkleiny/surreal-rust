//! Platform abstractions and utilities.

pub use desktop::*;

use crate::audio::AudioServer;
use crate::graphics::GraphicsServer;
use crate::input::InputServer;

pub mod desktop;

/// Represents a platform for game development.
///
/// Implementation of this trait provide servers for the various sub-systems that a game needs.
pub trait Platform {
  type Audio: AudioServer;
  type Graphics: GraphicsServer;
  type Input: InputServer;

  fn run(&mut self, callback: impl FnMut(&mut Self) -> bool);
  fn audio(&mut self) -> &mut Self::Audio;
  fn graphics(&mut self) -> &mut Self::Graphics;
  fn input(&mut self) -> &mut Self::Input;
}

/// Represents a general error in the underlying platform.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlatformError {
  FailedToCreate
}

impl From<PlatformError> for crate::Error {
  fn from(_: PlatformError) -> Self {
    crate::Error::PlatformFailure
  }
}
