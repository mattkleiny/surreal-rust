use super::*;

/// Builds a range between the given values.
pub const fn range<T: Scalar>(min: T, max: T) -> Range<T> {
  Range::new(min, max)
}

/// An inclusive range that spans the given (min, max) values.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Range<T> {
  pub min: T,
  pub max: T,
}

impl<T: Scalar> Range<T> {
  pub const fn new(min: T, max: T) -> Self {
    Self { min, max }
  }

  #[inline]
  pub fn delta(&self) -> T {
    self.max - self.min
  }

  #[inline]
  pub fn contains(&self, other: T) -> bool {
    other >= self.min && other <= self.max
  }

  #[inline]
  pub fn clamp(&self, value: T) -> T {
    value.clamp(self.min, self.max)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_produce_a_valid_f64_delta() {
    let range = range(-2., 2.);

    assert_eq!(4., range.delta());
  }

  #[test]
  fn test_produce_a_valid_i32_delta() {
    let range = range(-2, 2);

    assert_eq!(4, range.delta());
  }

  #[test]
  fn test_produce_a_valid_f64_range() {
    let range = range(0., 2.);

    assert!(range.contains(1.));
    assert!(!range.contains(3.));
  }

  #[test]
  fn test_produce_a_valid_i32_range() {
    let range = range(-2, 5);

    assert!(range.contains(-1));
    assert!(!range.contains(6));
  }

  #[test]
  fn test_clamp_a_valid_range() {
    let range = range(-2, 5);

    assert_eq!(-2, range.clamp(-100));
    assert_eq!(5, range.clamp(100));
  }
}
