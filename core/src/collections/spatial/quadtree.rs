use std::fmt::Debug;

use crate::maths::Rectangle;

/// A standard Quad Tree implementation.
///
/// A Quad Tree is a spatial data structure that can be used to efficiently
/// store and retrieve values in 2-dimensional space, with fast-lookups for
/// values based on their coordinates.
///
/// A Quad Tree is a tree data structure where each node has at most four
/// children. Each node represents a rectangular area of space, and each
/// child represents a quarter of that area. The root node represents the
/// entire area of space.
pub struct QuadTree<T> {
  root: Option<QuadTreeNode<T>>,
}

/// A single node in a [`QuadTree`]. A node can either be a leaf or a branch:
///
/// * A leaf contains a single value and the bounds of that value.
/// * A branch contains four child nodes.
enum QuadTreeNode<T> {
  Leaf(Option<QuadTreeCell<T>>),
  Branch(Box<[QuadTreeNode<T>; 4]>),
}

/// A single cell in a [`QuadTree`].
///
/// A cell contains a value and the bounds of that value in 2-space.
struct QuadTreeCell<T> {
  value: T,
  bounds: Rectangle,
}

/// Allows visiting the nodes of a [`QuadTree`] recursively.
#[allow(unused_variables)]
trait QuadTreeVisitor<T> {
  fn visit_node(&mut self, node: &QuadTreeNode<T>) {
    match node {
      QuadTreeNode::Leaf(None) => {} // no-op; empty leaf
      QuadTreeNode::Leaf(Some(cell)) => self.visit_leaf(&cell.value, &cell.bounds),
      QuadTreeNode::Branch(branch) => self.visit_branch(branch),
    }
  }

  fn visit_leaf(&mut self, leaf: &T, bounds: &Rectangle) {}
  fn visit_branch(&mut self, branch: &[QuadTreeNode<T>; 4]) {}
}

impl<T> QuadTreeNode<T> {
  fn accept(&self, visitor: &mut impl QuadTreeVisitor<T>) {
    visitor.visit_node(self);
  }
}

impl<T> Default for QuadTree<T> {
  /// Creates a new empty [`QuadTree`].
  fn default() -> Self {
    Self { root: None }
  }
}

impl<T: Debug> Debug for QuadTree<T> {
  /// Formats the [`QuadTree`] as a debug tree for easier visualisation.
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for (node, level) in self.iter() {
      let indent = if level > 0 {
        " ".repeat(level * 2) + "⤷"
      } else {
        " ".repeat(level * 2)
      };

      writeln!(formatter, "{indent}{node:?}")?;
    }

    Ok(())
  }
}

impl<T> QuadTree<T> {
  /// Creates a new empty [`QuadTree`].
  pub fn new() -> Self {
    Self::default()
  }

  /// Calculates the total bounds of the [`QuadTree`] by visiting all it's nodes
  pub fn calculate_bounds(&self) -> Rectangle {
    struct BoundsVisitor {
      total_bounds: Rectangle,
    }

    impl<T> QuadTreeVisitor<T> for BoundsVisitor {
      fn visit_leaf(&mut self, _leaf: &T, bounds: &Rectangle) {
        self.total_bounds.extend(bounds);
      }
    }

    let mut visitor = BoundsVisitor {
      total_bounds: Rectangle::default(),
    };

    if let Some(root) = &self.root {
      root.accept(&mut visitor);
    }

    visitor.total_bounds
  }

  /// Determines if the [`QuadTree`] is empty.
  pub fn is_empty(&self) -> bool {
    self.root.is_none()
  }

  /// Determines if the [`QuadTree`] contains the given value.
  pub fn contains(&self, value: T) -> bool
  where
    T: PartialEq,
  {
    self.iter().any(|(v, _)| v == &value)
  }

  /// Inserts a value into the [`QuadTree`].
  pub fn insert(&mut self, value: T, bounds: Rectangle)
  where
    T: Clone,
  {
    // determines which sub-quadrant the given bounds are in
    fn get_quadrant(bounds: Rectangle) -> usize {
      let half_width = bounds.width() / 2.0;
      let half_height = bounds.height() / 2.0;

      let x = bounds.x();
      let y = bounds.y();

      let x_mid = x + half_width;
      let y_mid = y + half_height;

      let x_quadrant = if x_mid < bounds.x() { 0 } else { 1 };
      let y_quadrant = if y_mid < bounds.y() { 0 } else { 2 };

      x_quadrant | y_quadrant
    }

    // insert a new value into the quadtree, recursively splitting any existing
    // branches into quadrants if necessary
    fn insert_recursive<T: Clone>(
      node: &mut QuadTreeNode<T>,
      value: T,
      bounds: Rectangle,
      level: usize,
    ) -> bool {
      match node {
        QuadTreeNode::Leaf(Some(cell)) => {
          // if the leaf is already occupied, split the leaf into a branch
          // and insert the new value into the appropriate quadrant
          let mut branch = Box::new([
            QuadTreeNode::Leaf(None),
            QuadTreeNode::Leaf(None),
            QuadTreeNode::Leaf(None),
            QuadTreeNode::Leaf(None),
          ]);
          let mut inserted = false;

          // insert the existing value into the appropriate quadrant
          let existing_bounds = cell.bounds;
          let existing_quadrant = get_quadrant(existing_bounds);
          let existing_node = &mut branch[existing_quadrant];
          let new_value = cell.value.clone();
          inserted |= insert_recursive(existing_node, new_value, existing_bounds, level + 1);

          // insert the new value into the appropriate quadrant
          let new_quadrant = get_quadrant(bounds);
          let new_node = &mut branch[new_quadrant];
          let new_value = cell.value.clone();
          inserted |= insert_recursive(new_node, new_value, bounds, level + 1);

          *node = QuadTreeNode::Branch(branch);

          inserted
        }
        QuadTreeNode::Leaf(None) => {
          // if the leaf is empty, insert the new value into the leaf
          *node = QuadTreeNode::Leaf(Some(QuadTreeCell { value, bounds }));

          true
        }
        QuadTreeNode::Branch(branch) => {
          // if the node is already  branch, insert the new value into the appropriate
          // quadrant
          let quadrant = get_quadrant(bounds);
          let node = &mut branch[quadrant];

          insert_recursive(node, value, bounds, level + 1)
        }
      }
    }

    // if the quadtree is empty, create a new root node
    let root = self.root.get_or_insert_with(|| QuadTreeNode::Leaf(None));

    // insert the new value into the root node
    insert_recursive(root, value, bounds, 0);
  }

  /// Removes a value from the [`QuadTree`].
  pub fn remove(&mut self, _value: T, _bounds: Rectangle) {
    todo!()
  }

  /// Clears the [`QuadTree`] of all values.
  pub fn clear(&mut self) {
    self.root = None;
  }

  /// Moves a value in the [`QuadTree`] to a new position.
  pub fn update_bounds(&mut self, _value: T, _old_bounds: Rectangle, _new_bounds: Rectangle) {
    todo!()
  }

  /// Finds all values in the [`QuadTree`] that intersect the given bounds.
  pub fn find_in_bounds(&self, bounds: Rectangle) -> Vec<&T> {
    fn depth_first_search<T>(_node: &QuadTreeNode<T>, _bounds: Rectangle, _results: &mut Vec<&T>) {
      todo!()
    }

    let mut results = Vec::new();

    if let Some(root) = &self.root {
      depth_first_search(root, bounds, &mut results);
    }

    results
  }

  /// Iterates over the values in the [`QuadTree`].
  pub fn iter(&self) -> impl Iterator<Item = (&T, usize)> {
    struct IterRecursive<'a, T> {
      stack: Vec<(&'a QuadTreeNode<T>, usize)>,
    }

    impl<'a, T> Iterator for IterRecursive<'a, T> {
      type Item = (&'a T, usize);

      fn next(&mut self) -> Option<Self::Item> {
        if let Some((node, level)) = self.stack.pop() {
          match node {
            QuadTreeNode::Leaf(Some(cell)) => {
              return Some((&cell.value, level));
            }
            QuadTreeNode::Leaf(None) => {} // no-op
            QuadTreeNode::Branch(branches) => {
              for branch in branches.iter() {
                self.stack.push((branch, level + 1));
              }
            }
          }
        }

        None
      }
    }

    if let Some(root) = &self.root {
      IterRecursive {
        stack: vec![(root, 0)],
      }
    } else {
      IterRecursive {
        stack: Vec::with_capacity(0),
      }
    }
  }
}

impl<'a, T> IntoIterator for &'a QuadTree<T> {
  type Item = (&'a T, usize);
  type IntoIter = impl Iterator<Item = Self::Item>;

  /// Iterates over the values in the [`QuadTree`] recursively.
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
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
