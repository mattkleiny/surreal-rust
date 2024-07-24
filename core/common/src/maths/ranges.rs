use super::*;

/// Builds a range between the given values.
pub const fn range<T: Scalar>(min: T, max: T) -> Range<T> {
  Range::new(min, max)
}

/// An inclusive range that spans the given (min, max) values.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Range<T> {
  pub min: T,
  pub max: T,
}

impl<T: Scalar> Range<T> {
  /// Creates a new range with the given min and max values.
  pub const fn new(min: T, max: T) -> Self {
    Self { min, max }
  }

  /// Computes the delta between the min and max values.
  #[inline]
  pub fn delta(&self) -> T {
    self.max - self.min
  }

  /// Determines if the given value is within the range.
  #[inline]
  pub fn contains(&self, other: T) -> bool {
    other >= self.min && other <= self.max
  }

  /// Clamps the given value to the range.
  #[inline]
  pub fn clamp(&self, value: T) -> T {
    value.clamp(self.min, self.max)
  }
}

impl<T: Lerp> Lerp for Range<T> {
  #[inline]
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    Self {
      min: T::lerp(a.min, b.min, t),
      max: T::lerp(a.max, b.max, t),
    }
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

  #[test]
  fn test_range_should_be_interpolatable() {
    let range1 = range(0.0, 10.0);
    let range2 = range(10.0, 20.0);

    let interpolated = Range::lerp(range1, range2, 0.5);

    assert_eq!(5.0, interpolated.min);
    assert_eq!(15.0, interpolated.max);
  }
}
