use std::str::FromStr;

/// A trait that represents a parser capable of parsing a string into an expression.
pub trait Parser {
  type Error;
  type Expression;

  /// Parses a string into an expression.
  fn parse_next(&mut self, source: &str) -> Result<Self::Expression, Self::Error>;
}

/// Represents a type that can be parsed from a string.
pub trait Parseable {
  type Error;

  /// Parses a string into an instance of the type.
  fn parse(source: &str) -> Result<Self, Self::Error>;
}

/// Blaknet implementation for any type that implements the [`Parseable`] trait.
impl<P: Parseable> FromStr for P {
  type Err = P::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(s.parse())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::maths::{Degrees, Radians};

  impl Parseable for Radians {
    fn parse(&self, source: &str) {
      todo!()
    }
  }

  impl Parseable for Degrees {
    fn parse(&self, source: &str) {
      todo!()
    }
  }

  #[test]
  fn radians_should_be_parseable_from_string() {
    let angle = "1.0rads".parse::<Radians>();

    assert!(angle.is_ok());
  }

  #[test]
  fn degrees_should_be_parseable_from_string() {
    let angle = "1.0°".parse::<Degrees>();

    assert!(angle.is_ok());
  }
}
