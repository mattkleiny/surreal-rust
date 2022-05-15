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
/// Platforms implement the core engine servers and provide access to them.
///
/// The platform is also responsible for the core loop, and should callback into user code
/// in order to process application logic.
pub trait Platform {
  fn tick(&mut self);
}