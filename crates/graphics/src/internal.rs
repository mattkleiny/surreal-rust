use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// A helper for working with internal graphics state.
pub(crate) struct GraphicsCell<T> {
  state: Arc<RwLock<T>>,
}

impl<T> Clone for GraphicsCell<T> {
  fn clone(&self) -> Self {
    Self {
      state: self.state.clone(),
    }
  }
}

impl<T> GraphicsCell<T> {
  /// Creates a new graphics state with the given value.
  pub fn new(value: T) -> Self {
    Self {
      state: Arc::new(RwLock::new(value)),
    }
  }

  /// Locks the state for reading.
  #[inline]
  pub fn read(&self) -> RwLockReadGuard<T> {
    self.state.read().expect("Failed to lock graphics state for reading")
  }

  /// Locks the state for reading and writing.
  #[inline]
  pub fn write(&self) -> RwLockWriteGuard<T> {
    self.state.write().expect("Failed to lock graphics state for writing")
  }

  /// Locks the state for reading and calls the given function with
  #[inline]
  pub fn with_read<R>(&self, body: impl FnOnce(&T) -> R) -> R {
    body(&self.read())
  }

  /// Locks the state for reading and writing and calls the given function with
  #[inline]
  pub fn with_write<R>(&self, body: impl FnOnce(&mut T) -> R) -> R {
    body(&mut self.write())
  }
}
