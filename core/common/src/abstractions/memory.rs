//! Memory management tools.

use std::{
  alloc::{AllocError, Allocator, Layout},
  ptr::NonNull,
  sync::atomic::{AtomicUsize, Ordering},
};

/// A stack allocator that uses a bump allocation strategy.
///
/// This allocator is very simple and fast, but it does not support
/// de-allocation. It's useful for phased allocation, where you allocate a bunch
/// of objects at once, and then deallocate them all at once.
///
/// This type implements the [`Allocator`] trait from the standard library,
/// allowing it to be used with all std types that require an allocator, and
/// most collections in this crate also support `new_in` methods that allow you
/// to specify a custom allocator.
pub struct StackAllocator<const C: usize> {
  heap: [u8; C],
  pointer: AtomicUsize,
}

impl<const C: usize> Default for StackAllocator<C> {
  /// Creates a new stack allocator.
  fn default() -> Self {
    Self::new()
  }
}

impl<const C: usize> StackAllocator<C> {
  /// Creates a new stack allocator.
  pub fn new() -> Self {
    Self {
      heap: [0; C],
      pointer: AtomicUsize::new(0),
    }
  }
}

unsafe impl<const C: usize> Allocator for StackAllocator<C> {
  fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
    let size = layout.size();
    let align = layout.align();
    let pointer = self.pointer.load(Ordering::Relaxed);

    let offset = pointer % align;

    let aligned_pointer = pointer + offset;
    let new_pointer = aligned_pointer + size;

    if new_pointer > C {
      return Err(AllocError);
    }

    self.pointer.store(new_pointer, Ordering::Relaxed);

    Ok(NonNull::from(&self.heap[aligned_pointer..new_pointer]))
  }

  unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
    // no-op
  }
}
