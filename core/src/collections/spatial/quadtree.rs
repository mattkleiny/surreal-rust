use crate::maths::Rectangle;

/// A standard Quad Tree implementation.
///
/// A Quad Tree is a spatial data structure that can be used to efficiently
/// store and retrieve values in 2-dimensional space, with fast-lookups for
/// values based on their coordinates.
pub struct QuadTree<T> {
  root: Option<QuadTreeNode<T>>,
}

/// A single node in a [`QuadTree`]
enum QuadTreeNode<T> {
  Leaf(T),
  Branch(Box<[QuadTreeNode<T>; 4]>),
}

impl<T> Default for QuadTree<T> {
  /// Creates a new empty Quad Tree.
  fn default() -> Self {
    Self { root: None }
  }
}

#[allow(unused_variables)]
impl<T> QuadTree<T> {
  /// Creates a new empty Quad Tree.
  pub fn new() -> Self {
    Self::default()
  }

  /// Determines if the Quad Tree is empty.
  pub fn is_empty(&self) -> bool {
    self.root.is_none()
  }

  /// Calculates the total bounds of the Quad Tree.
  pub fn calculate_bounds(&self) -> Rectangle {
    todo!()
  }

  /// Determines if the Quad Tree contains the given value.
  pub fn contains(&self, value: T) -> bool {
    todo!()
  }

  /// Inserts a value into the Quad Tree.
  pub fn insert(&mut self, value: T, bounds: Rectangle) {
    todo!()
  }

  /// Removes a value from the Quad Tree.
  pub fn remove(&mut self, value: T, bounds: Rectangle) {
    todo!()
  }

  /// Moves a value in the Quad Tree to a new position.
  pub fn move_bounds(&mut self, value: T, old_bounds: Rectangle, new_bounds: Rectangle) {
    todo!()
  }

  /// Finds all values in the Quad Tree that intersect the given bounds.
  pub fn find_in_bounds(&self, bounds: Rectangle) -> Vec<&T> {
    fn depth_first_search<T>(node: &QuadTreeNode<T>, bounds: Rectangle, results: &mut Vec<&T>) {
      todo!()
    }

    let mut results = Vec::new();

    if let Some(root) = &self.root {
      depth_first_search(root, bounds, &mut results);
    }

    results
  }

  /// Finds all values in the Quad Tree that intersect the given bounds.
  pub fn find_in_bounds_mut(&mut self, bounds: Rectangle) -> Vec<&mut T> {
    fn depth_first_search<T>(
      node: &mut QuadTreeNode<T>,
      bounds: Rectangle,
      results: &mut Vec<&mut T>,
    ) {
      todo!()
    }

    let mut results = Vec::new();

    if let Some(root) = &mut self.root {
      depth_first_search(root, bounds, &mut results);
    }

    results
  }

  /// Clears the Quad Tree of all values.
  pub fn clear(&mut self) {
    self.root = None;
  }

  /// Iterates over the values in the Quad Tree.
  pub fn iter(&self) -> impl Iterator<Item = &T> {
    struct Iter<'a, T> {
      quadtree: &'a QuadTree<T>,
    }

    impl<'a, T> Iterator for Iter<'a, T> {
      type Item = &'a T;

      fn next(&mut self) -> Option<Self::Item> {
        todo!()
      }
    }

    Iter { quadtree: self }
  }

  /// Iterates over the values in the Quad Tree mutably.
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
    struct IterMut<'a, T> {
      quadtree: &'a mut QuadTree<T>,
    }

    impl<'a, T> Iterator for IterMut<'a, T> {
      type Item = &'a mut T;

      fn next(&mut self) -> Option<Self::Item> {
        todo!()
      }
    }

    IterMut { quadtree: self }
  }
}

impl<'a, T> IntoIterator for &'a QuadTree<T> {
  type Item = &'a T;
  type IntoIter = impl Iterator<Item = Self::Item>;

  /// Iterates over the values in the Quad Tree.
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T> IntoIterator for &'a mut QuadTree<T> {
  type Item = &'a mut T;
  type IntoIter = impl Iterator<Item = Self::Item>;

  /// Iterates over the values in the Quad Tree mutably.
  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn quadtree_should_start_empty() {
    let tree = QuadTree::<()>::default();

    assert!(tree.is_empty());
  }
}