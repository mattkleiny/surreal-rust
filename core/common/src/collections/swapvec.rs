use std::ops::RangeFull;

/// A red/green swap [`Vec`].
///
/// This is a specialized [`Vec`] that allows for efficient red/green swaps,
/// such that one [`Vec`] is active while the other is being written to. This is
/// useful for scheduling tasks or for active/free lists.
pub struct SwapVec<T> {
  red: Vec<T>,
  green: Vec<T>,
  status: Status,
}

/// The status of a [`SwapVec`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Status {
  Red,
  Green,
}

impl<T> Default for SwapVec<T> {
  fn default() -> Self {
    Self {
      red: Vec::new(),
      green: Vec::new(),
      status: Status::Red,
    }
  }
}

impl<T> SwapVec<T> {
  /// Creates a new [`SwapVec`].
  pub fn new() -> Self {
    Self::default()
  }

  /// Creates a new [`SwapVec`] with the given capacity.
  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      red: Vec::with_capacity(capacity),
      green: Vec::with_capacity(capacity),
      status: Status::Red,
    }
  }

  /// Returns the length of the active [`Vec`].
  pub fn len(&self) -> usize {
    match self.status {
      Status::Red => self.red.len(),
      Status::Green => self.green.len(),
    }
  }

  /// Returns the capacity of the active [`Vec`].
  pub fn capacity(&self) -> usize {
    match self.status {
      Status::Red => self.red.capacity(),
      Status::Green => self.green.capacity(),
    }
  }

  /// Returns whether the active [`Vec`] is empty.
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Pushes a value into the active [`Vec`].
  pub fn push(&mut self, value: T) {
    match self.status {
      Status::Red => self.red.push(value),
      Status::Green => self.green.push(value),
    }
  }

  /// Pops a value from the active [`Vec`].
  pub fn pop(&mut self) -> Option<T> {
    match self.status {
      Status::Red => self.red.pop(),
      Status::Green => self.green.pop(),
    }
  }

  /// Drains the active [`Vec`].
  pub fn drain(&mut self, range: RangeFull) -> std::vec::Drain<T> {
    match self.status {
      Status::Red => self.red.drain(range),
      Status::Green => self.green.drain(range),
    }
  }

  /// Swaps the active [`Vec` with the inactive one.
  pub fn swap(&mut self) {
    self.status = match self.status {
      Status::Red => Status::Green,
      Status::Green => Status::Red,
    };
  }

  /// Clears the active [`Vec`].
  pub fn clear(&mut self) {
    match self.status {
      Status::Red => self.red.clear(),
      Status::Green => self.green.clear(),
    }
  }

  /// Clears both [`Vec`]s.
  pub fn clear_all(&mut self) {
    self.red.clear();
    self.green.clear();
  }

  /// Iterates over the active [`Vec`].
  pub fn iter(&self) -> std::slice::Iter<T> {
    match self.status {
      Status::Red => self.red.iter(),
      Status::Green => self.green.iter(),
    }
  }

  /// Mutably iterates over the active [`Vec`].
  pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
    match self.status {
      Status::Red => self.red.iter_mut(),
      Status::Green => self.green.iter_mut(),
    }
  }

  /// Returns a slice of the active [`Vec`].
  pub fn as_slice(&self) -> &[T] {
    match self.status {
      Status::Red => self.red.as_slice(),
      Status::Green => self.green.as_slice(),
    }
  }

  /// Returns a mutable slice of the active [`Vec`].
  pub fn as_mut_slice(&mut self) -> &mut [T] {
    match self.status {
      Status::Red => self.red.as_mut_slice(),
      Status::Green => self.green.as_mut_slice(),
    }
  }
}

impl<T> AsRef<[T]> for SwapVec<T> {
  #[inline]
  fn as_ref(&self) -> &[T] {
    self.as_slice()
  }
}

impl<T> AsMut<[T]> for SwapVec<T> {
  #[inline]
  fn as_mut(&mut self) -> &mut [T] {
    self.as_mut_slice()
  }
}

impl<'a, T> IntoIterator for &'a SwapVec<T> {
  type Item = &'a T;
  type IntoIter = std::slice::Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T> IntoIterator for &'a mut SwapVec<T> {
  type Item = &'a mut T;
  type IntoIter = std::slice::IterMut<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_swap_vec_operation() {
    let mut swap = SwapVec::new();

    swap.push(1);
    swap.push(2);
    swap.push(3);

    assert_eq!(swap.len(), 3);

    swap.swap();

    assert_eq!(swap.len(), 0);

    swap.push(4);
    swap.push(5);
    swap.push(6);

    assert_eq!(swap.len(), 3);
  }
}
