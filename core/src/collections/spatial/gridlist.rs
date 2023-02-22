use crate::{
  collections::FastHashMap,
  maths::{ivec2, IVec2},
};

/// A sparse grid list implementation.
///
/// This implementation uses a hashmap to bucket items based on their position.
/// This allows for fast insertion and removal of items, but iteration is slower
/// than a dense grid list.
#[derive(Clone)]
pub struct GridList<T> {
  entries: FastHashMap<IVec2, Vec<T>>,
  default_capacity: usize,
}

impl<T> Default for GridList<T> {
  fn default() -> Self {
    Self {
      entries: FastHashMap::default(),
      default_capacity: 1,
    }
  }
}

impl<T> GridList<T> {
  /// Creates a new grid list.
  pub fn new() -> Self {
    Self::default()
  }

  /// Creates a new grid list with a default capacity.
  pub fn with_default_capacity(capacity: usize) -> Self {
    Self {
      entries: FastHashMap::default(),
      default_capacity: capacity,
    }
  }

  /// Returns true if the grid list is empty.
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Returns the number of elements in the grid list.
  pub fn len(&self) -> usize {
    self.entries.iter().map(|(_, v)| v.len()).sum()
  }

  /// Adds an item to the grid list.
  pub fn add(&mut self, x: i32, y: i32, item: T) {
    let point = ivec2(x, y);

    let entry = self
      .entries
      .entry(point)
      .or_insert_with(|| Vec::with_capacity(self.default_capacity));

    entry.push(item);
  }

  /// Adds a list of items to the grid list.
  pub fn add_all(&mut self, x: i32, y: i32, items: &[T])
  where
    T: Clone,
  {
    let point = ivec2(x, y);

    let entry = self
      .entries
      .entry(point)
      .or_insert_with(|| Vec::with_capacity(items.len()));

    entry.extend_from_slice(items);
  }

  /// Removes an item from the grid list.
  pub fn remove(&mut self, x: i32, y: i32, item: T)
  where
    T: PartialEq,
  {
    let point = ivec2(x, y);

    if let Some(entries) = self.entries.get_mut(&point) {
      entries.retain(|i| *i != item);

      if entries.is_empty() {
        self.entries.remove(&point);
      }
    }
  }

  /// Removes all items at the given point from the grid list.
  pub fn remove_all(&mut self, x: i32, y: i32) {
    let point = ivec2(x, y);

    if let Some(entries) = self.entries.get_mut(&point) {
      entries.clear();
      self.entries.remove(&point);
    }
  }

  /// Clears the grid list.
  pub fn clear(&mut self) {
    self.entries.clear();
  }

  /// Returns an iterator over the items in the grid list.
  pub fn iter(&self) -> impl Iterator<Item = &T> {
    self.entries.iter().map(|(_, v)| v.iter()).flatten()
  }

  /// Returns a mutable iterator over the items in the grid list.
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
    self.entries.iter_mut().map(|(_, v)| v.iter_mut()).flatten()
  }
}

impl<'a, T> IntoIterator for &'a GridList<T> {
  type Item = &'a T;
  type IntoIter = impl Iterator<Item = &'a T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T> IntoIterator for &'a mut GridList<T> {
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
  fn gridlist_should_add_and_remove_items() {
    let mut grid = GridList::default();

    grid.add(0, 0, 1);
    grid.add(0, 0, 2);
    grid.add(1, 0, 2);

    assert_eq!(grid.len(), 3);

    grid.remove(0, 0, 1);

    assert_eq!(grid.len(), 2);

    grid.remove(0, 0, 2);

    assert_eq!(grid.len(), 1);
  }
}
