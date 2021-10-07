use std::cmp::Ordering;

pub type MinHeap<V, C> = Heap<V, C>;
pub type MaxHeap<V, C> = Heap<V, C>;

#[derive(Clone, Debug)]
struct HeapEntry<V, C> {
  value: V,
  cost: C,
}

/// A simple binary heap implementation with distinct cost type.
///
/// The ordering of the heap (min or max) is controlled by a const parameter, and dictates the direction
/// in which sorting occurs.
#[derive(Clone, Debug)]
pub struct Heap<V, C> {
  entries: Vec<HeapEntry<V, C>>,
}

impl<V, C> Heap<V, C> where C: PartialOrd {
  pub fn new() -> Self {
    Self { entries: Vec::new() }
  }

  pub fn push(&mut self, value: V, cost: C) {
    todo!()
  }

  pub fn peek(&self) -> Option<&V> {
    todo!()
  }

  pub fn pop(&mut self) -> Option<V> {
    todo!()
  }

  pub fn size(&self) -> usize {
    todo!()
  }

  pub fn clear(&mut self) {
    todo!()
  }

  fn bubble_up(&mut self) {
    todo!()
  }

  fn bubble_down(&mut self) {
    todo!()
  }

  #[inline(always)]
  fn compare(left: &C, right: &C) -> Ordering {
    todo!()
  }

  #[inline(always)]
  fn parent(key: usize) -> usize { (key - 1) / 2 }

  #[inline(always)]
  fn left(key: usize) -> usize { 2 * key + 1 }

  #[inline(always)]
  fn right(key: usize) -> usize { 2 * key + 2 }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn min_heap_should_insert_and_remove_items_in_the_right_order() {
    let mut heap = MinHeap::new();

    heap.push("A", 1.);
    heap.push("B", 2.);
    heap.push("C", 3.);

    assert_eq!("A", heap.pop().unwrap());
    assert_eq!("B", heap.pop().unwrap());
    assert_eq!("C", heap.pop().unwrap());
  }

  #[test]
  fn max_heap_should_insert_and_remove_items_in_the_right_order() {
    let mut heap = MaxHeap::new();

    heap.push("A", 1.);
    heap.push("B", 2.);
    heap.push("C", 3.);

    assert_eq!("C", heap.pop().unwrap());
    assert_eq!("B", heap.pop().unwrap());
    assert_eq!("A", heap.pop().unwrap());
  }
}