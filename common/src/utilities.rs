//! General utilities.

pub use bytemuck;
pub use events::*;
pub use size::*;
pub use timing::*;
pub use variant::*;
pub use version::*;

mod events;
mod size;
mod timing;
mod variant;
mod version;

/// Implements a new server type for the given backend.
#[macro_export]
macro_rules! impl_server {
  ($type:ident, $backend:ident) => {
    /// A wrapper for the core implementation.
    #[derive(Clone)]
    pub struct $type {
      backend: std::sync::Arc<Box<dyn $backend>>,
    }

    impl $type {
      /// Creates a new [`$type`] for the given [`$backend`].
      pub fn new(backend: impl $backend + 'static) -> Self {
        Self {
          backend: std::sync::Arc::new(Box::new(backend)),
        }
      }
    }

    unsafe impl Send for $type {}
    unsafe impl Sync for $type {}

    impl std::ops::Deref for $type {
      type Target = Box<dyn $backend>;

      fn deref(&self) -> &Self::Target {
        self.backend.as_ref()
      }
    }
  };
}
