/// A lightweight, fast and append-only ring buffer of elements of type T.
///
/// It's intended to be used for small windowed set operations, like time sampling or frequency analysis.
#[derive(Debug)]
pub struct RingBuffer<T> {
  cursor: usize,
  elements: Vec<Option<T>>,
}

impl<T> RingBuffer<T> {
  /// Creates a new ring buffer with the given capacity.
  pub fn new(capacity: usize) -> Self where T :Clone {
    Self {
      cursor: 0,
      elements: vec![None; capacity],
    }
  }

  /// Returns the maximum number of elements in the buffer.
  pub fn capacity(&self) -> usize {
    self.elements.capacity()
  }

  /// Returns the current number of elements in the buffer.
  pub fn occupied(&self) -> usize {
    self.elements.len()
  }

  /// Appends an element to the buffer.
  pub fn append(&mut self, element: T) {
    self.elements[self.cursor] = Some(element);
    self.cursor += 1;

    if self.cursor >= self.capacity() {
      self.cursor = 0;
    }
  }

  /// Clears the buffer of all elements.
  pub fn clear(&mut self) {
    self.cursor = 0;
    self.elements.clear();
  }

  /// Permits iterating over the ring buffer.
  pub fn iter(&self) -> RingBufferIterator<T> {
    RingBufferIterator {
      buffer: self,
      index: self.cursor,
      touched: 0,
    }
  }
}

/// Allows iterating over the ring buffer.
impl<'a, T> IntoIterator for &'a RingBuffer<T> {
  type Item = &'a T;
  type IntoIter = RingBufferIterator<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

/// An iterator for the ring buffer. This is a forward-only iterator,
/// and does not support in-place mutation.
pub struct RingBufferIterator<'a, T> {
  buffer: &'a RingBuffer<T>,
  index: usize,
  touched: usize,
}

impl<'a, T> Iterator for RingBufferIterator<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    // wrap around walking backwards
    if self.index == 0 {
      self.index = self.buffer.occupied() - 1;
    } else {
      self.index -= 1;
    }

    if self.touched < self.buffer.occupied() {
      self.touched += 1;

      match &self.buffer.elements[self.index] {
        Some(item) => Some(item),
        None => None,
      }
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ringbuffer_should_append_elements() {
    let mut buffer = RingBuffer::new(16);

    for i in 0..1000 {
      buffer.append(i);
    }

    assert_eq!(buffer.occupied(), 16);
  }

  #[test]
  fn ringbuffer_should_clear_elements() {
    let mut buffer = RingBuffer::new(16);

    for i in 0..1000 {
      buffer.append(i);
    }

    buffer.clear();

    assert_eq!(buffer.occupied(), 0);
  }

  #[test]
  fn ringbuffer_should_iterate_backwards() {
    let mut buffer: RingBuffer<u32> = RingBuffer::new(16);

    buffer.append(1);
    buffer.append(2);
    buffer.append(3);
    buffer.append(4);

    let results: Vec<&u32> = buffer.iter().collect();

    assert_eq!(*results[0], 4);
    assert_eq!(*results[1], 3);
    assert_eq!(*results[2], 2);
    assert_eq!(*results[3], 1);
  }
}
