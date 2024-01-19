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

/// Implements owned and borrowed string conversions for a type.
#[macro_export]
macro_rules! impl_string {
  ($type:ident) => {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct $type<'a>(Cow<'a, str>);

    impl<'a> From<&'a str> for $type<'a> {
      fn from(value: &'a str) -> Self {
        Self(std::borrow::Cow::Borrowed(value))
      }
    }

    impl<'a> From<String> for $type<'a> {
      fn from(value: String) -> Self {
        Self(std::borrow::Cow::Owned(value))
      }
    }
  };
}
