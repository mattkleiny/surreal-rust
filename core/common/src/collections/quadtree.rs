use std::fmt::Debug;

use glam::Vec2;

use crate::maths::Rectangle;

const THRESHOLD: usize = 16; // the maximum number of values in a leaf node
const MAX_DEPTH: usize = 8; // the maximum depth of the quadtree

/// A standard Quad Tree implementation.
///
/// A Quad Tree is a spatial data structure that can be used to efficiently
/// store and retrieve values in 2-dimensional space, with fast-lookups for
/// values based on their coordinates.
pub struct QuadTree<T> {
  root: Option<QuadTreeNode<T>>,
}

/// A single node in a [`QuadTree`].
///
/// A node can either be a leaf or a branch. Each branch contains up to
/// four sub-nodes, each representing a quarter of the parent node's area.
#[derive(Debug)]
pub enum QuadTreeNode<T> {
  Empty,
  Leaf(Vec<QuadTreeCell<T>>),
  Branch(Box<[QuadTreeNode<T>; 4]>),
}

/// A single cell in a [`QuadTree`].
///
/// A cell contains a value and the bounds of that value in 2-space.
#[derive(Clone, Debug)]
pub struct QuadTreeCell<T> {
  value: T,
  bounds: Rectangle,
}

impl<T> Default for QuadTree<T> {
  /// Creates a new empty [`QuadTree`].
  fn default() -> Self {
    Self { root: None }
  }
}

impl<T> QuadTree<T> {
  /// Creates a new empty [`QuadTree`].
  pub fn new() -> Self {
    Self::default()
  }

  /// Determines if the [`QuadTree`] is empty.
  pub fn is_empty(&self) -> bool {
    self.root.is_none()
  }

  /// Determines the number of values in the [`QuadTree`].
  pub fn len(&self) -> usize {
    self.iter().count()
  }

  /// Clears the [`QuadTree`] of all values.
  pub fn clear(&mut self) {
    self.root = None;
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

  /// Determines if the [`QuadTree`] contains the given value within the given
  /// bounds.
  pub fn contains_in_bounds(&self, _value: T, _position: Vec2) -> bool
  where
    T: PartialEq,
  {
    todo!()
  }

  /// Calculates the total bounds of the [`QuadTree`] by visiting all it's nodes
  pub fn calculate_bounds(&self) -> Rectangle {
    fn calculate_recursive<T>(node: &QuadTreeNode<T>, rect: &mut Rectangle) {
      match node {
        QuadTreeNode::Empty => {}
        QuadTreeNode::Leaf(cells) => {
          for cell in cells {
            rect.extend(&cell.bounds);
          }
        }
        QuadTreeNode::Branch(branch) => {
          for child in branch.iter() {
            calculate_recursive(child, rect);
          }
        }
      }
    }

    let mut bounds = Rectangle::default();

    if let Some(root) = &self.root {
      calculate_recursive(root, &mut bounds);
    }

    bounds
  }

  /// Inserts a value into the [`QuadTree`].
  ///
  /// If the [`QuadTree`] is empty, the value will be inserted into the root
  /// node. Otherwise, the value will be inserted into the appropriate quadrant
  /// of the [`QuadTree`], subdividing the tree if necessary into smaller
  /// quadrants.
  pub fn insert(&mut self, value: T, bounds: Rectangle)
  where
    T: Clone,
  {
    fn insert_recursive<T: Clone>(
      node: &mut QuadTreeNode<T>,
      value: T,
      parent: &Rectangle,
      bounds: Rectangle,
      depth: usize,
    ) {
      match node {
        QuadTreeNode::Empty => {
          // if the leaf is empty, insert the new value into the leaf
          let cells = vec![QuadTreeCell { value, bounds }];

          *node = QuadTreeNode::Leaf(cells);
        }
        QuadTreeNode::Leaf(cells) if cells.len() > THRESHOLD && depth < MAX_DEPTH => {
          // if the leaf is already occupied, split the leaf into a branch
          // and insert the new value into the appropriate sub-quadrant
          let mut branches = Box::new([
            QuadTreeNode::Empty,
            QuadTreeNode::Empty,
            QuadTreeNode::Empty,
            QuadTreeNode::Empty,
          ]);

          // work out which sub-quadrant the existing value is in
          let quadrant_index = get_quadrant_index(parent, &bounds);
          let sub_branch = &mut branches[quadrant_index];

          for cell in cells.drain(..) {
            // insert the existing values into the appropriate sub-quadrant
            insert_recursive(sub_branch, cell.value, parent, cell.bounds, depth + 1);
          }

          *node = QuadTreeNode::Branch(branches);
        }
        QuadTreeNode::Leaf(cells) => {
          // if the leaf is under the threshold, insert the new value into the leaf
          cells.push(QuadTreeCell { value, bounds });
        }
        QuadTreeNode::Branch(branch) => {
          // if the node is already a branch, insert the new value into the
          // appropriate sub-quadrant
          let quadrant = get_quadrant_index(parent, &bounds);
          let node = &mut branch[quadrant];

          insert_recursive(node, value, parent, bounds, depth + 1)
        }
      }
    }

    // if the quadtree is empty, create a new root node
    let root_bounds = self.calculate_bounds();
    let root = self.root.get_or_insert_with(|| QuadTreeNode::Empty);

    // insert the new value into the root node
    insert_recursive(root, value, &root_bounds, bounds, 0);
  }

  /// Removes a value from the [`QuadTree`].
  pub fn remove(&mut self, value: T)
  where
    T: PartialEq,
  {
    fn remove_recursive<T: PartialEq>(node: &mut QuadTreeNode<T>, value: &T) {
      match node {
        QuadTreeNode::Empty => {} // no-op
        QuadTreeNode::Leaf(cells) => {
          cells.retain(|cell| cell.value != *value);
        }
        QuadTreeNode::Branch(branch) => {
          for node in branch.iter_mut() {
            remove_recursive(node, value);
          }
        }
      }
    }

    if let Some(root) = &mut self.root {
      remove_recursive(root, &value);
    }
  }

  /// Finds all values in the [`QuadTree`] that intersect the bounds.
  pub fn find_in_bounds(&self, bounds: Rectangle) -> Vec<&T> {
    fn find_recursive<'a, T>(node: &'a QuadTreeNode<T>, bounds: Rectangle, results: &mut Vec<&'a T>) {
      match node {
        QuadTreeNode::Empty => {} // no-op
        QuadTreeNode::Leaf(cells) => {
          for cell in cells {
            if bounds.intersects(&cell.bounds) {
              results.push(&cell.value);
            }
          }
        }
        QuadTreeNode::Branch(branch) => {
          for node in branch.iter() {
            find_recursive(node, bounds, results);
          }
        }
      }
    }

    let mut results = Vec::new();

    if let Some(root) = &self.root {
      find_recursive(root, bounds, &mut results);
    }

    results
  }

  /// Finds all mutable values in the [`QuadTree`] that intersect the bounds.
  pub fn find_in_bounds_mut(&mut self, bounds: Rectangle) -> Vec<&mut T> {
    fn find_recursive<'a, T>(node: &'a mut QuadTreeNode<T>, bounds: Rectangle, results: &mut Vec<&'a mut T>) {
      // TODO: optimize this by only recursing into quadrants that intersect
      match node {
        QuadTreeNode::Empty => {} // no-op
        QuadTreeNode::Leaf(cells) => {
          for cell in cells {
            if bounds.intersects(&cell.bounds) {
              results.push(&mut cell.value);
            }
          }
        }
        QuadTreeNode::Branch(branch) => {
          for node in branch.iter_mut() {
            find_recursive(node, bounds, results);
          }
        }
      }
    }

    let mut results = Vec::new();

    if let Some(root) = &mut self.root {
      find_recursive(root, bounds, &mut results);
    }

    results
  }

  /// Recursively walks the [`QuadTree`] and calls the given function.
  pub fn walk(&self, mut body: impl FnMut(&T, &Rectangle)) {
    fn walk_recursive<T>(node: &QuadTreeNode<T>, body: &mut impl FnMut(&T, &Rectangle)) {
      match node {
        QuadTreeNode::Empty => {} // no-op
        QuadTreeNode::Leaf(cells) => {
          for cell in cells {
            body(&cell.value, &cell.bounds);
          }
        }
        QuadTreeNode::Branch(branch) => {
          for node in branch.iter() {
            walk_recursive(node, body);
          }
        }
      }
    }

    if let Some(root) = &self.root {
      walk_recursive(root, &mut body);
    }
  }

  /// Recursively walks the [`QuadTree`] and calls the given function.
  pub fn walk_mut(&mut self, mut body: impl FnMut(&mut T, &Rectangle)) {
    fn walk_mut_recursive<T>(node: &mut QuadTreeNode<T>, body: &mut impl FnMut(&mut T, &Rectangle)) {
      match node {
        QuadTreeNode::Empty => {} // no-op
        QuadTreeNode::Leaf(cells) => {
          for cell in cells {
            body(&mut cell.value, &cell.bounds);
          }
        }
        QuadTreeNode::Branch(branch) => {
          for node in branch.iter_mut() {
            walk_mut_recursive(node, body);
          }
        }
      }
    }

    if let Some(root) = &mut self.root {
      walk_mut_recursive(root, &mut body);
    }
  }

  /// Iterates over the values in the [`QuadTree`].
  pub fn iter(&self) -> impl Iterator<Item = (&T, usize)> {
    struct Iter<'a, T> {
      stack: Vec<IterEntry<'a, T>>,
    }

    enum IterEntry<'a, T> {
      Empty,
      Node(&'a QuadTreeNode<T>, usize),
      Cell(&'a QuadTreeCell<T>, usize),
    }

    impl<'a, T> Iterator for Iter<'a, T> {
      type Item = (&'a T, usize);

      fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
          Some(IterEntry::Node(node, depth)) => match node {
            QuadTreeNode::Leaf(cells) => {
              for cell in cells {
                self.stack.push(IterEntry::Cell(cell, depth));
              }

              self.next()
            }
            QuadTreeNode::Branch(branch) => {
              for node in branch.iter() {
                self.stack.push(IterEntry::Node(node, depth + 1));
              }

              self.next()
            }
            _ => None,
          },
          Some(IterEntry::Cell(cell, level)) => Some((&cell.value, level)),
          _ => None,
        }
      }
    }

    if let Some(root) = &self.root {
      Iter {
        stack: vec![IterEntry::Node(root, 0)],
      }
    } else {
      Iter {
        stack: vec![IterEntry::Empty],
      }
    }
  }

  /// Mutably iterates over the values in the [`QuadTree`].
  pub fn iter_mut(&mut self) -> impl Iterator<Item = (&mut T, usize)> {
    struct IterMut<'a, T> {
      stack: Vec<IterEntry<'a, T>>,
    }

    enum IterEntry<'a, T> {
      Empty,
      Node(&'a mut QuadTreeNode<T>, usize),
      Cell(&'a mut QuadTreeCell<T>, usize),
    }

    impl<'a, T> Iterator for IterMut<'a, T> {
      type Item = (&'a mut T, usize);

      fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
          Some(IterEntry::Node(node, depth)) => match node {
            QuadTreeNode::Leaf(cells) => {
              for cell in cells {
                self.stack.push(IterEntry::Cell(cell, depth));
              }

              self.next()
            }
            QuadTreeNode::Branch(branch) => {
              for node in branch.iter_mut() {
                self.stack.push(IterEntry::Node(node, depth + 1));
              }

              self.next()
            }
            _ => None,
          },
          Some(IterEntry::Cell(cell, level)) => Some((&mut cell.value, level)),
          _ => None,
        }
      }
    }

    if let Some(root) = &mut self.root {
      IterMut {
        stack: vec![IterEntry::Node(root, 0)],
      }
    } else {
      IterMut {
        stack: vec![IterEntry::Empty],
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

impl<'a, T> IntoIterator for &'a mut QuadTree<T> {
  type Item = (&'a mut T, usize);
  type IntoIter = impl Iterator<Item = Self::Item>;

  /// Mutably iterates over the values in the [`QuadTree`] recursively.
  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
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

/// Determines which sub-quadrant index the given bounds are in relative to its
/// parent bounds.
fn get_quadrant_index(parent_bounds: &Rectangle, value_bounds: &Rectangle) -> usize {
  let half_width = parent_bounds.width() / 2.0;
  let half_height = parent_bounds.height() / 2.0;

  let x = parent_bounds.x();
  let y = parent_bounds.y();

  let x_mid = x + half_width;
  let y_mid = y + half_height;

  let x_quadrant = if x_mid < value_bounds.x() { 0 } else { 1 };
  let y_quadrant = if y_mid < value_bounds.y() { 0 } else { 2 };

  x_quadrant | y_quadrant
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_quadrant_should_produce_correct_bounds() {
    let root_bounds = Rectangle::from_corner_points(-1., -1., 1., 1.);

    let quadrant1 = get_quadrant_index(&root_bounds, &Rectangle::from_corner_points(0., 0., 1., 1.));
    let quadrant2 = get_quadrant_index(&root_bounds, &Rectangle::from_corner_points(1., 0., 2., 1.));
    let quadrant3 = get_quadrant_index(&root_bounds, &Rectangle::from_corner_points(0., 1., 1., 2.));
    let quadrant4 = get_quadrant_index(&root_bounds, &Rectangle::from_corner_points(1., 1., 2., 2.));

    println!("{} {} {} {}", quadrant1, quadrant2, quadrant3, quadrant4);
  }

  #[test]
  fn test_quadtree_should_start_empty() {
    let tree = QuadTree::<()>::default();

    assert!(tree.is_empty());
  }

  #[test]
  fn test_quadtree_find_in_bounds_should_find_item() {
    let mut tree = QuadTree::default();
    let bounds = Rectangle::from_corner_points(0., 0., 1., 1.);

    tree.insert(1, bounds);

    let results = tree.find_in_bounds(bounds);

    assert_eq!(results.len(), 1);
  }

  #[test]
  fn test_quadtree_find_in_bounds_mut() {
    let mut tree = QuadTree::default();
    let bounds = Rectangle::from_corner_points(0., 0., 1., 1.);

    tree.insert(1, bounds);

    let results = tree.find_in_bounds_mut(bounds);

    assert_eq!(results.len(), 1);
  }

  #[test]
  fn test_quadtree_visualization() {
    let mut tree = QuadTree::default();

    tree.insert(0, Rectangle::from_corner_points(-1., -1., 1., 1.));
    tree.insert(1, Rectangle::from_corner_points(0., 0., 1., 1.));
    tree.insert(2, Rectangle::from_corner_points(1., 0., 2., 1.));
    tree.insert(3, Rectangle::from_corner_points(0., 1., 1., 2.));
    tree.insert(4, Rectangle::from_corner_points(1., 1., 2., 2.));
    tree.insert(5, Rectangle::from_corner_points(-1., -1., 0., 0.));
    tree.insert(6, Rectangle::from_corner_points(-2., -2., -1., -1.));
    tree.insert(7, Rectangle::from_corner_points(-2., -1., -1., 0.));
    tree.insert(8, Rectangle::from_corner_points(-1., -2., 0., -1.));
    tree.insert(9, Rectangle::from_corner_points(-1., 0., 0., 1.));
    tree.insert(10, Rectangle::from_corner_points(-2., 0., -1., 1.));
    tree.insert(11, Rectangle::from_corner_points(-1., 1., 0., 2.));
    tree.insert(12, Rectangle::from_corner_points(-2., 1., -1., 2.));
    tree.insert(13, Rectangle::from_corner_points(0., -1., 1., 0.));
    tree.insert(14, Rectangle::from_corner_points(0., -2., 1., -1.));
    tree.insert(15, Rectangle::from_corner_points(1., -1., 2., 0.));
    tree.insert(16, Rectangle::from_corner_points(1., -2., 2., -1.));
    tree.insert(17, Rectangle::from_corner_points(0., -1., 1., 0.));
    tree.insert(18, Rectangle::from_corner_points(0., -2., 1., -1.));

    println!("{:#?}", tree);
  }
}
