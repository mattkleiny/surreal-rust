//! Platform abstractions and utilities.

#[cfg(feature = "desktop")]
pub use desktop::*;
#[cfg(feature = "headless")]
pub use headless::*;

pub use crate::audio::AudioServer;
pub use crate::graphics::GraphicsServer;
pub use crate::input::InputServer;

#[cfg(feature = "desktop")]
pub mod desktop;
#[cfg(feature = "headless")]
pub mod headless;

/// Represents a fallible result in the platform subsystem.
pub type PlatformResult<T> = anyhow::Result<T>;

/// Represents a platform capable of executing the application.
///
/// Platforms implement the core engine servers and provide access to them to user code.
///
/// The platform is also responsible for the core loop, and should callback into user code
/// in order to process application logic.
pub trait Platform {
  /// The type of `PlatformHost` that this platform creates.
  type Host: PlatformHost;

  /// Creates a new instance of the platform host.
  fn create_host(&self) -> Self::Host;
}

/// A host for a particular `Platform`, allowing user code to interact with low-level platform
/// resources and configuration.
///
/// An example host is the main window for a game in desktop environments.
pub trait PlatformHost {
  fn width(&self) -> usize;
  fn height(&self) -> usize;
  fn is_visible(&self) -> bool;
  fn is_focused(&self) -> bool;
  fn is_closing(&self) -> bool;
  fn run(&mut self, body: impl FnMut(&mut Self));
  fn exit(&mut self);
}