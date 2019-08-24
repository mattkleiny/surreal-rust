//! A ring buffer of elements.

/// A ring buffer of elements of type T.
#[derive(Debug)]
pub struct RingBuffer<T> {
  capacity: usize,
  occupied: usize,
  elements: Vec<Option<T>>,
}

impl<T: Copy> RingBuffer<T> {
  pub fn new(capacity: usize) -> Self {
    Self {
      capacity,
      occupied: 0,
      elements: vec![None; capacity],
    }
  }

  pub fn capacity(&self) -> usize { self.capacity }
  pub fn occupied(&self) -> usize { self.occupied }

  /// Appends an element to the buffer.
  pub fn append(&mut self, _element: T) {
    unimplemented!()
  }

  /// Clears the buffer of all elements.
  pub fn clear(&mut self) {
    unimplemented!()
  }
}

impl<'a, T> IntoIterator for &'a RingBuffer<T> {
  type Item = T;
  type IntoIter = RingBufferIterator<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    RingBufferIterator {
      buffer: self,
      index: 0,
    }
  }
}

/// An iterator for ring buffers.
pub struct RingBufferIterator<'a, T> {
  buffer: &'a RingBuffer<T>,
  index: usize,
}

impl<'a, T> Iterator for RingBufferIterator<'a, T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    unimplemented!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_append_elements() {
    let mut buffer = RingBuffer::<u32>::new(16);

    for i in 0..1000 {
      buffer.append(i);
    }

    buffer.clear();
  }
}