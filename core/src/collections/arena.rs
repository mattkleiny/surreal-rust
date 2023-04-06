/// Represents a safe index into an [`Arena`].
///
/// This is a 64-bit integer that is split into two parts:
/// - The lower 32 bits are the index into the arena's internal storage.
/// - The upper 16 bits are the generation of the entry at that index.
///
/// The generation is incremented every time an entry is removed from the arena.
/// This allows us to detect when an index is no longer valid.
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
pub struct ArenaIndex {
  index: u32,
  generation: u16,
}

impl From<u64> for ArenaIndex {
  #[inline]
  fn from(packed: u64) -> Self {
    let generation = (packed >> 32) as u16;
    let index = packed as u32;

    ArenaIndex { index, generation }
  }
}

impl From<ArenaIndex> for u64 {
  #[inline]
  fn from(value: ArenaIndex) -> Self {
    (value.generation as u64) << 32 | value.index as u64
  }
}

/// A single entry in an `Arena`.
#[derive(Debug)]
struct ArenaEntry<T> {
  value: T,
  generation: u16,
}

/// A simple generational arena of elements of type [`T`] .
///
/// An arena exposes safe externalized indices in the form of [`ArenaIndex`]es
/// that can be passed around the application safely.
///
/// An arena is a contiguous block of memory that is used to store a collection
/// of elements. When an element is removed from the arena, the slot that it
/// occupied remains empty until the next insert.. This means that the order of
/// elements in the arena is not guaranteed to be the same as the order in which
/// they were inserted.
#[derive(Debug)]
pub struct Arena<T> {
  entries: Vec<Option<ArenaEntry<T>>>,
  current_generation: u16,
  is_generation_dirty: bool,
}

impl<T> Default for Arena<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> Arena<T> {
  /// Creates a new empty arena.
  pub fn new() -> Self {
    Self {
      entries: Vec::new(),
      current_generation: 1,
      is_generation_dirty: false,
    }
  }

  /// Creates a new empty arena with the given default capacity.
  pub fn with_capacity(size: usize) -> Self {
    Self {
      entries: Vec::with_capacity(size),
      current_generation: 1,
      is_generation_dirty: false,
    }
  }

  /// Is the arena empty?
  pub fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }

  /// Returns the number of elements in the arena.
  pub fn len(&self) -> usize {
    let mut count = 0;

    for entry in &self.entries {
      if entry.is_some() {
        count += 1;
      }
    }

    count
  }

  /// Determines if the arena contains the given index.
  pub fn contains(&self, index: ArenaIndex) -> bool {
    self.get(index).is_some()
  }

  /// Returns a reference to the item at the given index.
  pub fn get(&self, ArenaIndex { index, generation }: ArenaIndex) -> Option<&T> {
    // sanity check external index
    if index as usize >= self.entries.len() {
      return None;
    }

    // if this entry exists and the generation matches
    if let Some(Some(entry)) = self.entries.get(index as usize) {
      if entry.generation == generation {
        return Some(&entry.value);
      }
    }

    None
  }

  /// Returns a mutable reference to the item at the given index.
  pub fn get_mut(&mut self, ArenaIndex { index, generation }: ArenaIndex) -> Option<&mut T> {
    // sanity check external index
    if index as usize >= self.entries.len() {
      return None;
    }

    // if this entry exists and the generation matches
    if let Some(Some(entry)) = self.entries.get_mut(index as usize) {
      if entry.generation == generation {
        return Some(&mut entry.value);
      }
    }

    None
  }

  /// Inserts an entry to the arena and returns it's index.
  pub fn insert(&mut self, value: T) -> ArenaIndex {
    let index = self.allocate_index();

    self.entries[index.index as usize] = Some(ArenaEntry {
      value,
      generation: index.generation,
    });

    index
  }

  /// Removes an item from the arena.
  pub fn remove(&mut self, ArenaIndex { index, generation }: ArenaIndex) -> Option<T> {
    // sanity check external index
    if index as usize >= self.entries.len() {
      return None;
    }

    // if this is the relevant entry and the generation matches
    if let Some(entry) = &self.entries[index as usize] {
      if generation == entry.generation {
        let entry = self.entries[index as usize].take().unwrap();
        self.is_generation_dirty = true;

        return Some(entry.value);
      }
    }

    None
  }

  /// Clears all entries from the arena.
  pub fn clear(&mut self) {
    self.entries.clear();
    self.is_generation_dirty = true;
  }

  /// Allocates a new [`ArenaIndex`] for an item.
  fn allocate_index(&mut self) -> ArenaIndex {
    // increment the generation if necessary
    if self.is_generation_dirty {
      self.current_generation += 1;
      self.is_generation_dirty = false;
    }

    // scan through existing entries and find an empty slot
    for index in 0..self.entries.len() {
      if self.entries[index].is_none() {
        return ArenaIndex {
          index: index as u32,
          generation: self.current_generation,
        };
      }
    }

    // otherwise allocate a new entry
    self.entries.push(None);

    ArenaIndex {
      index: (self.entries.len() - 1) as u32,
      generation: self.current_generation,
    }
  }

  /// Iterates over the arena.
  pub fn iter(&self) -> impl Iterator<Item = (ArenaIndex, &T)> {
    pub struct Iter<'a, T> {
      arena: &'a Arena<T>,
      index: usize,
    }

    impl<'a, T> Iterator for Iter<'a, T> {
      type Item = (ArenaIndex, &'a T);

      fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.arena.entries.get(self.index) {
          if let Some(value) = entry {
            let arena_index = ArenaIndex {
              index: self.index as u32,
              generation: value.generation,
            };

            self.index += 1;

            return Some((arena_index, &value.value));
          }

          self.index += 1;
        }

        None
      }

      fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.arena.entries.len() - self.index;
        (remaining, Some(remaining))
      }
    }

    Iter {
      arena: self,
      index: 0,
    }
  }

  /// Mutably iterates over the arena.
  pub fn iter_mut(&mut self) -> impl Iterator<Item = (ArenaIndex, &mut T)> {
    pub struct IterMut<'a, T> {
      arena: &'a mut Arena<T>,
      index: usize,
    }

    impl<'a, T> Iterator for IterMut<'a, T> {
      type Item = (ArenaIndex, &'a mut T);

      fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.arena.entries.get_mut(self.index) {
          if let Some(value) = entry.as_mut() {
            let arena_index = ArenaIndex {
              index: self.index as u32,
              generation: value.generation,
            };

            self.index += 1;

            // elide the lifetime; rust has trouble with the borrow checker
            let value = crate::utilities::unsafe_mutable_alias(value);

            return Some((arena_index, &mut value.value));
          }

          self.index += 1;
        }

        None
      }

      fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.arena.entries.len() - self.index;
        (remaining, Some(remaining))
      }
    }

    IterMut {
      arena: self,
      index: 0,
    }
  }
}

impl<'a, T> IntoIterator for &'a Arena<T> {
  type Item = (ArenaIndex, &'a T);
  type IntoIter = impl Iterator<Item = Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T> IntoIterator for &'a mut Arena<T> {
  type Item = (ArenaIndex, &'a mut T);
  type IntoIter = impl Iterator<Item = Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

impl<A> FromIterator<A> for Arena<A> {
  fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
    let mut result = Self::default();

    for item in iter {
      result.insert(item);
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn arena_should_add_item() {
    let mut arena = Arena::new();

    let index1 = arena.insert("Item 1");
    let index2 = arena.insert("Item 2");
    let index3 = arena.insert("Item 3");

    assert_ne!(index1, index2);
    assert_ne!(index2, index3);
  }

  #[test]
  fn arena_should_remove_item() {
    let mut arena = Arena::new();

    let index1 = arena.insert("Item 1");
    let index2 = arena.insert("Item 2");

    arena.remove(index1);

    assert!(!arena.contains(index1));
    assert!(arena.contains(index2));
  }

  #[test]
  fn arena_should_access_item() {
    let mut arena = Arena::new();

    let index1 = arena.insert("Item 1");
    let index2 = arena.insert("Item 2");

    let index3 = ArenaIndex {
      index: 23,
      generation: 0,
    };

    assert!(arena.get(index1).is_some());
    assert!(arena.get_mut(index2).is_some());
    assert!(arena.get(index3).is_none());
  }

  #[test]
  fn arena_should_reuse_old_spaces() {
    let mut arena = Arena::new();

    let _index1 = arena.insert("Item 1");
    let index2 = arena.insert("Item 2");
    let _index3 = arena.insert("Item 3");

    arena.remove(index2);

    let index4 = arena.insert("Item 4");

    assert_eq!(index2.index, index4.index);
    assert_ne!(index2.generation, index4.generation);
  }

  #[test]
  fn arena_should_iterate() {
    let mut arena = Arena::new();

    arena.insert("Item 1");
    let index2 = arena.insert("Item 2");
    arena.insert("Item 3");
    arena.insert("Item 4");

    arena.remove(index2);

    for (index, item) in &arena {
      println!("{item} at {index:?}");
    }
  }

  #[test]
  fn arena_should_iterate_mutably() {
    let mut arena = Arena::new();

    arena.insert("Item 1");
    let index2 = arena.insert("Item 2");
    arena.insert("Item 3");
    arena.insert("Item 4");

    arena.remove(index2);

    for (index, item) in &mut arena {
      *item = "Test 1";

      println!("{item} at {index:?}");
    }
  }

  #[test]
  fn arena_index_should_pack_and_unpack_from_u64() {
    let index = ArenaIndex {
      index: 10,
      generation: 3,
    };
    let packed: u64 = index.into();
    let unpacked = ArenaIndex::from(packed);

    assert_eq!(index, unpacked);
  }

  #[test]
  fn arena_should_collect_from_iterator() {
    let items = (0..16).collect::<Arena<_>>();

    assert_eq!(items.len(), 16);
  }
}
