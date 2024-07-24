//! Weighted selection of elements from a collection.

use glam::f64;

use super::*;

/// A trait for types that can be selected in a weighted fashion.
pub trait WeightedIterator {
  type Item;

  /// Selects a weighted element from the collection.
  fn weighted(&mut self, random: &mut Random, selector: impl Fn(&Self::Item) -> f64) -> Option<Self::Item>;
}

impl<T, I: Iterator<Item = T>> WeightedIterator for I {
  type Item = T;

  fn weighted(&mut self, random: &mut Random, selector: impl Fn(&Self::Item) -> f64) -> Option<Self::Item> {
    let mut total = 0.0;
    let mut result = None;

    for entry in self.by_ref() {
      let weight = selector(&entry);
      total += weight;

      if random.next_f64() < weight / total {
        result = Some(entry);
      }
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestItem {
    pub id: u32,
    pub weight: f64,
  }

  impl TestItem {
    pub fn new(id: u32, weight: f64) -> Self {
      Self { id, weight }
    }
  }

  #[test]
  fn test_select_weighted() {
    let mut random = Random::with_seed(0);
    let items = [
      // don't format
      TestItem::new(1, 0.1),
      TestItem::new(2, 0.2),
      TestItem::new(3, 0.3),
    ];

    let result = items.iter().weighted(&mut random, |it| it.weight).unwrap();

    assert_eq!(result.id, 3);
  }
}
