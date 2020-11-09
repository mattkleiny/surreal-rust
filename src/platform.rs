//! Platform abstractions and utilities.

use crate::audio::AudioDevice;
use crate::graphics::GraphicsDevice;
use crate::input::InputDevice;

#[cfg(feature = "desktop")]
pub mod desktop;

/// Represents a platform capable of executing the application.
///
/// Platforms implement the core engine servers and provide access to them.
///
/// The platform is also responsible for the core loop, and should callback into user code
/// in order to process application logic.
pub trait Platform {
  type AudioDevice: AudioDevice;
  type GraphicsDevice: GraphicsDevice;
  type InputDevice: InputDevice;
  type PlatformWindow: PlatformWindow;

  fn audio(&mut self) -> &mut Self::AudioDevice;
  fn graphics(&mut self) -> &mut Self::GraphicsDevice;
  fn input(&mut self) -> &mut Self::InputDevice;
  fn window(&mut self) -> &mut Self::PlatformWindow;

  /// Runs platform, invoking the given callback when available to process the next frame.
  fn run(self, callback: impl FnMut(&mut Self));
}

/// Abstracts over the window provider of a device.
///
/// Permits interaction with the underlying window API through a higher-level abstraction.
pub trait PlatformWindow {
  fn set_title(&mut self, title: impl AsRef<str>);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
  General,
}

impl From<Error> for crate::Error {
  fn from(error: Error) -> Self {
    Self::Platform(error)
  }
}
