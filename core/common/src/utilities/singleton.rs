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
pub struct Singleton<T> {
  lock: Once,
  value: UnsafeCell<MaybeUninit<T>>,
  factory: fn() -> T,
}

unsafe impl<T> Send for Singleton<T> {}
unsafe impl<T> Sync for Singleton<T> {}

impl<T: Default> Singleton<T> {
  /// Creates a new [`Singleton`] with the default value.
  pub const fn default() -> Self {
    Self::new(T::default)
  }
}

impl<T> Singleton<T> {
  /// Creates a new [`Singleton`] with the given value.
  pub const fn new(factory: fn() -> T) -> Self {
    Self {
      lock: Once::new(),
      value: UnsafeCell::new(MaybeUninit::uninit()),
      factory,
    }
  }
}

/// Allows dereferencing the singleton to the inner value.
impl<T> Deref for Singleton<T> {
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
impl<T> DerefMut for Singleton<T> {
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
