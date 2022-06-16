//! A simple memory arena.

use core::panic;
use std::{alloc::Layout, ptr::NonNull};

// TODO: implement the allocator API here

/// A simple fixed-size memory arena that can be used for transient allocations.
pub struct MemoryArena<const SIZE: usize> {
  memory: [u8; SIZE],
  pointer: usize,
}

impl<const SIZE: usize> Default for MemoryArena<SIZE> {
  fn default() -> Self {
    Self {
      memory: [0; SIZE],
      pointer: 0,
    }
  }
}

impl<const SIZE: usize> MemoryArena<SIZE> {
  /// Attempts to allocate a block of memory.
  pub unsafe fn allocate(&mut self, layout: Layout) -> crate::Result<NonNull<u8>> {
    self.pointer += layout.size();

    if self.pointer > SIZE {
      anyhow::bail!("Out of memory!")
    }

    Ok(NonNull::new_unchecked(
      self.memory.get_unchecked_mut(self.pointer - layout.size()),
    ))
  }

  /// Deallocates the memory referenced by `ptr`.
  pub unsafe fn deallocate(&mut self, _pointer: NonNull<u8>, _layout: Layout) {
    panic!("De-allocation is not supported by this memory arena")
  }
}
