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

/// Represents a type that can be used as a singleton.
pub trait Singleton {
  /// Returns the singleton instance of this type.
  fn instance() -> &'static Self;
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
  ($type:ident by $backend:ident default $default:ty) => {
    /// A wrapper for the core implementation.
    pub struct $type {
      backend: core::cell::UnsafeCell<Box<dyn $backend>>,
    }

    static INSTANCE: std::sync::LazyLock<$type> = std::sync::LazyLock::new(|| $type {
      backend: core::cell::UnsafeCell::new(Box::new(<$default>::default())),
    });

    unsafe impl Send for $type {}
    unsafe impl Sync for $type {}

    impl $type {
      /// Gets the singleton instance of the [`$type`].
      pub fn instance() -> &'static dyn $backend {
        use std::ops::Deref;

        unsafe { INSTANCE.backend.get().as_ref().unwrap().deref() }
      }

      /// Creates a new [`$type`] for the given [`$backend`].
      pub fn install(backend: impl $backend + 'static) {
        unsafe { INSTANCE.backend.get().replace(Box::new(backend)) };
      }
    }
  };
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
