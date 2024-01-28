use crate::unsafe_mutable_alias;

/// Represents a safe index into an [`Arena`].
///
/// This is a 64-bit integer that is split into two parts:
/// - The lower 32 bits are the index into the arena's internal storage.
/// - The upper 16 bits are the generation of the entry at that index.
///
/// The generation is incremented every time an entry is removed from the arena.
/// This allows us to detect when an index is no longer valid.
pub trait ArenaIndex {
  /// Creates a new [`ArenaIndex`] from an ordinal and generation.
  fn from_parts(ordinal: u32, generation: u32) -> Self;

  /// Gets the generation of this index.
  fn generation(&self) -> u32;

  /// Gets the ordinal of this index.
  fn ordinal(&self) -> u32;
}

/// Creates a new, opaque arena index type.
///
/// The type is implicitly convertible to and from [`u64`], [`u32`], and
/// [`ArenaIndex`], and can be used as a key in the [`Arena`] structure.
#[macro_export]
macro_rules! impl_arena_index {
  ($name:ident, $comment:literal) => {
    #[doc = $comment]
    #[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct $name {
      ordinal: u32,
      generation: u32,
    }

    impl $name {
      /// A sentinel value representing an empty index.
      pub const NONE: Self = Self {
        ordinal: 0,
        generation: 0,
      };
    }

    impl From<u32> for $name {
      #[inline]
      fn from(id: u32) -> Self {
        Self {
          ordinal: id,
          generation: 0,
        }
      }
    }

    impl From<$name> for u32 {
      #[inline(always)]
      fn from(value: $name) -> Self {
        value.ordinal
      }
    }

    impl From<u64> for $name {
      #[inline(always)]
      fn from(packed: u64) -> Self {
        let generation = (packed >> 32) as u32;
        let ordinal = packed as u32;

        Self { ordinal, generation }
      }
    }

    impl From<$name> for u64 {
      #[inline(always)]
      fn from(value: $name) -> Self {
        (value.generation as u64) << 32 | value.ordinal as u64
      }
    }

    impl $crate::ArenaIndex for $name {
      #[inline(always)]
      fn from_parts(ordinal: u32, generation: u32) -> Self {
        Self { ordinal, generation }
      }

      #[inline(always)]
      fn generation(&self) -> u32 {
        self.generation
      }

      #[inline(always)]
      fn ordinal(&self) -> u32 {
        self.ordinal
      }
    }

    impl $crate::FromRandom for $name {
      #[inline]
      fn from_random(random: &mut $crate::Random) -> Self {
        Self::from(random.next_u64())
      }
    }
  };
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
pub struct Arena<K, V> {
  entries: Vec<Option<ArenaEntry<V>>>,
  current_generation: u32,
  is_generation_dirty: bool,
  _key: std::marker::PhantomData<K>,
}

/// A single entry in an `Arena`.
#[derive(Debug)]
struct ArenaEntry<V> {
  value: V,
  generation: u32,
}

impl<K: ArenaIndex, V> Default for Arena<K, V> {
  fn default() -> Self {
    Self::new()
  }
}

impl<K: ArenaIndex, V> Arena<K, V> {
  /// Creates a new empty arena.
  pub fn new() -> Self {
    Self {
      entries: Vec::new(),
      current_generation: 1,
      is_generation_dirty: false,
      _key: std::marker::PhantomData,
    }
  }

  /// Creates a new empty arena with the given default capacity.
  pub fn with_capacity(size: usize) -> Self {
    Self {
      entries: Vec::with_capacity(size),
      current_generation: 1,
      is_generation_dirty: false,
      _key: std::marker::PhantomData,
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
  pub fn contains(&self, key: K) -> bool {
    self.get(key).is_some()
  }

  /// Returns a reference to the item at the given index.
  pub fn get(&self, key: K) -> Option<&V> {
    let ordinal = key.ordinal();
    let generation = key.generation();

    // sanity check external index
    if ordinal as usize >= self.entries.len() {
      return None;
    }

    // if this entry exists and the generation matches
    if let Some(Some(entry)) = self.entries.get(ordinal as usize) {
      if entry.generation == generation {
        return Some(&entry.value);
      }
    }

    None
  }

  /// Returns a mutable reference to the item at the given index.
  pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
    let ordinal = key.ordinal();
    let generation = key.generation();

    // sanity check external index
    if ordinal as usize >= self.entries.len() {
      return None;
    }

    // if this entry exists and the generation matches
    if let Some(Some(entry)) = self.entries.get_mut(ordinal as usize) {
      if entry.generation == generation {
        return Some(&mut entry.value);
      }
    }

    None
  }

  /// Inserts an entry to the arena and returns it's index.
  pub fn insert(&mut self, value: V) -> K {
    let key = self.allocate_key();

    let ordinal = key.ordinal();
    let generation = key.generation();

    self.entries[ordinal as usize] = Some(ArenaEntry {
      value,
      generation: generation,
    });

    key
  }

  /// Removes an item from the arena.
  pub fn remove(&mut self, key: K) -> Option<V> {
    let ordinal = key.ordinal();
    let generation = key.generation();

    // sanity check external index
    if ordinal as usize >= self.entries.len() {
      return None;
    }

    // if this is the relevant entry and the generation matches
    if let Some(entry) = &self.entries[ordinal as usize] {
      if generation == entry.generation {
        let entry = self.entries[ordinal as usize].take().unwrap();
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

  /// Iterates over the arena.
  pub fn iter(&self) -> impl Iterator<Item = &V> {
    pub struct Iter<'a, K, V> {
      arena: &'a Arena<K, V>,
      index: usize,
    }

    impl<'a, K: ArenaIndex, V> Iterator for Iter<'a, K, V> {
      type Item = &'a V;

      fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.arena.entries.get(self.index) {
          if let Some(value) = entry {
            self.index += 1;

            return Some(&value.value);
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

    Iter { arena: self, index: 0 }
  }

  /// Mutably iterates over the arena.
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut V> {
    pub struct IterMut<'a, K, V> {
      arena: &'a mut Arena<K, V>,
      index: usize,
    }

    impl<'a, K: ArenaIndex, V> Iterator for IterMut<'a, K, V> {
      type Item = &'a mut V;

      fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.arena.entries.get_mut(self.index) {
          if let Some(value) = entry.as_mut() {
            self.index += 1;

            let value = unsafe { unsafe_mutable_alias(value) }; // elide the lifetime

            return Some(&mut value.value);
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

    IterMut { arena: self, index: 0 }
  }

  /// Enumerates the indices and contents of the arena.
  pub fn enumerate(&self) -> impl Iterator<Item = (K, &V)> {
    pub struct Enumerate<'a, K, V> {
      arena: &'a Arena<K, V>,
      index: usize,
    }

    impl<'a, K: ArenaIndex, V> Iterator for Enumerate<'a, K, V> {
      type Item = (K, &'a V);

      fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.arena.entries.get(self.index) {
          if let Some(value) = entry {
            let key = K::from_parts(self.index as u32, value.generation);

            self.index += 1;

            return Some((key, &value.value));
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

    Enumerate { arena: self, index: 0 }
  }

  /// Mutably enumerates the indices and contents of the arena.
  pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (K, &mut V)> {
    pub struct EnumerateMut<'a, K, V> {
      arena: &'a mut Arena<K, V>,
      index: usize,
    }

    impl<'a, K: ArenaIndex, V> Iterator for EnumerateMut<'a, K, V> {
      type Item = (K, &'a mut V);

      fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.arena.entries.get_mut(self.index) {
          if let Some(value) = entry.as_mut() {
            let key = K::from_parts(self.index as u32, value.generation);
            let value = unsafe { unsafe_mutable_alias(value) }; // elide the lifetime

            self.index += 1;

            return Some((key, &mut value.value));
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

    EnumerateMut { arena: self, index: 0 }
  }

  /// Allocates a new [`K`] for an item.
  fn allocate_key(&mut self) -> K {
    // increment the generation if necessary
    if self.is_generation_dirty {
      self.current_generation += 1;
      self.is_generation_dirty = false;
    }

    // scan through existing entries and find an empty slot
    for index in 0..self.entries.len() {
      if self.entries[index].is_none() {
        return K::from_parts(index as u32, self.current_generation);
      }
    }

    // otherwise allocate a new entry
    self.entries.push(None);

    K::from_parts((self.entries.len() - 1) as u32, self.current_generation)
  }
}

/// Iterates over the arena.
impl<'a, K: ArenaIndex, V> IntoIterator for &'a Arena<K, V> {
  type Item = &'a V;
  type IntoIter = impl Iterator<Item = Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

/// Mutably iterates over the arena.
impl<'a, K: ArenaIndex, V> IntoIterator for &'a mut Arena<K, V> {
  type Item = &'a mut V;
  type IntoIter = impl Iterator<Item = Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

/// Allows an arena to be created from an iterator.
impl<K: ArenaIndex, V> FromIterator<V> for Arena<K, V> {
  fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
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

  impl_arena_index!(TestId, "A simple test id");

  #[test]
  fn test_add_item() {
    let mut arena = Arena::<TestId, &'static str>::new();

    let index1 = arena.insert("Item 1");
    let index2 = arena.insert("Item 2");
    let index3 = arena.insert("Item 3");

    assert_ne!(index1, index2);
    assert_ne!(index2, index3);
  }

  #[test]
  fn test_remove_item() {
    let mut arena = Arena::<TestId, &'static str>::new();

    let index1 = arena.insert("Item 1");
    let index2 = arena.insert("Item 2");

    arena.remove(index1);

    assert!(!arena.contains(index1));
    assert!(arena.contains(index2));
  }

  #[test]
  fn test_access_item() {
    let mut arena = Arena::<TestId, &'static str>::new();

    let index1 = arena.insert("Item 1");
    let index2 = arena.insert("Item 2");

    let index3 = TestId {
      ordinal: 23,
      generation: 0,
    };

    assert!(arena.get(index1).is_some());
    assert!(arena.get_mut(index2).is_some());
    assert!(arena.get(index3).is_none());
  }

  #[test]
  fn test_reuse_old_spaces() {
    let mut arena = Arena::<TestId, &'static str>::new();

    let _index1 = arena.insert("Item 1");
    let index2 = arena.insert("Item 2");
    let _index3 = arena.insert("Item 3");

    arena.remove(index2);

    let index4 = arena.insert("Item 4");

    assert_eq!(index2.ordinal, index4.ordinal);
    assert_ne!(index2.generation, index4.generation);
  }

  #[test]
  fn test_iterate() {
    let mut arena = Arena::<TestId, &'static str>::new();

    arena.insert("Item 1");
    let index2 = arena.insert("Item 2");
    arena.insert("Item 3");
    arena.insert("Item 4");

    arena.remove(index2);

    for item in &arena {
      println!("{item}");
    }
  }

  #[test]
  fn test_enumerate_mut() {
    let mut arena = Arena::<TestId, &'static str>::new();

    arena.insert("Item 1");
    let index2 = arena.insert("Item 2");
    arena.insert("Item 3");
    arena.insert("Item 4");

    arena.remove(index2);

    for (index, item) in arena.enumerate_mut() {
      *item = "Test 1";

      println!("{item} at {index:?}");
    }
  }

  #[test]
  fn test_should_pack_and_unpack_from_u64() {
    let index = TestId {
      ordinal: 10,
      generation: 3,
    };

    let packed: u64 = index.into();
    let unpacked = TestId::from(packed);

    assert_eq!(index, unpacked);
  }
}
