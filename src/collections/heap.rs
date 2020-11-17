pub type MinHeap<T, TCost> = Heap<T, TCost, { HeapType::Min }>;
pub type MaxHeap<T, TCost> = Heap<T, TCost, { HeapType::Max }>;

/// Different heap types supported.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HeapType { Min, Max }

/// A simple binary heap implementation with distinct cost type.
#[derive(Clone, Debug)]
pub struct Heap<V, C, const TYPE: HeapType> {
  entries: Vec<Entry<V, C>>,
}

impl<V, C, const TYPE: HeapType> Heap<V, C, { TYPE }> where C: PartialOrd {
  pub fn new() -> Self {
    Self { entries: Vec::new() }
  }

  pub fn push(&mut self, value: V, cost: C) {
    self.entries.push(Entry { value, cost });
  }

  pub fn pop(&mut self) -> Option<V> {
    unimplemented!()
  }

  pub fn size(&self) -> usize {
    self.entries.len()
  }

  // key index accessors
  const fn parent(key: usize) -> usize { (key - 1) / 2 }
  const fn left(key: usize) -> usize { 2 * key + 1 }
  const fn right(key: usize) -> usize { 2 * key + 2 }
}

/// An entry in a heap.
#[derive(Copy, Clone, Debug)]
struct Entry<V, C> {
  value: V,
  cost: C,
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