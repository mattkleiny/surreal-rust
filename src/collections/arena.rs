//! A generational arena that allows flat list-like structures with safe externalised indices.
//!
//! The core element of the arena is an index. The index is composed of two parts, a raw array
//! index and a generation. We use the generation to determine whether or not a space in the array
//! had previously been occupied by a different element, side-stepping the ABA problem.
//!
//! This allows us to retain efficient storage in a flat array, good iteration performance, and a
//! safe way to circulate the index throughout the application.

/// Represents an index into a generational arena.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArenaIndex {
  index: usize,
  generation: u64,
}

impl Into<usize> for ArenaIndex {
  fn into(self) -> usize {
    self.index
  }
}

/// Represents an entry into an arena.
#[derive(Clone, Debug)]
enum ArenaEntry<T> {
  /// This slot in the list is free and can contain a new element.
  Free {
    next_free: Option<usize>
  },
  /// This slot is occupied and already contains an element.
  Occupied {
    generation: u64,
    value: T,
  },
}

/// A generational arena backed by a vec of elements.
///
/// The arena will grow as required to accommodate new elements.
#[derive(Clone, Debug)]
pub struct Arena<T> {
  items: Vec<ArenaEntry<T>>,
  generation: u64,
  length: usize,
  next_free: Option<usize>,
}

impl<T> Arena<T> {
  pub fn new() -> Self {
    Self {
      items: Vec::new(),
      generation: 0,
      length: 0,
      next_free: None,
    }
  }

  /// Returns the length of the arena.
  #[inline]
  pub fn len(&self) -> usize {
    self.length
  }

  /// Inserts an element into the arena, returning it's index.
  pub fn insert(&mut self, element: T) -> ArenaIndex {
    unimplemented!()
  }

  /// Removes an existing element from the arena.
  pub fn remove(&mut self, index: ArenaIndex) {
    unimplemented!()
  }

  /// Determines if the arena contains the given index.
  pub fn contains(&self, index: ArenaIndex) -> bool {
    unimplemented!()
  }

  /// Retrieves an existing element from the arena.
  pub fn get(&self, index: ArenaIndex) -> Option<&T> {
    unimplemented!()
  }

  /// Retrieves mutably an existing element from the arena.
  pub fn get_mut(&mut self, index: ArenaIndex) -> Option<&mut T> {
    unimplemented!()
  }

  /// Clears the contents of the arena.
  ///
  /// This is safe, and retains the generation so that old indices won't access new elements.
  pub fn clear(&mut self) {
    self.items.clear();
    self.length = 0;
    self.next_free = None;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn arena_should_read_and_write() {
    let mut arena = Arena::new();

    let index1 = arena.insert("Test 1");
    let index2 = arena.insert("Test 2");
    let index3 = arena.insert("Test 3");

    assert_ne!(index1, index2);
    assert_ne!(index2, index3);

    arena.clear();
  }
}