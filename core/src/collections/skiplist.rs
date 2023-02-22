/// A simple skip-list implementation.
///
/// Skip-lists are probabilistic data structures that allow for efficient
/// insertion, deletion, and lookup of elements in a sorted list. They are
/// similar to balanced binary trees, but are more space-efficient and
/// have a lower time complexity for insertion and deletion.
///
/// The implementation is based on the paper "Skip Lists: A Probabilistic
/// Alternative to Balanced Trees" by William Pugh.
#[derive(Clone)]
pub struct SkipList<T> {
  elements: Vec<SkipListNode<T>>,
}

/// A single node in a [`SkipList`].
#[derive(Clone)]
enum SkipListNode<T> {
  Element(T),
  End,
}

impl<T> SkipList<T> {
  pub fn new() -> Self {
    SkipList {
      elements: vec![SkipListNode::End],
    }
  }

  /// Returns the number of elements in the skip list.
  pub fn len(&self) -> usize {
    todo!()
  }

  /// Returns `true` if the skip list contains no elements.
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }
}
