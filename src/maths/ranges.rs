use crate::maths::{clamp, Numeric};

/// Builds a range between the given values.
#[inline]
pub const fn range<T>(min: T, max: T) -> Range<T> where T : Numeric {
  Range::new(min, max)
}

/// An inclusive range that spans the given (min, max) values.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Range<T> {
  pub min: T,
  pub max: T,
}

impl<T> Range<T> where T: Numeric {
  pub const fn new(min: T, max: T) -> Self {
    Self { min, max }
  }

  pub fn delta(&self) -> T {
    self.max - self.min
  }

  pub fn contains(&self, other: T) -> bool {
    other >= self.min && other <= self.max
  }

  pub fn clamp(&self, value: T) -> T {
    clamp(value, self.min, self.max)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn range_should_produce_a_valid_f64_delta() {
    let range = range(-2., 2.);

    assert_eq!(4., range.delta());
  }

  #[test]
  fn range_should_produce_a_valid_i32_delta() {
    let range = range(-2, 2);

    assert_eq!(4, range.delta());
  }

  #[test]
  fn range_should_produce_a_valid_f64_range() {
    let range = range(0., 2.);

    assert!(range.contains(1.));
    assert!(!range.contains(3.));
  }

  #[test]
  fn range_should_produce_a_valid_i32_range() {
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
}
