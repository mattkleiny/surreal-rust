use std::collections::BinaryHeap;

use crate::maths::ApproxEq;

/// A lightweight priority queue with per-element ordering based on single
/// floating point value.
///
/// This is a lightweight wrapper over the built in [`BinaryHeap`] with an
/// internal node ordering explicitly defined at the point of insertion.
///
/// We use `f32` values because they work well for scoring algorithms and
/// similar.
pub struct PriorityQueue<T> {
  elements: BinaryHeap<Node<T>>,
}

/// A node in a priority queue with a custom ordering field on the element.
struct Node<T> {
  pub value: T,
  pub order: f32,
}

impl<T: PartialEq> Default for PriorityQueue<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: PartialEq> PriorityQueue<T> {
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
  pub fn push(&mut self, value: T, order: f32) {
    self.elements.push(Node { value, order });
  }

  /// Pops an element from the top of the queue.
  #[inline]
  pub fn pop(&mut self) -> Option<T> {
    self.elements.pop().map(|node| node.value)
  }
}

impl<T: PartialEq> PartialEq for Node<T> {
  fn eq(&self, other: &Self) -> bool {
    self.value == other.value && self.order.approx_eq(other.order)
  }
}

impl<T: PartialEq> Eq for Node<T> {}

impl<T: PartialEq> PartialOrd for Node<T> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.order.partial_cmp(&other.order)
  }
}

impl<T: PartialEq> Ord for Node<T> {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.order.partial_cmp(&other.order).unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_should_be_empty_when_created_with_new() {
    let queue: PriorityQueue<i32> = PriorityQueue::new();

    assert!(queue.is_empty());
    assert_eq!(queue.len(), 0);
  }

  #[test]
  fn test_should_be_empty_when_created_with_with_capacity() {
    let queue: PriorityQueue<i32> = PriorityQueue::with_capacity(10);

    assert!(queue.is_empty());
    assert_eq!(queue.len(), 0);
  }

  #[test]
  fn test_should_not_be_empty_after_push() {
    let mut queue = PriorityQueue::new();

    queue.push("a", 1.0);

    assert!(!queue.is_empty());
    assert_eq!(queue.len(), 1);
  }

  #[test]
  fn test_should_return_elements_in_order() {
    let mut queue = PriorityQueue::new();

    queue.push("a", 3.0);
    queue.push("b", 1.0);
    queue.push("c", 2.0);

    assert_eq!(queue.pop().unwrap(), "b");
    assert_eq!(queue.pop().unwrap(), "c");
    assert_eq!(queue.pop().unwrap(), "a");
  }

  #[test]
  fn test_should_return_none_when_popped_from_empty_queue() {
    let mut queue: PriorityQueue<i32> = PriorityQueue::new();

    assert_eq!(queue.pop(), None);
  }

  #[test]
  fn test_should_order_based_on_explicit_order() {
    let mut queue = PriorityQueue::new();

    queue.push("a", 1.0);
    queue.push("b", 2.0);
    queue.push("c", 3.0);

    assert_eq!(queue.pop().unwrap(), "c");
    assert_eq!(queue.pop().unwrap(), "b");
    assert_eq!(queue.pop().unwrap(), "a");
  }
}
