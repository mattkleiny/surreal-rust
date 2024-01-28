use crate::Plane;

/// A binary space partitioning tree.
#[derive(Default)]
pub struct BspTree<T> {
  root: Option<BspNode<T>>,
}

/// A node in a binary space partitioning tree.
enum BspNode<T> {
  Leaf(T),
  Branch {
    plane: Plane,
    front: Box<BspNode<T>>,
    back: Box<BspNode<T>>,
  },
}

impl<T> BspTree<T> {
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

impl<T> BspNode<T> {
  /// Inserts a value into the node.
  pub fn insert(&mut self, value: T) {
    todo!()
  }
}
