//! Platform abstractions and utilities.

use crate::audio::AudioServer;
use crate::graphics::GraphicsServer;
use crate::input::InputServer;
use crate::window::WindowServer;

pub mod desktop;

pub trait Platform {
  type Audio: AudioServer;
  type Graphics: GraphicsServer;
  type Input: InputServer;
  type Window: WindowServer;

  fn audio(&mut self) -> &mut Self::Audio;
  fn graphics(&mut self) -> &mut Self::Graphics;
  fn input(&mut self) -> &mut Self::Input;
  fn window(&mut self) -> &mut Self::Window;

  /// Runs platform, invoking the given callback when available to process the next frame.
  fn run(self, callback: impl FnMut(&mut Self) -> bool);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlatformError {
  General,
}

impl From<PlatformError> for crate::Error {
  fn from(_: PlatformError) -> Self {
    crate::Error::Platform
  }
}
