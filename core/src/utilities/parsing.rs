/// Defines a recursive descent parser.
///
/// A recursive descent parser is a parser that is able to parse a string of
/// tokens into an expression. The parser is recursive because it calls itself
/// to parse sub-expressions. The parser is descent because it parses the
/// expression from the top down.
pub trait Parser {
  type Expression;
  type Error;
}

pub trait Parseable {
  type Parser;

  fn parse(&self, source: &str);
}
