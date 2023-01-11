//! General utilities.

pub use object::*;
pub use services::*;
pub use singleton::*;
pub use size::*;
pub use timing::*;
pub use variant::*;
pub use version::*;

mod object;
mod services;
mod singleton;
mod size;
mod timing;
mod variant;
mod version;

/// Abstracts over resource IDs.
pub trait RID: Copy + Eq + std::hash::Hash + From<u64> {}

/// Creates an opaque ID for a resource in a implementation.
#[macro_export]
macro_rules! impl_rid {
  ($name:ident) => {
    #[repr(transparent)]
    #[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct $name(pub u64);

    impl $name {
      #[inline]
      pub const fn new(id: u64) -> Self {
        Self(id)
      }
    }

    impl $crate::utilities::RID for $name {}

    impl From<u64> for $name {
      #[inline]
      fn from(id: u64) -> Self {
        Self(id)
      }
    }
  };
}

/// Creates a server wrapper for some server backend.
#[macro_export]
macro_rules! impl_server {
  ($server:ident, $backend:ty) => {
    /// A wrapper for the core [`$backend`] implementation.
    #[derive(Clone)]
    pub struct $server {
      backend: std::rc::Rc<Box<$backend>>,
    }

    impl $server {
      /// Creates a new [`$server`] for the given [`$backend`].
      pub fn from_backend(backend: impl $backend + 'static) -> Self {
        Self {
          backend: std::rc::Rc::new(Box::new(backend)),
        }
      }
    }

    impl std::ops::Deref for $server {
      type Target = $backend;

      fn deref(&self) -> &Self::Target {
        self.backend.as_ref().as_ref()
      }
    }
  };
}

/// Creates an unsafe mutable alias to the given value.
///
/// This breaks many assumptions in the Rust type system, so use with great caution.
#[inline(always)]
pub(crate) fn unsafe_mutable_alias<'a, T>(value: &T) -> &'a mut T {
  unsafe {
    let pointer = value as *const T;
    let pointer = pointer as *mut T;

    &mut *pointer
  }
}
