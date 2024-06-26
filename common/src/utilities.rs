//! General utilities.

pub use errors::*;
pub use events::*;
pub use reflect::*;
pub use settings::*;
pub use version::*;

mod errors;
mod events;
mod reflect;
mod settings;
mod version;

pub use macros::{Reflect, Singleton};

/// Represents a type that can be used as a singleton.
pub trait Singleton {
  /// Returns the singleton instance of this type.
  fn instance() -> &'static Self;
}

/// Creates a singleton instance of the given type.
#[macro_export]
macro_rules! impl_singleton {
  ($name:ty) => {
    impl crate::utilities::Singleton for $name {
      fn instance() -> &'static Self {
        static INSTANCE: std::sync::LazyLock<$name> = std::sync::LazyLock::new(|| <$name>::default());

        std::ops::Deref::deref(&INSTANCE)
      }
    }
  };
}

/// Reinterprets the given reference as a reference to a different type.
///
/// # Safety
/// This is only safe if the new type is the same size as the old type.
#[inline(always)]
pub unsafe fn reinterpret_cast<T, U>(value: &T) -> &U {
  unsafe { &*(value as *const T as *const U) }
}

/// Reinterprets the given mutable reference as a reference to a different type.
///
/// # Safety
/// This is only safe if the new type is the same size as the old type.
#[inline(always)]
pub unsafe fn reinterpret_cast_mut<T, U>(value: &mut T) -> &mut U {
  unsafe { &mut *(value as *mut T as *mut U) }
}

/// Creates an unsafe mutable alias to the given value.
///
/// This breaks many assumptions in the Rust type system, so use with great
/// caution and only to facilitate a cleaner API.
///
/// # Safety
/// This is only safe if the given value is not modified while the alias is
/// alive. This is _very_ hard to guarantee.
#[inline(always)]
#[allow(invalid_reference_casting)]
pub unsafe fn unsafe_mutable_alias<'a, T>(value: &T) -> &'a mut T {
  // TODO: find a way to remove this completely
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
      /// Creates a new [`$type`] for the given [`$backend`].
      pub fn new(backend: impl $backend + 'static) -> Self {
        Self {
          backend: std::sync::Arc::new(Box::new(backend)),
        }
      }
    }

    impl std::ops::Deref for $type {
      type Target = Box<dyn $backend>;

      fn deref(&self) -> &Self::Target {
        self.backend.as_ref()
      }
    }
  };
}

/// Implements a conversion from on error type to a nested variant in another.
#[macro_export]
macro_rules! impl_from_error {
  ($error:tt for $other:tt) => {
    impl From<$error> for $other {
      #[inline(always)]
      fn from(error: $error) -> Self {
        Self::$error(error)
      }
    }
  };
}
