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

/// A generational arena that allows flat list-like structures with safe externalised indices.
///
/// The core element of the arena is an index. The index is composed of two parts, a raw array
/// index and a generation. We use the generation to determine whether or not a space in the array
/// had previously been occupied by a different element, side-stepping the ABA problem.
///
/// This allows us to retain efficient storage in a flat array, good iteration performance, and a
/// safe way to circulate the index throughout the application.
#[derive(Clone, Debug)]
pub struct Arena<T> {
  items: Vec<ArenaEntry<T>>,
  generation: u64,
  length: usize,
  next_free: Option<usize>,
}

impl<T> Arena<T> {
  pub fn new() -> Self {
    let mut arena = Self {
      items: Vec::new(),
      generation: 0,
      length: 0,
      next_free: None,
    };

    arena.reserve(16);
    arena
  }

  /// Returns the length of the arena.
  #[inline]
  pub fn len(&self) -> usize {
    self.length
  }

  /// Inserts an element into the arena, returning it's index.
  pub fn insert(&mut self, element: T) -> ArenaIndex {
    match self.try_insert(element) {
      Ok(index) => index,
      Err(element) => {
        // double the size of the list and try again
        self.reserve(self.items.len());
        self.try_insert(element).map_err(|_| ()).expect("Failed to insert item!")
      }
    }
  }

  fn try_insert(&mut self, element: T) -> Result<ArenaIndex, T> {
    match self.next_free {
      Some(index) => match self.items[index] {
        ArenaEntry::Occupied { .. } => panic!("Corrupt free list!"),
        ArenaEntry::Free { next_free } => {
          self.next_free = next_free;
          self.length += 1;

          self.items[index] = ArenaEntry::Occupied {
            generation: self.generation,
            value: element,
          };

          Ok(ArenaIndex {
            index,
            generation: self.generation,
          })
        }
      },
      None => Err(element),
    }
  }

  /// Removes an existing element from the arena.
  pub fn remove(&mut self, index: ArenaIndex) -> Option<T> {
    if index.index > self.items.len() {
      return None;
    }

    let entry = std::mem::replace(
      &mut self.items[index.index],
      ArenaEntry::Free {
        next_free: self.next_free
      },
    );

    match entry {
      ArenaEntry::Occupied { generation, value } => {
        if generation == index.generation {
          self.generation += 1;
          self.next_free = Some(index.index);
          self.length -= 1;

          Some(value)
        } else {
          self.items[index.index] = ArenaEntry::Occupied {
            generation,
            value,
          };

          None
        }
      }
      entry @ ArenaEntry::Free { .. } => {
        self.items[index.index] = entry;
        None
      }
    }
  }

  /// Determines if the arena contains the given index.
  pub fn contains(&self, index: ArenaIndex) -> bool {
    self.get(index).is_some()
  }

  /// Retrieves an existing element from the arena.
  pub fn get(&self, index: ArenaIndex) -> Option<&T> {
    match self.items.get(index.index) {
      Some(ArenaEntry::Occupied { generation, ref value, }) if *generation == index.generation => Some(value),
      _ => None
    }
  }

  /// Retrieves mutably an existing element from the arena.
  pub fn get_mut(&mut self, index: ArenaIndex) -> Option<&mut T> {
    match self.items.get_mut(index.index) {
      Some(ArenaEntry::Occupied { generation, ref mut value, }) if *generation == index.generation => Some(value),
      _ => None
    }
  }

  /// Reserves the given additional capacity within the arena.
  pub fn reserve(&mut self, additional_capacity: usize) {
    let start = self.items.len();
    let end = self.items.len() + additional_capacity;

    let old_head = self.next_free;

    self.items.reserve_exact(additional_capacity);
    self.items.extend((start..end).map(|i| {
      if i == end - 1 {
        ArenaEntry::Free {
          next_free: old_head,
        }
      } else {
        ArenaEntry::Free {
          next_free: Some(i + 1),
        }
      }
    }));

    self.next_free = Some(start);
  }

  /// Clears the contents of the arena.
  ///
  /// This is safe, and retains the generation so that old indices won't access new elements.
  pub fn clear(&mut self) {
    self.length = 0;
    self.next_free = None;

    let capacity = self.items.capacity();

    // reset the entries
    for (index, item) in self.items.iter_mut().enumerate() {
      if index == capacity - 1 {
        *item = ArenaEntry::Free {
          next_free: None
        };
      } else {
        *item = ArenaEntry::Free {
          next_free: Some(index + 1),
        }
      }
    }
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

    arena.remove(index2);

    let index4 = arena.insert("Test 4");

    assert_ne!(index2, index4);
    assert!(arena.get(index2).is_none());
    assert_eq!(*arena.get(index4).unwrap(), "Test 4");

    arena.clear();
  }
}