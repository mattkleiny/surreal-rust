use std::ops::Deref;

mod headless;

// A unique [`RID`] for audio resources.
surreal::impl_rid_type!(AudioId);

/// The singleton audio server implementation for the project.
///
/// All instructions to the graphics server should be sent through this facade.
/// Internally we delegate to the active [`AudioServerBackend`], which can
/// vary depending on the target platform.
pub struct AudioServer {
  backend: Box<dyn AudioServerBackend>,
}

impl AudioServer {
  /// Creates a [`AudioServer`] for a Headless, no-op backend.
  pub fn from_headless() -> surreal::Result<Self> {
    Ok(Self::from_backend(headless::HeadlessBackend::default()))
  }

  /// Create a [`AudioServer`] from the given [`AudioServerBackend`].
  pub fn from_backend(backend: impl AudioServerBackend + 'static) -> Self {
    AudioServer {
      backend: Box::new(backend),
    }
  }
}

impl Deref for AudioServer {
  type Target = dyn AudioServerBackend;

  fn deref(&self) -> &Self::Target {
    self.backend.as_ref()
  }
}

/// An abstraction on top of the underlying audio system.
///
/// This is a high-level abstraction that makes use of 'opaque' [`AudioId`]
/// to hide away implementation details. The server is intended to be a low-level
/// implementation abstraction.
pub trait AudioServerBackend {}
