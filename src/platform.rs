//! Platform abstractions and utilities.

use crate::audio::AudioDevice;
use crate::graphics::GraphicsDevice;
use crate::input::InputManager;
use crate::window::WindowManager;

#[cfg(feature = "console")]
pub mod console;
#[cfg(feature = "desktop")]
pub mod desktop;
#[cfg(feature = "mobile")]
pub mod mobile;
#[cfg(feature = "web")]
pub mod web;

/// Represents a platform capable of executing the application.
///
/// Platforms implement the core engine servers and provide access to them.
///
/// The platform is also responsible for the core loop, and should callback into user code
/// in order to process application logic.
pub trait Platform {
  type Audio: AudioDevice;
  type Graphics: GraphicsDevice;
  type Input: InputManager;
  type Window: WindowManager;

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
