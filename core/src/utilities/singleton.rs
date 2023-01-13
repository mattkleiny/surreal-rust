use std::{
  cell::UnsafeCell,
  ops::{Deref, DerefMut},
};

use once_cell::sync::Lazy;

/// A singleton that can be referenced and mutated statically in the
/// application.
pub trait Singleton: 'static {
  /// Retrieves the static instance of this type.
  fn instance() -> &'static mut Self;
}

/// An unsafe cell for mutably alias-able [`Lazy`] values.
///
/// This type should be used very sparingly, and only to remove friction from
/// the API.
pub struct UnsafeLazyCell<T>(Lazy<UnsafeCell<T>>);

impl<T> UnsafeLazyCell<T> {
  /// Constructs a new [`UnsafeLazyCell`] with a [`Default::default`]
  /// constructor.
  pub const fn new() -> Self
  where
    T: Default,
  {
    Self(Lazy::new(|| UnsafeCell::new(T::default())))
  }
}

unsafe impl<T: Send> Send for UnsafeLazyCell<T> {}
unsafe impl<T: Sync> Sync for UnsafeLazyCell<T> {}

impl<T> Deref for UnsafeLazyCell<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &*self.0.get() }
  }
}

impl<T> DerefMut for UnsafeLazyCell<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *self.0.get() }
  }
}

#[cfg(test)]
mod tests {
  use macros::Singleton;

  use crate as surreal;

  #[derive(Singleton, Default)]
  struct TestSingleton;

  impl TestSingleton {
    pub fn example_method(&self) -> u32 {
      42u32
    }
  }

  #[test]
  fn singleton_should_access_for_read_write_usage() {
    let test = TestSingleton::instance();

    assert_eq!(42, test.example_method());
  }
}
