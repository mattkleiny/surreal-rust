use std::ops::Sub;

use crate::maths::clamp;

/// Builds a range between the given values.
#[inline]
pub const fn range<T>(min: T, max: T) -> Range<T> {
  Range::new(min, max)
}

/// An inclusive range that spans the given (min, max) values.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Range<T> {
  pub min: T,
  pub max: T,
}

impl<T> Range<T> {
  #[inline]
  pub const fn new(min: T, max: T) -> Self {
    Self { min, max }
  }

  #[inline]
  pub fn delta(&self) -> T where T: Copy + Sub<Output=T> {
    self.max - self.min
  }

  #[inline]
  pub fn contains(&self, other: T) -> bool where T: PartialOrd {
    other >= self.min && other <= self.max
  }

  #[inline]
  pub fn clamp(&self, value: T) -> T where T: Copy + PartialOrd {
    clamp(value, self.min, self.max)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn range_should_produce_a_valid_f64_range() {
    let range = range(0., 2.);

    assert!(range.contains(1.));
    assert!(!range.contains(3.));
  }

  #[test]
  fn range_should_produce_a_valid_i64_range() {
    let range = range(-2, 5);

    assert!(range.contains(-1));
    assert!(!range.contains(6));
  }

  #[test]
  fn range_should_clamp_a_valid_range() {
    let range = range(-2, 5);

    assert_eq!(-2, range.clamp(-100));
    assert_eq!(5, range.clamp(100));
  }

  #[test]
  fn range_should_produce_a_valid_str_range() {
    let range = range("Test 1", "Test 3");

    assert!(range.contains("Test 3"));
  }
}
