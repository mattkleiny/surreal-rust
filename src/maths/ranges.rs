use std::ops::Sub;

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

  pub fn delta(&self) -> T
    where T: Copy + Sub<Output=T> {
    self.max - self.min
  }
}

pub fn range<T>(min: T, max: T) -> Range<T> {
  Range::new(min, max)
}