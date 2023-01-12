//! General utilities.

pub use object::*;
pub use services::*;
pub use singleton::*;
pub use size::*;
pub use timing::*;
pub use variant::*;
pub use version::*;

use crate::collections::ArenaIndex;

mod object;
mod services;
mod singleton;
mod size;
mod timing;
mod variant;
mod version;

/// Abstracts over resource IDs.
pub trait ResourceId: Copy + Eq + From<ArenaIndex> + Into<ArenaIndex> {}

/// Creates a new, opaque [`ResourceId`] type.
#[macro_export]
macro_rules! impl_rid {
  ($name:ident) => {
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub struct $name($crate::collections::ArenaIndex);

    impl $crate::utilities::ResourceId for $name {}

    impl From<$crate::collections::ArenaIndex> for $name {
      #[inline]
      fn from(id: $crate::collections::ArenaIndex) -> Self {
        Self(id)
      }
    }

    impl From<$name> for $crate::collections::ArenaIndex {
      #[inline]
      fn from(id: $name) -> Self {
        id.0
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
