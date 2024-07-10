use std::{collections::HashMap, hash::RandomState};

use crate::{
  impl_arena_index,
  maths::{ivec2, IVec2, Vec2},
  unsafe_mutable_alias, Arena,
};

// Internal index for a spatial hash map entry.
impl_arena_index!(EntryIndex);

/// A shape in a spatial hash map.
pub enum SpatialShape {
  Circle { center: Vec2, radius: f32 },
  AABB { min: Vec2, max: Vec2 },
}

impl SpatialShape {
  /// Determines if this shape intersects the given other shape
  pub fn intersects(&self, _other: &SpatialShape) -> bool {
    todo!()
  }
}

/// A sparse spatial hash map implementation.
///
/// This implementation uses a hashmap to bucket items based on their position.
/// This allows for fast insertion and removal of items, but iteration is slower
/// than a dense spatial hash grid.
pub struct SpatialHashMap<T, S = RandomState> {
  resolution: f32,
  values: Arena<EntryIndex, Entry<T>>,
  lookup: HashMap<IVec2, Vec<EntryIndex>, S>,
}

/// A single entry in a spatial hash grid.
struct Entry<T> {
  value: T,
  shape: SpatialShape,
}

impl<T> Default for SpatialHashMap<T> {
  fn default() -> Self {
    Self {
      resolution: 100.0,
      values: Arena::default(),
      lookup: HashMap::default(),
    }
  }
}

impl<T> SpatialHashMap<T> {
  /// Creates a new spatial hash map.
  pub fn new() -> Self {
    Self::default()
  }

  /// Creates a new spatial hash map with the specified resolution.
  pub fn with_resolution(resolution: f32) -> Self {
    Self {
      resolution,
      values: Arena::default(),
      lookup: HashMap::default(),
    }
  }

  /// Returns true if the spatial hash map is empty.
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Returns the number of elements in the spatial hash map.
  pub fn len(&self) -> usize {
    self.lookup.values().map(|contents| contents.len()).sum()
  }

  /// The number of cells in the spatial hash map.
  pub fn cells(&self) -> usize {
    self.lookup.len()
  }

  /// Adds an item to the spatial hash map.
  pub fn add(&mut self, shape: SpatialShape, value: T) {
    match &shape {
      SpatialShape::Circle { center, radius } => {
        let min = (*center - Vec2::splat(*radius)).floor();
        let max = (*center + Vec2::splat(*radius)).ceil();

        self.add(SpatialShape::AABB { min, max }, value);
      }
      SpatialShape::AABB { min, max } => {
        let min = (*min / self.resolution).floor();
        let max = (*max / self.resolution).ceil();
        let capacity = max.x as usize - min.x as usize;

        let entry = Entry { value, shape };
        let index = self.values.insert(entry);

        for x in min.x as i32..max.x as i32 {
          for y in min.y as i32..max.y as i32 {
            let point = ivec2(x, y);

            self
              .lookup
              .entry(point)
              .or_insert_with(|| Vec::with_capacity(capacity))
              .push(index);
          }
        }
      }
    };
  }

  /// Queries for all entries in the given shape
  pub fn query(&self, shape: SpatialShape) -> impl Iterator<Item = &T> {
    self
      .values
      .iter()
      .filter(move |it| it.shape.intersects(&shape))
      .map(|it| &it.value)
  }

  /// Mutably queries for all entries in the given shape
  pub fn query_mut(&mut self, shape: SpatialShape) -> impl Iterator<Item = &T> {
    self
      .values
      .iter_mut()
      .filter(move |it| it.shape.intersects(&shape))
      .map(|it| &it.value)
  }

  /// Clears the spatial hash map of all entries.
  pub fn clear(&mut self) {
    self.lookup.clear();
  }

  /// Returns an iterator over the items in the spatial hash map.
  pub fn iter(&self) -> impl Iterator<Item = &T> {
    self
      .lookup
      .values()
      .flat_map(|contents| contents.iter())
      .map(|index| self.values.get(*index))
      .flat_map(|entry| entry.map(|it| &it.value))
  }

  /// Returns a mutable iterator over the items in the spatial hash map.
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
    struct Iter<'a, T> {
      indices: Vec<EntryIndex>,
      values: &'a mut Arena<EntryIndex, Entry<T>>,
      marker: std::marker::PhantomData<&'a T>,
    }

    impl<'a, T> Iterator for Iter<'a, T> {
      type Item = &'a mut T;

      fn next(&mut self) -> Option<Self::Item> {
        while let Some(index) = self.indices.pop() {
          if let Some(entry) = self.values.get_mut(index) {
            return Some(unsafe { unsafe_mutable_alias(&mut entry.value) });
          }
        }

        None
      }
    }

    // collect indices in advance
    let indices = self
      .lookup
      .values()
      .flat_map(|contents| contents.iter())
      .map(|it| *it)
      .collect();

    Iter {
      indices,
      values: &mut self.values,
      marker: std::marker::PhantomData,
    }
  }
}

impl<'a, T> IntoIterator for &'a SpatialHashMap<T> {
  type Item = &'a T;
  type IntoIter = impl Iterator<Item = Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T> IntoIterator for &'a mut SpatialHashMap<T> {
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
  fn spatial_hash_map_should_insert_at_defined_resolution() {
    let mut map = SpatialHashMap::new();

    map.add(
      SpatialShape::Circle {
        center: Vec2::ZERO,
        radius: 100.0,
      },
      "Hello, World!",
    );

    assert_eq!(map.len(), 4);
  }
}
