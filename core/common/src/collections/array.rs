use std::{
  alloc::{Allocator, Layout},
  ptr::NonNull,
};

/// A mutable array that is backed by a dynamic [`Allocator`].
///
/// This type is similar to built in [`Vec`], although with less functionality.
///
/// N.B: When using an array with a bump or stack allocations strategy, it's
/// important to size the array correctly, as it does not support de-allocation
/// and so subsequent resizes of the array will result in a non-recoverable
/// allocation.
pub struct Array<'a, T> {
  data: NonNull<T>,
  len: usize,
  capacity: usize,
  allocator: &'a dyn Allocator,
}

impl<T> Default for Array<'static, T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> Array<'static, T> {
  /// Creates a new array with the default global allocator.
  pub fn new() -> Self {
    Self::new_in(&std::alloc::Global as &'static dyn Allocator)
  }

  /// Creates a new array with the given capacity in the default global
  /// allocator.
  pub fn with_capacity(capacity: usize) -> Self {
    Self::with_capacity_in(&std::alloc::Global as &'static dyn Allocator, capacity)
  }
}

impl<'a, T> Array<'a, T> {
  /// Creates a new array with the given allocator.
  pub fn new_in(allocator: &'a dyn Allocator) -> Self {
    Self {
      data: NonNull::dangling(),
      len: 0,
      capacity: 0,
      allocator,
    }
  }

  /// Creates a new array with the given capacity in the given allocator.
  pub fn with_capacity_in(allocator: &'a dyn Allocator, capacity: usize) -> Self {
    let layout = Layout::array::<T>(capacity).unwrap();
    let data = allocator.allocate(layout).unwrap().as_ptr() as *mut T;

    Self {
      data: NonNull::new(data).unwrap(),
      len: 0,
      capacity,
      allocator,
    }
  }

  /// Returns true if the array is empty.
  pub fn is_empty(&self) -> bool {
    self.len == 0
  }

  /// Returns the number of elements in the array.
  pub fn len(&self) -> usize {
    self.len
  }

  /// Pushes a value onto the end of the array.
  pub fn push(&mut self, value: T) {
    if self.len == self.capacity {
      self.grow();
    }

    unsafe {
      self.data.as_ptr().add(self.len).write(value);
      self.len += 1;
    }
  }

  /// Pops a value from the end of the array.
  pub fn pop(&mut self) -> Option<T> {
    if self.len == 0 {
      return None;
    }

    self.len -= 1;

    unsafe { Some(self.data.as_ptr().add(self.len).read()) }
  }

  /// Clears the array, removing all elements.
  pub fn clear(&mut self) {
    self.len = 0;
  }

  /// Returns an iterator over the elements of the array.
  pub fn iter(&self) -> impl Iterator<Item = &T> {
    unsafe { std::slice::from_raw_parts(self.data.as_ptr(), self.len).iter() }
  }

  /// Returns a mutable iterator over the elements of the array.
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
    unsafe { std::slice::from_raw_parts_mut(self.data.as_ptr(), self.len).iter_mut() }
  }

  /// Grows the size of the array.
  fn grow(&mut self) {
    // calculate the new capacity
    let new_capacity = if self.capacity == 0 { 1 } else { self.capacity * 2 };
    let new_layout = Layout::array::<T>(new_capacity).unwrap();

    // allocate the new memory
    let new_data = self.allocator.allocate(new_layout).unwrap().cast();

    unsafe {
      // copy the old data into the new memory
      std::ptr::copy_nonoverlapping(self.data.as_ptr(), new_data.as_ptr(), self.len);

      // reclaim the old memory, if possible
      let old_layout = Layout::array::<T>(self.capacity).unwrap();
      self.allocator.deallocate(self.data.cast(), old_layout);
    }

    // update the array fields
    self.data = new_data.cast();
    self.capacity = new_capacity;
  }
}

impl<'a, T> Drop for Array<'a, T> {
  fn drop(&mut self) {
    let layout = Layout::array::<T>(self.capacity).unwrap();

    unsafe {
      self.allocator.deallocate(self.data.cast(), layout);
    }
  }
}

impl<'a, 'b, T> IntoIterator for &'b Array<'a, T> {
  type Item = &'b T;
  type IntoIter = impl Iterator<Item = &'b T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, 'b, T> IntoIterator for &'b mut Array<'a, T> {
  type Item = &'b mut T;
  type IntoIter = impl Iterator<Item = &'b mut T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_array_global_allocation_and_free() {
    let mut array = Array::new();

    assert!(array.is_empty());

    array.push(1);
    array.push(2);
    array.push(3);

    assert!(!array.is_empty());
    assert_eq!(array.len(), 3);

    let mut iter = array.iter();

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn test_array_stack_allocation_and_free() {
    let allocator = crate::StackAllocator::<64>::new();
    let mut array = Array::new_in(&allocator);

    array.push(1);
    array.push(2);
    array.push(3);

    assert!(!array.is_empty());
    assert_eq!(array.len(), 3);

    let mut iter = array.iter();

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
  }
}
