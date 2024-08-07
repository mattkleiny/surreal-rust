use std::{
  cell::UnsafeCell,
  mem::MaybeUninit,
  ops::{Deref, DerefMut},
  sync::Once,
};

/// An unsafe singleton that can be used to store a single value.
///
/// The value is lazily initialized and can be accessed through the `Deref` and
/// `DerefMut` traits.
pub struct UnsafeSingleton<T> {
  lock: Once,
  value: UnsafeCell<MaybeUninit<T>>,
  factory: fn() -> T,
}

unsafe impl<T> Send for UnsafeSingleton<T> {}
unsafe impl<T> Sync for UnsafeSingleton<T> {}

impl<T: Default> UnsafeSingleton<T> {
  /// Creates a new [`UnsafeSingleton`] with the default value.
  pub const fn default() -> Self {
    Self::new(T::default)
  }
}

impl<T> UnsafeSingleton<T> {
  /// Creates a new [`UnsafeSingleton`] with the given value.
  pub const fn new(factory: fn() -> T) -> Self {
    Self {
      lock: Once::new(),
      value: UnsafeCell::new(MaybeUninit::uninit()),
      factory,
    }
  }
}

/// Allows dereferencing the singleton to the inner value.
impl<T> Deref for UnsafeSingleton<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.lock.call_once(|| {
      let value = unsafe { &mut *self.value.get() };
      let actual = (self.factory)();

      *value = MaybeUninit::new(actual);
    });

    unsafe { self.value.get().as_ref() }
      .map(|it| unsafe { &*it.as_ptr() })
      .unwrap()
  }
}

/// Allows mutable dereferencing the singleton to the inner value.
impl<T> DerefMut for UnsafeSingleton<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.lock.call_once(|| {
      let value = unsafe { &mut *self.value.get() };
      let actual = (self.factory)();

      *value = MaybeUninit::new(actual);
    });

    unsafe { self.value.get().as_mut() }
      .map(|it| unsafe { &mut *it.as_mut_ptr() })
      .unwrap()
  }
}
