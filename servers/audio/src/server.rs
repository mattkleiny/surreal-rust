use std::ops::Deref;

mod headless;
mod rodio;

/// The singleton audio server implementation for the project.
///
/// All instructions to the graphics server should be sent through this facade.
/// Internally we delegate to the active [`AudioBackend`], which can
/// vary depending on the target platform.
pub struct AudioServer {
  backend: std::sync::Arc<dyn AudioBackend>,
}

impl AudioServer {
  /// Creates a [`AudioServer`] for a Headless, no-op backend.
  pub fn from_headless() -> Self {
    Self::from_backend(headless::HeadlessAudioBackend::default())
  }

  /// Creates an [`AudioServer`] that's backed by Rodio as the audio engine.
  pub fn from_rodio() -> surreal::Result<Self> {
    Ok(Self::from_backend(rodio::RodioBackend::new()?))
  }

  /// Create a [`AudioServer`] from the given [`AudioBackend`].
  pub fn from_backend(backend: impl AudioBackend + 'static) -> Self {
    AudioServer {
      backend: std::sync::Arc::new(backend),
    }
  }
}

unsafe impl Send for AudioServer {}
unsafe impl Sync for AudioServer {}

impl Deref for AudioServer {
  type Target = dyn AudioBackend;

  fn deref(&self) -> &Self::Target {
    self.backend.as_ref()
  }
}

/// An abstraction on top of the underlying graphics API.
///
/// This is a mid-level abstraction that makes use of 'opaque' resource IDs to
/// hide away implementation details and lifetimes. The backend forms the
/// foundation of higher-level abstractions that make it simpler to build
/// graphics programs.
pub trait AudioBackend {}

surreal::impl_rid!(AudioClipId);
surreal::impl_rid!(AudioSourceId);
