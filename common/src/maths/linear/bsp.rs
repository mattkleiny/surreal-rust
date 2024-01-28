use crate::{Plane, Vec3};

/// A value that can be inserted into a binary space partitioning tree.
pub trait BspEntry {
  /// Returns the point of the entry.
  fn point(&self) -> Vec3;
}

/// A binary space partitioning tree.
#[derive(Default)]
pub struct BspTree<T> {
  root: Option<BspNode<T>>,
}

/// A node in a binary space partitioning tree.
#[allow(dead_code)]
enum BspNode<T> {
  Leaf(T),
  Branch {
    plane: Plane,
    front: Box<BspNode<T>>,
    back: Box<BspNode<T>>,
  },
}

impl<T: BspEntry> BspTree<T> {
  /// Creates a new binary space partitioning tree.
  pub fn new() -> Self {
    Self { root: None }
  }

  /// Inserts a value into the tree.
  pub fn insert(&mut self, value: T) {
    if let Some(node) = &mut self.root {
      node.insert(value);
    } else {
      self.root = Some(BspNode::Leaf(value));
    }
  }
}

impl<T: BspEntry> BspNode<T> {
  /// Inserts a value into the node.
  pub fn insert(&mut self, value: T) {
    // recurse down the hierarchy until we find a leaf node
    fn insert_recursive<T: BspEntry>(node: &mut BspNode<T>, value: T) {
      match node {
        BspNode::Leaf(_) => {
          todo!()
        }
        BspNode::Branch { plane, front, back } => {
          if plane.distance_to_point(value.point()) > 0.0 {
            insert_recursive(front, value);
          } else {
            insert_recursive(back, value);
          }
        }
      }
    }

    insert_recursive(self, value);
  }
}
