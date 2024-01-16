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
#[derive(Debug)]
pub enum QuadTreeNode<T> {
  Leaf(Option<QuadTreeCell<T>>),
  Branch(Box<[QuadTreeNode<T>; 4]>),
}

/// A single cell in a [`QuadTree`].
///
/// A cell contains a value and the bounds of that value in 2-space.
#[derive(Debug)]
pub struct QuadTreeCell<T> {
  value: T,
  bounds: Rectangle,
}

/// Allows visiting the nodes of a [`QuadTree`] recursively.
#[allow(unused_variables)]
pub trait QuadTreeVisitor<T> {
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

/// Allows any function to act as a visitor against nodes in the tree.
impl<T, F: FnMut(&QuadTreeNode<T>)> QuadTreeVisitor<T> for F {
  fn visit_node(&mut self, node: &QuadTreeNode<T>) {
    self(node);

    match node {
      QuadTreeNode::Leaf(None) => {} // no-op; empty leaf
      QuadTreeNode::Leaf(Some(cell)) => self.visit_leaf(&cell.value, &cell.bounds),
      QuadTreeNode::Branch(branch) => self.visit_branch(branch),
    }
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
    fn calculate_recursive<T>(node: &QuadTreeNode<T>, rect: &mut Rectangle, depth: usize) {
      if depth > 100 {
        panic!("exceeded maximum recursion depth");
      }

      match node {
        QuadTreeNode::Leaf(None) => {} // no-op; empty leaf
        QuadTreeNode::Leaf(Some(cell)) => rect.extend(&cell.bounds),
        QuadTreeNode::Branch(branch) => {
          for child in branch.iter() {
            calculate_recursive(child, rect, depth + 1);
          }
        }
      }
    }

    let mut bounds = Rectangle::default();

    if let Some(root) = &self.root {
      calculate_recursive(root, &mut bounds, 0);
    }

    bounds
  }

  /// Determines if the [`QuadTree`] is empty.
  pub fn is_empty(&self) -> bool {
    self.root.is_none()
  }

  /// Determines if the [`QuadTree`] contains the given value.
  ///
  /// This involves iterating over all the nodes in the [`QuadTree`], so it is
  /// not recommended for use in performance-critical code.
  pub fn contains(&self, value: T) -> bool
  where
    T: PartialEq,
  {
    self.iter().any(|(v, _)| v == &value)
  }

  /// Inserts a value into the [`QuadTree`].
  ///
  /// If the [`QuadTree`] is empty, the value will be inserted into the root
  /// node. Otherwise, the value will be inserted into the appropriate quadrant
  /// of the [`QuadTree`], sub-dividing the tree if necessary into smaller
  /// quadrants.
  ///
  /// If the value already exists in the [`QuadTree`], it will be replaced.
  /// This is determined by comparing the value's bounds to the bounds of
  /// existing values in the [`QuadTree`].
  ///
  /// If you instead wish to move an item within the tree, use the [`update`]
  /// method instead.
  pub fn insert(&mut self, value: T, bounds: Rectangle)
  where
    T: Clone,
  {
    fn insert_recursive<T: Clone>(
      node: &mut QuadTreeNode<T>,
      value: T,
      bounds: Rectangle,
      depth: usize,
    ) -> bool {
      if depth > 100 {
        panic!("exceeded maximum recursion depth");
      }

      match node {
        QuadTreeNode::Leaf(Some(cell)) => {
          // if the leaf is already occupied, split the leaf into a branch
          // and insert the new value into the appropriate sub-quadrant
          let mut branches = Box::new([
            QuadTreeNode::Leaf(None),
            QuadTreeNode::Leaf(None),
            QuadTreeNode::Leaf(None),
            QuadTreeNode::Leaf(None),
          ]);

          let quadrant = get_quadrant(&bounds);
          let sub_branch = &mut branches[quadrant];

          insert_recursive(sub_branch, cell.value.clone(), cell.bounds, depth + 1);
          insert_recursive(sub_branch, value, bounds, depth + 1);

          *node = QuadTreeNode::Branch(branches);

          true
        }
        QuadTreeNode::Leaf(None) => {
          // if the leaf is empty, insert the new value into the leaf
          *node = QuadTreeNode::Leaf(Some(QuadTreeCell { value, bounds }));

          true
        }
        QuadTreeNode::Branch(branch) => {
          // if the node is already a branch, insert the new value into the
          // appropriate sub-quadrant
          let quadrant = get_quadrant(&bounds);
          let node = &mut branch[quadrant];

          insert_recursive(node, value, bounds, depth + 1)
        }
      }
    }

    // if the quadtree is empty, create a new root node
    let root = self.root.get_or_insert_with(|| QuadTreeNode::Leaf(None));

    // insert the new value into the root node
    insert_recursive(root, value, bounds, 0);
  }

  /// Removes a value from the [`QuadTree`].
  pub fn remove(&mut self, _value: T) {
    todo!()
  }

  /// Clears the [`QuadTree`] of all values.
  pub fn clear(&mut self) {
    self.root = None;
  }

  /// Finds all values in the [`QuadTree`] that intersect the bounds.
  pub fn find_in_bounds(&self, bounds: Rectangle) -> Vec<&T> {
    fn find_recursive<'a, T>(
      node: &'a QuadTreeNode<T>,
      bounds: Rectangle,
      results: &mut Vec<&'a T>,
      depth: usize,
    ) {
      if depth > 100 {
        panic!("exceeded maximum recursion depth");
      }

      match node {
        QuadTreeNode::Leaf(Some(cell)) => {
          if bounds.intersects(&cell.bounds) {
            results.push(&cell.value);
          }
        }
        QuadTreeNode::Leaf(None) => {} // no-op
        QuadTreeNode::Branch(branch) => {
          for node in branch.iter() {
            find_recursive(node, bounds, results, depth + 1);
          }
        }
      }
    }

    let mut results = Vec::new();

    if let Some(root) = &self.root {
      find_recursive(root, bounds, &mut results, 0);
    }

    results
  }

  /// Finds all mutable values in the [`QuadTree`] that intersect the bounds.
  pub fn find_in_bounds_mut(&mut self, bounds: Rectangle) -> Vec<&mut T> {
    fn find_recursive<'a, T>(
      node: &'a mut QuadTreeNode<T>,
      bounds: Rectangle,
      results: &mut Vec<&'a mut T>,
      depth: usize,
    ) {
      if depth > 100 {
        panic!("exceeded maximum recursion depth");
      }

      // TODO: optimize this by only recursing into quadrants that intersect
      match node {
        QuadTreeNode::Leaf(Some(cell)) => {
          if bounds.intersects(&cell.bounds) {
            results.push(&mut cell.value);
          }
        }
        QuadTreeNode::Leaf(None) => {} // no-op
        QuadTreeNode::Branch(branch) => {
          for node in branch.iter_mut() {
            find_recursive(node, bounds, results, depth + 1);
          }
        }
      }
    }

    let mut results = Vec::new();

    if let Some(root) = &mut self.root {
      find_recursive(root, bounds, &mut results, 0);
    }

    results
  }

  /// Accepts a visitor to visit all values in the [`QuadTree`].
  pub fn accept(&self, visitor: &mut impl QuadTreeVisitor<T>) {
    if let Some(root) = &self.root {
      visitor.visit_node(root);
    }
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

/// Determines which sub-quadrant the given bounds are in
fn get_quadrant(bounds: &Rectangle) -> usize {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn quadtree_should_start_empty() {
    let tree = QuadTree::<()>::default();

    assert!(tree.is_empty());
  }

  #[test]
  fn quadtree_find_in_bounds_should_find_item() {
    let mut tree = QuadTree::default();
    let bounds = Rectangle::from_corner_points(0., 0., 1., 1.);

    tree.insert(1, bounds);

    let results = tree.find_in_bounds(bounds);

    assert_eq!(results.len(), 1);
  }

  // #[test]
  // fn quadtree_find_in_bounds_should_find_all_items_in_same_space() {
  //   let mut tree = QuadTree::default();
  //   let bounds = Rectangle::from_corner_points(0., 0., 1., 1.);

  //   tree.insert(1, bounds);
  //   tree.insert(2, bounds);

  //   let results = tree.find_in_bounds(bounds);

  //   assert_eq!(results.len(), 2);
  // }

  #[test]
  fn quadtree_find_in_bounds_mut() {
    let mut tree = QuadTree::default();
    let bounds = Rectangle::from_corner_points(0., 0., 1., 1.);

    tree.insert(1, bounds);

    let results = tree.find_in_bounds_mut(bounds);

    assert_eq!(results.len(), 1);
  }

  #[test]
  fn quadtree_should_accept_visitors() {
    let mut tree = QuadTree::default();

    tree.insert(1, Rectangle::from_corner_points(0., 0., 1., 1.));

    tree.accept(&mut |node: &QuadTreeNode<i32>| {
      println!("value: {node:?}");
    });
  }
}
