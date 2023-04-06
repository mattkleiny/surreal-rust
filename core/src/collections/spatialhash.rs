use crate::{
  collections::FastHashMap,
  maths::{ivec2, IVec2},
};

/// A sparse spatial hash grid implementation.
///
/// This implementation uses a hashmap to bucket items based on their position.
/// This allows for fast insertion and removal of items, but iteration is slower
/// than a dense spatial hash grid.
pub struct SpatialHashGrid<T> {
  entries: FastHashMap<IVec2, Vec<Entry<T>>>,
  default_capacity: usize,
}

/// A single entry in a spatial hash grid.
struct Entry<T> {
  value: T,
}

impl<T> Default for SpatialHashGrid<T> {
  fn default() -> Self {
    Self {
      entries: FastHashMap::default(),
      default_capacity: 1,
    }
  }
}

impl<T> SpatialHashGrid<T> {
  /// Creates a new spatial hash grid.
  pub fn new() -> Self {
    Self::default()
  }

  /// Creates a new spatial hash grid with a default capacity.
  pub fn with_default_capacity(capacity: usize) -> Self {
    Self {
      entries: FastHashMap::default(),
      default_capacity: capacity,
    }
  }

  /// Returns true if the spatial hash grid is empty.
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Returns the number of elements in the spatial hash grid.
  pub fn len(&self) -> usize {
    self.entries.values().map(|contents| contents.len()).sum()
  }

  /// The number of cells in the spatial hash grid.
  pub fn cells(&self) -> usize {
    self.entries.len()
  }

  /// Adds an item to the spatial hash grid.
  pub fn add(&mut self, x: i32, y: i32, value: T) {
    let point = ivec2(x, y);
    let entry = Entry { value };

    self
      .entries
      .entry(point)
      .or_insert_with(|| Vec::with_capacity(self.default_capacity))
      .push(entry);
  }

  /// Removes all items at the given cell in the list.
  pub fn remove_all(&mut self, x: i32, y: i32) {
    let point = ivec2(x, y);

    if let Some(entries) = self.entries.get_mut(&point) {
      entries.clear();
      self.entries.remove(&point);
    }
  }

  /// Clears the spatial hash grid of all entries.
  pub fn clear(&mut self) {
    self.entries.clear();
  }

  /// Returns an iterator over the items in the spatial hash grid.
  pub fn iter(&self) -> impl Iterator<Item = &T> {
    self
      .entries
      .values()
      .flat_map(|contents| contents.iter().map(|entry| &entry.value))
  }

  /// Returns a mutable iterator over the items in the spatial hash grid.
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
    self
      .entries
      .values_mut()
      .flat_map(|contents| contents.iter_mut().map(|entry| &mut entry.value))
  }
}

impl<'a, T> IntoIterator for &'a SpatialHashGrid<T> {
  type Item = &'a T;
  type IntoIter = impl Iterator<Item = Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T> IntoIterator for &'a mut SpatialHashGrid<T> {
  type Item = &'a mut T;
  type IntoIter = impl Iterator<Item = Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_grid_behaviour() {
    let mut grid = SpatialHashGrid::default();

    grid.add(0, 0, 1);
    grid.add(0, 1, 2);

    assert_eq!(grid.len(), 2); // 2 items in the grid
  }
}
