use std::{collections::HashMap, hash::Hash};

/// A simple [`HashMap`]  with multiple values per key.
#[derive(Default, Debug)]
pub struct MultiMap<K, V> {
  entries: HashMap<K, Vec<V>>,
}

impl<K: Eq + Hash, V> MultiMap<K, V> {
  /// Creates a new multi-map.
  pub fn new() -> Self {
    Self {
      entries: HashMap::new(),
    }
  }

  /// Determines if the map is empty.
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }

  /// Returns the length of the map, in keys.
  #[inline]
  pub fn len(&self) -> usize {
    self.entries.keys().len()
  }

  /// Returns the total length of the map, in values.
  #[inline]
  pub fn total_len(&self) -> usize {
    self.entries.values().map(|v| v.len()).sum()
  }

  /// Determines if the given key is contained in the map.
  #[inline]
  pub fn contains_key(&self, key: &K) -> bool {
    self.entries.contains_key(key)
  }

  /// Determines if the given key-value pair is contained in the map.
  pub fn contains_value(&self, key: &K, value: &V) -> bool
  where
    V: PartialEq,
  {
    self
      .entries
      .get(key)
      .map(|vec| vec.contains(value))
      .unwrap_or(false)
  }

  /// Gets all values for the given key.
  #[inline]
  pub fn get(&self, key: &K) -> Option<&[V]> {
    self.entries.get(key).map(|vec| vec.as_slice())
  }

  /// Mutably gets all values for the given key.
  #[inline]
  pub fn get_mut(&mut self, key: &K) -> Option<&mut [V]> {
    self.entries.get_mut(key).map(|vec| vec.as_mut_slice())
  }

  /// Inserts the given key-value pair into the map.
  #[inline]
  pub fn insert(&mut self, key: K, value: V) {
    self.entries.entry(key).or_insert_with(Vec::new).push(value);
  }

  /// Removes the given key-value pair from the map.
  pub fn remove(&mut self, key: &K, value: V)
  where
    V: PartialEq,
  {
    if let Some(entries) = self.entries.get_mut(key) {
      entries.retain(|v| v != &value);

      if entries.is_empty() {
        self.entries.remove(key);
      }
    }
  }

  /// Removes all values for the given key from the map.
  #[inline]
  pub fn remove_all(&mut self, key: &K) {
    self.entries.remove(key);
  }

  /// Clears the map.
  #[inline]
  pub fn clear(&mut self) {
    self.entries.clear();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_insert_multiple_values_per_key() {
    let mut map = MultiMap::new();

    map.insert(1, "Value 1");
    map.insert(1, "Value 2");
    map.insert(2, "Value 3");
    map.insert(3, "Value 4");

    assert!(map.contains_key(&1));
    assert!(map.contains_key(&2));
    assert!(map.contains_key(&3));
    assert!(!map.contains_key(&4));
  }

  #[test]
  fn test_remove_key_value_pair() {
    let mut map = MultiMap::new();

    map.insert(1, "Value 1");
    map.insert(1, "Value 2");
    map.insert(2, "Value 3");
    map.insert(3, "Value 4");

    map.remove(&1, "Value 1");

    assert!(map.contains_key(&1));
    assert!(map.contains_key(&2));
    assert!(map.contains_key(&3));
    assert!(!map.contains_key(&4));
  }

  #[test]
  fn test_remove_all_values_for_single_key() {
    let mut map = MultiMap::new();

    map.insert(1, "Value 1");
    map.insert(1, "Value 2");
    map.insert(2, "Value 3");
    map.insert(3, "Value 4");

    map.remove_all(&1);

    assert!(!map.contains_key(&1));
    assert!(map.contains_key(&2));
    assert!(map.contains_key(&3));
    assert!(!map.contains_key(&4));
  }
}
