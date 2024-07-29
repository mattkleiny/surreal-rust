//! Weighted selection of elements.

use super::*;

/// A set of weighted elements.
#[derive(Default, Debug, Clone)]
pub struct WeightedSet<T> {
  entries: Vec<Entry<T>>,
}

/// An entry in a weighted set.
#[derive(Default, Debug, Clone)]
struct Entry<T> {
  pub item: T,
  pub weight: f32,
}

impl<T> WeightedSet<T> {
  /// Creates a new weighted set.
  pub fn new() -> Self {
    Self { entries: Vec::new() }
  }

  /// Creates a weighted set from a vec of entries.
  pub fn from_vec(vec: Vec<(T, f32)>) -> Self {
    let mut entries = Vec::new();

    for (item, weight) in vec.into_iter() {
      entries.push(Entry { item, weight })
    }

    Self { entries }
  }

  /// Creates a weighted set from an iterator of entries.
  pub fn from_iter(iter: impl IntoIterator<Item = (T, f32)>) -> Self {
    let mut iter = iter.into_iter();
    let mut entries = Vec::new();

    while let Some((item, weight)) = iter.next() {
      entries.push(Entry { item, weight });
    }

    Self { entries }
  }

  /// True if the set is empty.
  pub fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }

  /// The number of items in the set.
  pub fn len(&self) -> usize {
    self.entries.len()
  }

  /// The total weight of the set.
  pub fn total_weight(&self) -> f32 {
    self.entries.iter().map(|entry| entry.weight).sum()
  }

  /// Adds an item to the set.
  pub fn add(&mut self, item: T, weight: f32) {
    self.entries.push(Entry { item, weight });
  }

  /// Selects a random item from the set.
  pub fn select(&self, random: &mut Random) -> Option<&T> {
    let total_weight = self.total_weight();

    if total_weight == 0.0 {
      return None;
    }

    let mut weight = random.next::<f32>() * total_weight;

    for entry in &self.entries {
      weight -= entry.weight;

      if weight <= 0.0 {
        return Some(&entry.item);
      }
    }

    None
  }

  /// Mutably selects a random item from the set.
  pub fn select_mut(&mut self, random: &mut Random) -> Option<&mut T> {
    let total_weight = self.total_weight();

    if total_weight == 0.0 {
      return None;
    }

    let mut weight = random.next::<f32>() * total_weight;

    for entry in &mut self.entries {
      weight -= entry.weight;

      if weight <= 0.0 {
        return Some(&mut entry.item);
      }
    }

    None
  }

  /// Removes an item from the set.
  pub fn remove(&mut self, item: T)
  where
    T: PartialEq,
  {
    self.entries.retain(|entry| entry.item != item);
  }

  /// Clears the set.
  pub fn clear(&mut self) {
    self.entries.clear();
  }

  /// Iterates over the set.
  pub fn iter(&self) -> impl Iterator<Item = &T> {
    self.entries.iter().map(|entry| &entry.item)
  }

  /// Iterates over the set mutably.
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
    self.entries.iter_mut().map(|entry| &mut entry.item)
  }
}

/// Allow direct iteration over the set.
impl<'a, T> IntoIterator for &'a WeightedSet<T> {
  type Item = &'a T;
  type IntoIter = impl Iterator<Item = &'a T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

/// Allow direct mutable iteration over the set.
impl<'a, T> IntoIterator for &'a mut WeightedSet<T> {
  type Item = &'a mut T;
  type IntoIter = impl Iterator<Item = &'a mut T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_weighted_set_selection() {
    let mut random = Random::with_seed(0);
    let mut set = WeightedSet::new();

    set.add(1, 1.0);
    set.add(2, 2.0);
    set.add(3, 3.0);

    let result = set.select(&mut random);

    assert_eq!(result, Some(&1));
  }

  #[test]
  fn test_weighted_set_iteration() {
    let mut set = WeightedSet::new();

    set.add(1, 1.0);
    set.add(2, 2.0);
    set.add(3, 3.0);

    let mut iter = set.iter();

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
  }

  #[test]
  fn test_weighted_set_from_iterator() {
    let mut set = WeightedSet::from_iter([(1, 1.0), (2, 2.0), (3, 3.0)]);

    assert_eq!(set.len(), 3);
  }
}
