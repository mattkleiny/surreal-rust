//! Low-level memory management for Surreal.

use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::collections::LinkedList;
use std::marker::PhantomData;
use std::mem::size_of;

/// A raw pointer type; be careful as this is inherently unsafe.
type Memory = *mut u8;

/// The default alignment for allocated memory blocks.
/// This needs to be a power of 2.
const DEFAULT_ALIGNMENT: usize = 2;

/// Possible error types for this module.
#[derive(Debug)]
pub enum Error {
  OutOfMemory,
}

// Helpers for computing size and memory.
pub const fn bytes(value: usize) -> usize { value }
pub const fn kilobytes(value: usize) -> usize { bytes(value * 1024) }
pub const fn megabytes(value: usize) -> usize { kilobytes(value * 1024) }
pub const fn gigabytes(value: usize) -> usize { megabytes(value * 1024) }

/// Represents a heap of memory; a heap is a collection of memory blocks
/// that can be used to perform large and persistent allocations for long
/// running purposes.
pub struct MemoryHeap<A: Allocator> {
  size_per_block: usize,
  maximum_size: usize,
  used_size: usize,
  blocks: LinkedList<MemoryBlock<A>>,
}

impl<A: Allocator> MemoryHeap<A> {
  pub fn new(size_per_block: usize, maximum_size: usize) -> Self {
    let mut blocks = LinkedList::new();
    let initial_block = MemoryBlock::new(size_per_block);

    blocks.push_back(initial_block);

    Self {
      size_per_block,
      maximum_size,
      used_size: 0,
      blocks,
    }
  }

  pub fn capacity(&self) -> usize {
    self.maximum_size - self.used_size
  }

  pub fn allocate(&mut self, size: usize) -> Result<Memory, Error> {
    // make sure we're not yet at capacity
    let capacity = self.capacity();
    if capacity < size {
      return Err(Error::OutOfMemory);
    }

    // find the first available block
    for block in self.blocks.iter_mut().rev() {
      if block.capacity() >= size {
        self.used_size += size;
        return block.allocate(size);
      }
    }

    // insert a new block
    let mut block = MemoryBlock::new(self.size_per_block);
    let result = block.allocate(size);
    if result.is_ok() {
      self.blocks.push_back(block);
    }
    self.used_size += size;
    result
  }
}

/// Represents a single block in a memory heap; blocks are smaller than
/// the heap total to allow fragmented allocation patterns.
struct MemoryBlock<A: Allocator> {
  size: usize,
  used: usize,
  address: Memory,
  _allocator: PhantomData<A>,
}

impl<A: Allocator> MemoryBlock<A> {
  pub fn new(size: usize) -> Self {
    Self {
      size,
      used: 0,
      address: A::allocate(size),
      _allocator: PhantomData,
    }
  }

  pub fn capacity(&self) -> usize {
    self.size - self.used
  }

  pub fn allocate(&mut self, size: usize) -> Result<Memory, Error> {
    let capacity = self.capacity();

    if capacity < size {
      return Err(Error::OutOfMemory);
    }

    let result = unsafe {
      self.address.offset(self.used as isize)
    };

    self.used += size;

    Ok(result)
  }
}

impl<A: Allocator> Drop for MemoryBlock<A> {
  fn drop(&mut self) {
    A::release(self.address, self.size);
  }
}

/// Represents a component capable of allocating raw memory.
pub trait Allocator {
  fn allocate(size: usize) -> Memory;
  fn release(address: Memory, size: usize);
}

/// The portable system allocation from Rust itself.
pub struct PortableAllocator;

impl Allocator for PortableAllocator {
  fn allocate(size: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, DEFAULT_ALIGNMENT).unwrap();
    unsafe { alloc_zeroed(layout) }
  }

  fn release(address: *mut u8, size: usize) {
    let layout = Layout::from_size_align(size, DEFAULT_ALIGNMENT).unwrap();
    unsafe { dealloc(address, layout); }
  }
}

/// Permits in-place allocation of a type directly into a memory heap.
pub trait InPlaceAllocation<A: Allocator>: Sized {
  fn allocate(memory: &mut MemoryHeap<A>) -> &mut Self;
}

impl<T: Default, A: Allocator> InPlaceAllocation<A> for T {
  fn allocate(memory: &mut MemoryHeap<A>) -> &mut Self {
    unsafe {
      let memory = memory.allocate(size_of::<T>());
      let element = memory.unwrap() as *mut T;

      // initialize the element in-place with a default value
      *element = T::default();

      &mut *element
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /// A point in 3-space for testing.
  #[derive(Default)]
  struct Point(f32, f32, f32);

  #[test]
  fn heap_should_allocate_without_fault() {
    let mut heap = MemoryHeap::<PortableAllocator>::new(kilobytes(1), kilobytes(4));

    heap.allocate(kilobytes(1)).unwrap();
    heap.allocate(kilobytes(1)).unwrap();
    heap.allocate(kilobytes(1)).unwrap();
    heap.allocate(kilobytes(1)).unwrap();

    assert_eq!(4, heap.blocks.len());
  }

  #[test]
  fn block_allocate_without_fault() {
    let mut block = MemoryBlock::<PortableAllocator>::new(kilobytes(4));

    block.allocate(kilobytes(2)).unwrap();
    block.allocate(kilobytes(2)).unwrap();
  }

  #[test]
  fn heap_should_allow_in_place_allocations() {
    let mut heap = MemoryHeap::<PortableAllocator>::new(kilobytes(1), kilobytes(4));
    let point = Point::allocate(&mut heap);

    point.0 = 1.;
    point.1 = 2.;
    point.2 = 3.;
  }
}