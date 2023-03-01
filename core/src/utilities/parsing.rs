/// A trait that represents a parser capable of parsing a string into an expression.
pub trait Parser {
  type Error;
  type Expression;

  /// Parses a string into an expression.
  fn parse_next(&mut self, source: &str) -> Result<Self::Expression, Self::Error>;
}

/// Represents a type that can be parsed from a string.
pub trait Parseable: Sized {
  type Error;

  /// Parses a string into an instance of the type.
  fn parse(source: &str) -> Result<Self, Self::Error>;
}
