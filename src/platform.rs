//! Platform abstractions and utilities.

use crate::audio::Audio;
use crate::graphics::Graphics;
use crate::input::Input;

#[cfg(feature = "desktop")]
pub mod desktop;

/// Represents a platform capable of executing the application.
///
/// Platforms implement the core engine servers and provide access to them.
///
/// The platform is also responsible for the core loop, and should callback into user code
/// in order to process application logic.
pub trait Platform {
  type Audio: Audio;
  type Graphics: Graphics;
  type Input: Input;
  type Window: Window;

  fn audio(&mut self) -> &mut Self::Audio;
  fn graphics(&mut self) -> &mut Self::Graphics;
  fn input(&mut self) -> &mut Self::Input;
  fn window(&mut self) -> &mut Self::Window;

  /// Runs platform, invoking the given callback when available to process the next frame.
  fn run(self, callback: impl FnMut(&mut Self));
}

/// Abstracts over the window provider of a device.
///
/// Permits interaction with the underlying window API through a higher-level abstraction.
pub trait Window {
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
