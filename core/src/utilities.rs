//! General utilities.

pub use bytemuck;
pub use object::*;
pub use parsing::*;
pub use services::*;
pub use singleton::*;
pub use size::*;
pub use timing::*;
pub use variant::*;
pub use version::*;

mod object;
mod parsing;
mod services;
mod singleton;
mod size;
mod timing;
mod variant;
mod version;

/// Creates an unsafe mutable alias to the given value.
///
/// This breaks many assumptions in the Rust type system, so use with great
/// caution and only to facilitate a cleaner API.
#[inline(always)]
pub(crate) fn unsafe_mutable_alias<'a, T>(value: &T) -> &'a mut T {
  unsafe {
    let pointer = value as *const T;
    let pointer = pointer as *mut T;

    &mut *pointer
  }
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
      /// Creates a new [`$type ] for the given [`$backend`].
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
