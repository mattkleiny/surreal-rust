//! Memory allocation and support

/// A fixed-size memory allocation arena, for use in transient allocation scenarios.
pub struct MemoryArena<const S: usize> {
  memory: [u8; S],
  pointer: usize,
}

impl<const S: usize> Default for MemoryArena<S> {
  fn default() -> Self {
    Self {
      memory: [0; S],
      pointer: 0,
    }
  }
}

impl<const S: usize> MemoryArena<S> {
  /// Resets the memory arena, by simply resetting the pointer.
  pub fn reset(&mut self) {
    self.pointer = 0;
  }
}
