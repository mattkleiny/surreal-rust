use std::cmp::Ordering;

/// A simple binary min-heap implementation with distinct cost type.
#[derive(Clone, Debug)]
pub struct MinHeap<T, TCost> {
  entries: Vec<Entry<T, TCost>>,
}

impl<T, TCost> MinHeap<T, TCost> {
  pub fn new() -> Self {
    Self { entries: Vec::new() }
  }

  pub fn push(&mut self, value: T, cost: TCost) {
    unimplemented!()
  }

  pub fn pop(&mut self) -> Option<T> {
    unimplemented!()
  }

  pub fn size(&self) -> usize {
    self.entries.len()
  }
}

/// An entry in a min or max heap.
#[derive(Clone, Debug)]
struct Entry<T, TCost> {
  value: T,
  cost: TCost,
}

impl<T, TCost> Entry<T, TCost> {
  #[inline]
  pub const fn new(value: T, cost: TCost) -> Self {
    Self { value, cost }
  }
}

impl<T, TCost: PartialEq> PartialEq for Entry<T, TCost> {
  fn eq(&self, other: &Self) -> bool {
    self.cost.eq(&other.cost)
  }
}

impl<T, TCost: PartialOrd> PartialOrd for Entry<T, TCost> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.cost.partial_cmp(&other.cost)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_insert_items() {
    let mut heap = MinHeap::new();

    heap.push("Hello", 1.);
  }
}