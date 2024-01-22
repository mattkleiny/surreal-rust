//! General utilities.

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

pub use macros::Singleton;

/// Reinterprets the given reference as a reference to a different type.
#[inline(always)]
pub unsafe fn reinterpret_cast<T, U>(value: &T) -> &U {
  unsafe { &*(value as *const T as *const U) }
}

/// Mutably reinterprets the given reference as a reference to a different type.
#[inline(always)]
pub unsafe fn reinterpret_cast_mut<T, U>(value: &mut T) -> &mut U {
  unsafe { &mut *(value as *mut T as *mut U) }
}

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
