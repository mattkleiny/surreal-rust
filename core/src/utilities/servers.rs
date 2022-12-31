use std::ops::{Deref, DerefMut};

// TODO: improve this

/// A managed wrapper for some server backend `B`.
///
/// Servers are singletons across the application that provide a high-level
/// interface to a specific subsystem.
pub struct Server<B> {
  backend: Box<B>,
}

impl<B> Server<B> {
  /// Creates a new server with the given backend, `B`.
  pub fn new(backend: B) -> Self {
    Self {
      backend: Box::new(backend),
    }
  }
}

impl<B> Deref for Server<B> {
  type Target = B;

  fn deref(&self) -> &Self::Target {
    self.backend.as_ref()
  }
}

impl<B> DerefMut for Server<B> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.backend.as_mut()
  }
}
