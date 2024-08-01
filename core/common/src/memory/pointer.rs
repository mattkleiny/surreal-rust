use std::{
  fmt::Debug,
  marker::Unsize,
  ops::{CoerceUnsized, Deref, DerefMut},
};

/// A pointer to a heap-allocated object.
///
/// This is an unsafe version of `Box` that allows for more fine-grained control
/// over the memory management of the object.
///
/// When the value that the pointer is pointing to is no longer needed, the
/// `delete` method should be called to deallocate the memory.
///
/// This type can be used to share ownership of a heap-allocated object between
/// multiple threads, or to pass ownership of a heap-allocated object to a C
/// library.
///
/// # Safety
///
/// This type is unsafe because it allows for the creation of multiple mutable
/// references to the same object, which can lead to undefined behavior if not
/// used correctly.
pub struct Pointer<T: ?Sized> {
  ptr: *mut T,
}

impl<T> Pointer<T> {
  /// Creates a new pointer from a boxed object.
  pub fn new(value: T) -> Self {
    Self::from_box(Box::new(value))
  }
}

impl<T: ?Sized> Pointer<T> {
  /// Creates a new pointer from a boxed object.
  pub fn from_box(boxed: Box<T>) -> Self {
    Self {
      ptr: Box::into_raw(boxed),
    }
  }

  /// Consumes the pointer and returns the boxed object.
  #[inline(always)]
  pub fn into_box(self) -> Box<T> {
    unsafe { Box::from_raw(self.ptr) }
  }

  /// Creates a new pointer from a raw pointer.
  #[inline(always)]
  pub fn from_raw(ptr: *const T) -> Self {
    Self { ptr: ptr as *mut T }
  }

  /// Creates a new pointer from a raw pointer.
  #[inline(always)]
  pub fn from_raw_mut(ptr: *mut T) -> Self {
    Self { ptr }
  }

  /// Consumes the pointer and returns the raw pointer.
  #[inline(always)]
  pub const fn into_raw(self) -> *mut T {
    self.ptr
  }

  /// Consumes the pointer and returns the raw pointer as a void pointer.
  #[inline(always)]
  pub const fn into_void(self) -> *mut std::ffi::c_void {
    self.ptr as *mut std::ffi::c_void
  }

  /// Returns a reference to the object.
  #[inline(always)]
  pub fn as_ref(&self) -> &T {
    unsafe { &*self.ptr }
  }

  /// Returns a mutable reference to the object.
  #[inline(always)]
  pub fn as_mut(&mut self) -> &mut T {
    unsafe { &mut *self.ptr }
  }

  /// Allow casting of the pointer from one type to another.
  ///
  /// It's up to the caller to ensure that the types are compatible.
  #[inline(always)]
  pub unsafe fn cast_unchecked<U>(self) -> Pointer<U> {
    Pointer::from_raw_mut(self.into_raw() as *mut U)
  }

  /// Frees the memory allocated by the pointer.
  pub fn delete(self) {
    unsafe {
      drop(Box::from_raw(self.ptr));
    }
  }
}

/// Allow printing of the pointer.
impl<T: ?Sized> Debug for Pointer<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("Pointer").field(&self.ptr).finish()
  }
}

/// Allow cloning of the pointer.
impl<T: ?Sized> Clone for Pointer<T> {
  fn clone(&self) -> Self {
    Self { ptr: self.ptr }
  }
}

/// Allow dereferencing of the pointer.
impl<T: ?Sized> Deref for Pointer<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.as_ref()
  }
}

/// Allow mutable dereferencing of the pointer.
impl<T: ?Sized> DerefMut for Pointer<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.as_mut()
  }
}

/// Allow unsized coercion between pointers types.
impl<T: ?Sized, U: ?Sized + Unsize<T>> CoerceUnsized<Pointer<T>> for Pointer<U> {}
