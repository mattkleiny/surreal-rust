/// Represents a safe index into an [`Arena`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ArenaIndex {
  pub index: usize,
  pub generation: u16,
}

/// A single entry in an `Arena`.
#[derive(Debug)]
struct ArenaEntry<T> {
  value: T,
  generation: u16,
}

/// A simple generational arena of elements of type [`T`] .
///
/// An arena exposes safe externalized indices in the form of [`ArenaIndex`]es.
#[derive(Debug)]
pub struct Arena<T> {
  entries: Vec<Option<ArenaEntry<T>>>,
  current_generation: u16,
  is_generation_dirty: bool,
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
  pub fn get(&self, index: ArenaIndex) -> Option<&T> {
    // sanity check external index
    if index.index >= self.entries.len() {
      return None;
    }

    // if this entry exists and the generation matches
    if let Some(Some(entry)) = self.entries.get(index.index) {
      if entry.generation == index.generation {
        return Some(&entry.value);
      }
    }

    return None;
  }

  /// Returns a mutable reference to the item at the given index.
  pub fn get_mut(&mut self, index: ArenaIndex) -> Option<&mut T> {
    // sanity check external index
    if index.index >= self.entries.len() {
      return None;
    }

    // if this entry exists and the generation matches
    if let Some(Some(entry)) = self.entries.get_mut(index.index) {
      if entry.generation == index.generation {
        return Some(&mut entry.value);
      }
    }

    return None;
  }

  /// Adds an entry to the arena and returns it's index.
  pub fn add(&mut self, value: T) -> ArenaIndex {
    let index = self.allocate_index();

    self.entries[index.index] = Some(ArenaEntry {
      value,
      generation: index.generation,
    });

    index
  }

  /// Removes an item from the arena.
  pub fn remove(&mut self, ArenaIndex { index, generation }: ArenaIndex) -> bool {
    // sanity check external index
    if index >= self.entries.len() {
      return false;
    }

    // if this is the relevant entry and the generation matches
    if let Some(entry) = &self.entries[index] {
      if generation == entry.generation {
        self.entries[index] = None;
        self.is_generation_dirty = true;

        return true;
      }
    }

    return false;
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
      self.current_generation = self.current_generation + 1;
      self.is_generation_dirty = false;
    }

    // scan through existing entries and find an empty slot
    for i in 0..self.entries.len() {
      if let None = self.entries[i] {
        return ArenaIndex {
          index: i,
          generation: self.current_generation,
        };
      }
    }

    // otherwise allocate a new entry
    self.entries.push(None);

    ArenaIndex {
      index: self.entries.len() - 1,
      generation: self.current_generation,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn arena_should_add_item() {
    let mut arena = Arena::new();

    let index1 = arena.add("Item 1");
    let index2 = arena.add("Item 2");
    let index3 = arena.add("Item 3");

    assert_ne!(index1, index2);
    assert_ne!(index2, index3);
  }

  #[test]
  fn arena_should_remove_item() {
    let mut arena = Arena::new();

    let index1 = arena.add("Item 1");
    let index2 = arena.add("Item 2");

    arena.remove(index1);

    assert!(!arena.contains(index1));
    assert!(arena.contains(index2));
  }

  #[test]
  fn arena_should_access_item() {
    let mut arena = Arena::new();

    let index1 = arena.add("Item 1");
    let index2 = arena.add("Item 2");
    let index3 = ArenaIndex { index: 23, generation: 0 };

    assert!(arena.get(index1).is_some());
    assert!(arena.get_mut(index2).is_some());
    assert!(!arena.get(index3).is_some());
  }

  #[test]
  fn arena_should_reuse_old_spaces() {
    let mut arena = Arena::new();

    let _index1 = arena.add("Item 1");
    let index2 = arena.add("Item 2");
    let _index3 = arena.add("Item 3");

    arena.remove(index2);

    let index4 = arena.add("Item 4");

    assert_eq!(index2.index, index4.index);
    assert_ne!(index2.generation, index4.generation);
  }
}