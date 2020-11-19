use std::cmp::Ordering;

pub type MinHeap<T, TCost> = Heap<T, TCost, { HeapType::Min }>;
pub type MaxHeap<T, TCost> = Heap<T, TCost, { HeapType::Max }>;

/// Different `Heap` types supported by this implementation.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HeapType { Min, Max }

/// A single entry in a `Heap`.
#[derive(Clone, Debug)]
struct HeapEntry<V, C> { value: V, cost: C }

/// A simple binary heap implementation with distinct cost type.
///
/// The ordering of the heap (min or max) is controlled by a const parameter, and dictates the direction
/// in which sorting occurs.
#[derive(Clone, Debug)]
pub struct Heap<V, C, const TYPE: HeapType> {
  entries: Vec<HeapEntry<V, C>>,
}

impl<V, C, const TYPE: HeapType> Heap<V, C, { TYPE }> where C: PartialOrd {
  pub fn new() -> Self {
    Self { entries: Vec::new() }
  }

  pub fn push(&mut self, value: V, cost: C) {
    self.entries.push(HeapEntry { value, cost });
    self.bubble_up();
  }

  pub fn peek(&self) -> Option<&V> {
    if self.entries.len() > 0 {
      Some(&self.entries[0].value)
    } else {
      None
    }
  }

  pub fn pop(&mut self) -> Option<V> {
    if self.entries.len() > 0 {
      let entry = self.entries.remove(0);
      self.bubble_down();

      Some(entry.value)
    } else {
      None
    }
  }

  pub fn size(&self) -> usize {
    self.entries.len()
  }

  pub fn clear(&mut self) {
    self.entries.clear();
  }

  fn bubble_up(&mut self) {
    // there's nothing to bubble
    if self.size() < 2 { return; }

    let mut index = self.size() - 1;
    let mut parent_index = Self::parent(index);

    while Self::compare(&self.entries[index].cost, &self.entries[parent_index].cost) == Ordering::Less {
      self.entries.swap(index, parent_index);

      if parent_index > 0 {
        index = parent_index;
        parent_index = Self::parent(index);
      }

      if parent_index == index {
        break;
      }
    }

    unimplemented!()
  }

  fn bubble_down(&mut self) {
    // there's nothing to bubble
    if self.size() < 2 { return; }

    let mut parent_index = 0;

    let size = self.size();

    loop {
      let left_index = Self::left(parent_index);
      let right_index = Self::right(parent_index);

      if left_index >= size { break; }

      let index = if right_index >= size { left_index } else {
        match Self::compare(&self.entries[left_index].cost, &self.entries[right_index].cost) {
          Ordering::Less => left_index,
          Ordering::Equal => right_index,
          Ordering::Greater => right_index,
        }
      };

      if let Ordering::Less = Self::compare(&self.entries[0].cost, &self.entries[index].cost) {
        break;
      }

      self.entries.swap(parent_index, index);
      parent_index = index;
    }

    unimplemented!()
  }

  #[inline(always)]
  fn compare(left: &C, right: &C) -> Ordering {
    match TYPE {
      HeapType::Min => left.partial_cmp(right).unwrap_or(Ordering::Equal),
      HeapType::Max => right.partial_cmp(left).unwrap_or(Ordering::Equal),
    }
  }

  const fn parent(key: usize) -> usize { (key - 1) / 2 }
  const fn left(key: usize) -> usize { 2 * key + 1 }
  const fn right(key: usize) -> usize { 2 * key + 2 }
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