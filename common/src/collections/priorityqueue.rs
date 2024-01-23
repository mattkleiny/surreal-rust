use std::collections::BinaryHeap;

/// A priority queue with per-element ordering.
///
/// This is a lightweight wrapper over the built in [`BinaryHeap`] with an
/// internal node ordering explicitly defined at the point of insertion.
///
/// The weight of each element is defined by the generic type `W`, which must
/// implement [`Ord`]. Elements with a higher weight will be returned before
/// elements with a lower weight.
pub struct PriorityQueue<T, W: Ord = usize> {
  elements: BinaryHeap<Node<T, W>>,
}

/// A node in a priority queue with a custom ordering field on the element.
struct Node<T, W: Ord> {
  pub value: T,
  pub order: W,
}

impl<T, W: Ord> Default for PriorityQueue<T, W> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T, W: Ord> PriorityQueue<T, W> {
  /// Creates a new empty queue.
  #[inline]
  pub fn new() -> Self {
    Self {
      elements: BinaryHeap::new(),
    }
  }

  /// Creates a new empty queue with the given pre-sized backing capacity.
  #[inline]
  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      elements: BinaryHeap::with_capacity(capacity),
    }
  }

  /// Is the queue empty?
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.elements.is_empty()
  }

  /// Returns the number of elements in the queue.
  #[inline]
  pub fn len(&self) -> usize {
    self.elements.len()
  }

  /// Pushes a new element onto the queue with a custom order.
  #[inline]
  pub fn push(&mut self, value: T, order: W) {
    self.elements.push(Node { value, order });
  }

  /// Pops an element from the top of the queue.
  #[inline]
  pub fn pop(&mut self) -> Option<T> {
    self.elements.pop().map(|node| node.value)
  }
}

impl<T, W: Ord + Eq> Eq for Node<T, W> {}

impl<T, W: Ord + PartialEq> PartialEq for Node<T, W> {
  fn eq(&self, other: &Self) -> bool {
    self.order == other.order
  }
}

impl<T, W: Ord> Ord for Node<T, W> {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.order.cmp(&other.order)
  }
}

impl<T, W: Ord + PartialOrd> PartialOrd for Node<T, W> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.order.cmp(&other.order))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_should_not_be_empty_after_push() {
    let mut queue = PriorityQueue::new();

    queue.push("a", 1);

    assert!(!queue.is_empty());
    assert_eq!(queue.len(), 1);
  }

  #[test]
  fn test_should_return_elements_in_order() {
    let mut queue = PriorityQueue::new();

    queue.push("a", 1);
    queue.push("b", 3);
    queue.push("c", 2);

    assert_eq!(queue.pop().unwrap(), "b");
    assert_eq!(queue.pop().unwrap(), "c");
    assert_eq!(queue.pop().unwrap(), "a");
  }
}
