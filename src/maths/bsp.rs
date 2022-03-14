//! A simple binary space partitioning tree, with support for basic recursion and deconstruction.

use super::Axis;

type Point = super::Vector2<f32>;
type Bounds = super::Rect<f32>;

/// A node in the BSP tree.
enum Node<T> {
  Split {
    left: Box<BSP<T>>,
    right: Box<BSP<T>>,
  },
  Leaf(Option<T>),
}

/// A BSP tree of type T with support for recursive expansion.
pub struct BSP<T> {
  bounds: Bounds,
  node: Node<T>,
}

impl<T> BSP<T> {
  pub fn new_leaf(bounds: Bounds, value: T) -> Self {
    Self {
      bounds,
      node: Node::Leaf(Some(value)),
    }
  }

  pub fn new_empty(bounds: Bounds) -> Self {
    Self {
      bounds,
      node: Node::Leaf(None),
    }
  }

  /// Determines if this node is a leaf node in the tree.
  pub fn is_leaf(&self) -> bool {
    match self.node {
      Node::Split { .. } => false,
      Node::Leaf(_) => true,
    }
  }

  /// Determines if this node is a split node in the tree.
  pub fn is_split(&self) -> bool {
    !self.is_leaf()
  }

  /// Borrows the value of this leaf node.
  pub fn get(&self) -> Option<&T> {
    match &self.node {
      Node::Leaf(Some(value)) => Some(value),
      _ => None,
    }
  }

  /// Mutably borrows the value of this leaf node.
  pub fn get_mut(&mut self) -> Option<&mut T> {
    match &mut self.node {
      Node::Leaf(Some(value)) => Some(value),
      _ => None,
    }
  }

  /// Splits the BSP node at the given point along the given axis.
  pub fn split(&mut self, point: Point, axis: Axis) -> bool {
    if self.is_split() {
      return false; // we're already split
    }

    // split the node on the desired axis
    // TODO: implement these properly
    let bounds = &self.bounds;
    let (left, right) = match axis {
      Axis::Horizontal => {
        let top = Bounds::new(
          bounds.left(),
          bounds.top(),
          bounds.right(),
          bounds.bottom() / 2. + bounds.height() / 2.
        );

        let bottom = Bounds::new(
          bounds.left(),
          bounds.top() + bounds.height() / 2.,
          bounds.right(),
          bounds.bottom() / 2. + bounds.height() / 2.
        );

        (top, bottom)
      }
      Axis::Vertical => {
        let left = Bounds::new(
          bounds.left(),
          bounds.top(),
          bounds.right() / 2.,
          bounds.bottom(),
        );

        let right = Bounds::new(
          bounds.left() + bounds.width() / 2.,
          bounds.top(),
          bounds.right() / 2. + bounds.width() / 2.,
          bounds.bottom(),
        );

        (left, right)
      }
    };

    self.node = Node::Split {
      left: Box::new(Self::new_empty(left)),
      right: Box::new(Self::new_empty(right)),
    };

    true
  }

  /// Visits all nodes in the graph recursively.
  pub fn accept(&self, visitor: &mut impl Visitor<T>) {
    match &self.node {
      Node::Split { left, right } => visitor.visit_split(&self.bounds, left, right),
      Node::Leaf(value) => visitor.visit_leaf(&self.bounds, value),
    }
  }
}

/// A visitor for BSP trees.
pub trait Visitor<T> : Sized {
  fn visit_split(&mut self, bounds: &Bounds, left: &BSP<T>, right: &BSP<T>) {
    left.accept(self);
    right.accept(self);
  }

  fn visit_leaf(&mut self, bounds: &Bounds, value: &Option<T>);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn bsp_should_split_and_create_simple_trees() {
    let bounds = Bounds::new(0., 0., 100., 100.);
    let mut bsp = BSP::new_leaf(bounds, "Test");

    bsp.split(Point::new(50., 50.), Axis::Horizontal);
  }

  #[test]
  fn bsp_should_visit_all_nodes() {
    let tree = BSP {
      bounds: Bounds::new(0., 0., 100., 100.),
      node: Node::Split {
        left: Box::new(BSP {
          bounds: Bounds::new(0., 0., 50., 50.),
          node: Node::Leaf(None),
        }),
        right: Box::new(BSP {
          bounds: Bounds::new(50., 50., 100., 100.),
          node: Node::Leaf(Some("Hello, World!")),
        }),
      },
    };

    struct TestVisitor {
      buffer: String,
    }

    impl<T: std::fmt::Debug> Visitor<T> for TestVisitor {
      fn visit_leaf(&mut self, bounds: &Bounds, value: &Option<T>) {
        if let Some(value) = value {
          self.buffer += format!("The value is {:?}", value).as_str();
        }
      }
    }

    let mut visitor = TestVisitor {
      buffer: String::new(),
    };

    tree.accept(&mut visitor);

    assert!(visitor.buffer.len() > 0);
  }
}