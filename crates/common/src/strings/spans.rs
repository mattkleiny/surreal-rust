use std::{fmt::Display, str::FromStr};

/// A span of a string.
#[derive(Debug, Clone, Copy, Hash)]
pub struct StringSpan<'a> {
  pub string: &'a str,
  pub start: usize,
  pub end: usize,
}

impl<'a> StringSpan<'a> {
  /// Constructs a new [`StringSpan`] from a string and a range.
  #[inline]
  pub const fn new(string: &'a str) -> Self {
    Self {
      string,
      start: 0,
      end: string.len(),
    }
  }

  /// Is the span empty?
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.start == self.end
  }

  /// Returns the length of the span.
  #[inline]
  pub fn len(&self) -> usize {
    self.end - self.start
  }

  /// Returns the span as a string slice.
  #[inline]
  pub fn as_str(&self) -> &'a str {
    &self.string[self.start..self.end]
  }

  /// Slices the span.
  #[inline]
  pub fn slice(&self, start: usize, end: usize) -> Self {
    Self {
      string: self.string,
      start: self.start + start,
      end: self.start + end,
    }
  }

  /// Slices the span from the given start index to the end.
  #[inline]
  pub fn slice_from(&self, start: usize) -> Self {
    Self {
      string: self.string,
      start: self.start + start,
      end: self.end,
    }
  }

  /// Slices the span from the start to the given end index.
  #[inline]
  pub fn slice_to(&self, end: usize) -> Self {
    Self {
      string: self.string,
      start: self.start,
      end: self.start + end,
    }
  }

  /// Attempts to parse the span as the given value.
  #[inline]
  pub fn parse<T: FromStr>(&self) -> Option<T> {
    self.as_str().parse().ok()
  }
}

impl<'a> From<&'a str> for StringSpan<'a> {
  #[inline]
  fn from(string: &'a str) -> Self {
    Self::new(string)
  }
}

impl<'a> From<&'a String> for StringSpan<'a> {
  #[inline]
  fn from(string: &'a String) -> Self {
    Self::new(string)
  }
}

impl PartialEq<&str> for StringSpan<'_> {
  #[inline]
  fn eq(&self, other: &&str) -> bool {
    self.as_str() == *other
  }
}

impl<'a> Display for StringSpan<'a> {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Allows a type to be converted into a [`StringSpan`].
pub trait ToStringSpan {
  fn to_string_span(&self) -> StringSpan;
}

impl ToStringSpan for String {
  #[inline]
  fn to_string_span(&self) -> StringSpan {
    StringSpan::new(self)
  }
}

impl ToStringSpan for &str {
  #[inline]
  fn to_string_span(&self) -> StringSpan {
    StringSpan::new(self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_create_a_span_from_a_string() {
    let span = "Hello, World!".to_string_span();

    assert_eq!(span, "Hello, World!");
  }
}
