//! General utilities.

pub use errors::*;
pub use events::*;
pub use owned::*;
pub use reflect::*;
pub use settings::*;
pub use singleton::*;
pub use version::*;

mod errors;
mod events;
mod owned;
mod reflect;
mod settings;
mod singleton;
mod version;

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
    pub struct $type {
      backend: core::cell::UnsafeCell<Box<dyn $backend>>,
    }

    static SINGLETON: $crate::UnsafeSingleton<$type> = $crate::UnsafeSingleton::new(|| $type {
      backend: core::cell::UnsafeCell::new(Box::new(<$default>::default())),
    });

    unsafe impl Send for $type {}
    unsafe impl Sync for $type {}

    impl $type {
      /// Gets the singleton instance of the [`$type`].
      pub fn instance() -> &'static dyn $backend {
        use std::ops::Deref;

        unsafe { SINGLETON.backend.get().as_ref().unwrap().deref() }
      }

      /// Creates a new [`$type`] for the given [`$backend`].
      pub fn install(backend: impl $backend + 'static) {
        unsafe { SINGLETON.backend.get().replace(Box::new(backend)) };
      }
    }
  };
}

/// Implements error coercion for some root error type.
#[macro_export]
macro_rules! impl_error_coercion {
  ($error:tt into $root:ty) => {
    impl From<$error> for $root {
      #[inline(always)]
      fn from(error: $error) -> Self {
        <$root>::$error(error)
      }
    }
  };
}
