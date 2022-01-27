//! Platform abstractions and utilities.

use std::sync::Mutex;
#[cfg(feature = "desktop")]
pub use desktop::*;

use crate::audio::AudioServer;
use crate::graphics::GraphicsServer;

#[cfg(feature = "desktop")]
pub mod desktop;

/// Represents a fallible result in the platform subsystem.
pub type PlatformResult<T> = anyhow::Result<T>;

/// Represents a platform capable of executing the application.
///
/// Platforms implement the core engine servers and provide access to them.
///
/// The platform is also responsible for the core loop, and should callback into user code
/// in order to process application logic.
pub trait Platform {
  fn audio(&self) -> &Mutex<dyn AudioServer>;
  fn graphics(&self) -> &Mutex<dyn GraphicsServer>;

  /// Runs platform, invoking the given callback when available to process the next frame.
  fn run(self);
}