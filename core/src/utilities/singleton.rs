use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

use once_cell::sync::Lazy;

/// A singleton that can be referenced and mutated statically in the application.
pub trait Singleton: 'static {
  /// Retrieves the static instance of this type.
  fn instance() -> &'static mut Self;
}

/// An unsafe cell for mutably alias-able singleton values.
///
/// This is a wrapper around [`UnsafeCell`] that provides an interface for
/// accessing the singleton data.
///
/// This type should be used very sparingly, and only to remove friction from the API.
// TODO: make this safe?
pub struct SingletonCell<T>(Lazy<UnsafeCell<T>>);

impl<T: Singleton + Default> SingletonCell<T> {
  /// Constructs a new [`SingletonCell`] with a default constructor.
  pub const fn new() -> Self {
    Self(Lazy::new(|| UnsafeCell::new(T::default())))
  }
}

unsafe impl<T> Send for SingletonCell<T> {}
unsafe impl<T> Sync for SingletonCell<T> {}

impl<T> Deref for SingletonCell<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &*self.0.get() }
  }
}

impl<T> DerefMut for SingletonCell<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *self.0.get() }
  }
}

/// Declares a singleton instance of the given type.
///
/// The singleton can have no constructed dependencies, and is expected
/// to run in complete isolation of the rest of the application.
#[macro_export]
macro_rules! impl_singleton {
  ($target:ident) => {
    use $crate::utilities::{Singleton, SingletonCell};

    impl Singleton for $target {
      fn instance() -> &'static mut Self {
        static mut INSTANCE: SingletonCell<$target> = SingletonCell::new();

        unsafe { &mut INSTANCE }
      }
    }
  };
}

#[cfg(test)]
mod tests {
  #[derive(Default)]
  struct TestSingleton;

  impl TestSingleton {
    pub fn example_method(&self) -> u32 {
      42u32
    }
  }

  impl_singleton!(TestSingleton);

  #[test]
  fn singleton_should_access_for_read_write_usage() {
    let test = TestSingleton::instance();

    assert_eq!(42, test.example_method());
  }
}
