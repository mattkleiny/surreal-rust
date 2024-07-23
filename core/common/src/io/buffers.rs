/// A buffer of data stored with an accessible heap-allocated lifetime.
#[repr(transparent)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Buffer<T> {
  data: Box<[T]>,
}

impl<T> Buffer<T> {
  /// Creates a new buffer with the given data.
  #[inline]
  pub fn new(data: Box<[T]>) -> Self {
    Buffer { data }
  }

  /// Creates a new buffer with the given data.
  #[inline]
  pub fn from_vec(data: Vec<T>) -> Self {
    Buffer {
      data: data.into_boxed_slice(),
    }
  }

  /// Returns a reference to the buffer's data.
  #[inline]
  pub fn as_slice(&self) -> &[T] {
    &self.data
  }

  /// Returns a mutable reference to the buffer's data.
  #[inline]
  pub fn as_mut_slice(&mut self) -> &mut [T] {
    &mut self.data
  }

  /// Returns a pointer to the buffer's data.
  #[inline]
  pub fn as_ptr(&self) -> *const T {
    self.data.as_ptr()
  }

  /// Returns a mutable pointer to the buffer's data.
  #[inline]
  pub fn as_mut_ptr(&mut self) -> *mut T {
    self.data.as_mut_ptr()
  }

  /// Consumes the buffer and returns its data.
  #[inline]
  pub fn into_vec(self) -> Vec<T> {
    self.data.into_vec()
  }
}

impl<T> From<Vec<T>> for Buffer<T> {
  #[inline]
  fn from(value: Vec<T>) -> Self {
    Buffer::from_vec(value)
  }
}

impl<T> From<Box<[T]>> for Buffer<T> {
  #[inline]
  fn from(value: Box<[T]>) -> Self {
    Buffer::new(value)
  }
}

impl<T> From<Buffer<T>> for Vec<T> {
  #[inline]
  fn from(value: Buffer<T>) -> Self {
    value.into_vec()
  }
}

impl<T> From<Buffer<T>> for Box<[T]> {
  #[inline]
  fn from(value: Buffer<T>) -> Self {
    value.data
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn buffer_should_create_from_vec() {
    let buffer = Buffer::from_vec(vec![1, 2, 3]);

    assert_eq!(buffer.as_slice(), &[1, 2, 3]);
  }
}
