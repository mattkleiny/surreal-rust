use std::ops::Sub;

/// Builds a range between the given values.
pub fn range<T>(min: T, max: T) -> Range<T> { Range::new(min, max) }

/// An inclusive range that spans the given (min, max) values.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Range<T> {
  min: T,
  max: T,
}

impl<T> Range<T> {
  pub fn new(min: T, max: T) -> Self {
    Self { min, max }
  }

  pub fn delta(&self) -> T where T: Copy + Sub<Output=T> {
    self.max - self.min
  }

  pub fn contains(&self, other: T) -> bool where T: PartialOrd {
    other >= self.min && other <= self.max
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_produce_a_valid_f64_range() {
    let range = range(0., 2.);

    assert!(range.contains(1.));
    assert!(!range.contains(3.));
  }

  #[test]
  fn it_should_produce_a_valid_i64_range() {
    let range = range(-2, 5);

    assert!(range.contains(-1));
    assert!(!range.contains(6));
  }

  #[test]
  fn it_should_produce_a_valid_str_range() {
    let range = range("Test 1", "Test 3");

    assert!(range.contains("Test 3"));
  }
}