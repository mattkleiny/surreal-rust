//! Memory allocation and support

/// A reasonable default memory arena size.
const DEFAULT_ARENA_SIZE: usize = 4 * 1024;

/// A fixed-size memory allocation arena, for use in transient allocation scenarios.
pub struct FixedMemoryArena<const S: usize = DEFAULT_ARENA_SIZE> {
  memory: [u8; S],
  pointer: usize,
}

impl<const S: usize> Default for FixedMemoryArena<S> {
  fn default() -> Self {
    Self {
      memory: [0; S],
      pointer: 0,
    }
  }
}

impl<const S: usize> FixedMemoryArena<S> {
  /// Resets the memory arena, by simply resetting the pointer.
  pub fn reset(&mut self) {
    self.pointer = 0;
  }
}
