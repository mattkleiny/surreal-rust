//! A ring buffer of elements.

/// A ring buffer of elements of type T.
#[derive(Debug)]
pub struct RingBuffer<T> {
  occupied: usize,
  write_pos: usize,
  elements: Vec<Option<T>>,
}

impl<T: Clone> RingBuffer<T> {
  pub fn new(capacity: usize) -> Self {
    Self {
      occupied: 0,
      write_pos: 0,
      elements: vec![None; capacity],
    }
  }

  pub fn capacity(&self) -> usize { self.elements.len() }
  pub fn occupied(&self) -> usize { self.occupied }

  /// Appends an element to the buffer.
  pub fn append(&mut self, element: T) {
    self.elements[self.write_pos] = Some(element);
    self.write_pos += 1;

    if self.write_pos >= self.capacity() {
      self.write_pos = 0;
    }
    if self.occupied < self.capacity() {
      self.occupied += 1;
    }
  }

  /// Clears the buffer of all elements.
  pub fn clear(&mut self) {
    self.occupied = 0;
    self.write_pos = 0;

    for element in self.elements.iter_mut() {
      *element = None;
    }
  }
}

impl<'a, T: Clone> IntoIterator for &'a RingBuffer<T> {
  type Item = &'a T;
  type IntoIter = RingBufferIterator<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    RingBufferIterator {
      buffer: self,
      index: 0,
      touched: 0,
    }
  }
}

/// An iterator for ring buffers.
pub struct RingBufferIterator<'a, T> {
  buffer: &'a RingBuffer<T>,
  index: usize,
  touched: usize,
}

impl<'a, T: Clone> Iterator for RingBufferIterator<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    // iterate backwards, wrapping around the list
    if self.index <= 1 {
      self.index = self.buffer.capacity() - 1;
    } else {
      self.index -= 1;
    }

    // count the number of touched elements
    self.touched += 1;
    if self.touched < self.buffer.occupied() {
      self.buffer.elements[self.index].as_ref()
    } else {
      None
    }
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