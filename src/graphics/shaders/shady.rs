//! A compilable shader language that permits us to write shader programs
//! once and theoretically run them anywhere.
//!
//! The compilation backend for a program is the SPIR-V format (via the `rspirv` crate).
//! This allows us to target any graphics platform with the same shader language, and
//! compile changes online and via hot-reloading.
//!
//! An extension of this might also allow constructions of shaders via a shader graph.

/// A high-level shady program.
#[derive(Clone, Debug)]
pub struct ShadyProgram {
  pub kind: ProgramKind,
  pub statements: Vec<Statement>,
}

impl ShadyProgram {
  pub fn parse(raw: impl AsRef<str>) -> ParseResult<ShadyProgram> {
    Parser::parse(raw.as_ref())
  }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ConstantType {
  Int,
  Float,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RuntimeType {
  Void,
  Bool,
  Vec2,
  Vec3,
  Vec4,
  Int,
  Float,
  Sampler2D,
  Sampler3D,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operator {
  Equal,
  NotEqual,
  Less,
  LessEqual,
  Greater,
  GreaterEqual,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Precision {
  Default,
  Low,
  Medium,
  High,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Interpolation {
  Flat,
  Smooth,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ProgramKind {
  Sprite,
  Mesh,
  Compute,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MethodBinding {
  VertexBody,
  FragmentBody,
}

#[derive(Clone, Debug)]
pub enum Token {
  Unknown,
  Identifier,
  True,
  False,
  Constant(ConstantType),
  Type(RuntimeType),
}

#[derive(Clone, Debug)]
pub enum Expression {
  Unknown,
  Empty,
  Operator {
    name: String,
    return_type: RuntimeType,
    precision: Precision,
    operator: Operator,
    arguments: Vec<Expression>,
  },
  Variable {
    name: String,
    is_const: bool,
  },
}

#[derive(Clone, Debug)]
pub enum Statement {
  Unknown,
  Empty,
  KindSpecification {
    kind: ProgramKind,
    version: u16,
  },
  MethodDefinition {
    name: String,
    binding: MethodBinding,
    return_type: RuntimeType,
    arguments: Vec<Expression>,
  },
}

/// Visitation pattern for the shady AST.
pub trait Visitor<T> {
  fn visit_statement(&mut self, statement: &Statement) -> T;
  fn visit_expression(&mut self, expression: &Expression) -> T;
}

/// A tokenizer for Shady programs.
///
/// Converts strings into Shady `Token`s.
struct Tokenizer<'a> {
  input: &'a str,
  row: usize,
  column: usize,
}

impl<'a> Tokenizer<'a> {
  pub fn tokenize(input: &'a str) -> ParseResult<Vec<Token>> {
    let mut tokenizer = Self::new(input);
    let tokens = tokenizer.extract_tokens();

    Ok(tokens)
  }

  fn new(input: &'a str) -> Self {
    Self {
      input,
      row: 0,
      column: 0,
    }
  }

  fn extract_tokens(&mut self) -> Vec<Token> {
    unimplemented!();
  }
}

/// A parser for Shady programs.
///
/// Turns raw text into the shady AST.
#[derive(Clone, Debug)]
struct Parser {
  tokens: Vec<Token>,
  position: usize,
}

impl Parser {
  /// Parses a Shady program from the given string representation.
  pub fn parse(raw: &str) -> ParseResult<ShadyProgram> {
    let tokens = Tokenizer::tokenize(raw)?;
    let mut parser = Self::new(tokens);

    parser.parse_tokens(raw.as_ref())
  }

  fn new(tokens: Vec<Token>) -> Self {
    Self {
      tokens,
      position: 0,
    }
  }

  fn parse_tokens(&mut self, input: &str) -> ParseResult<ShadyProgram> {
    unimplemented!()
  }
}

/// Possible errors when parsing.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
  InvalidSymbol,
  NoKindSpecified,
}

type ParseResult<T> = std::result::Result<T, ParseError>;

#[cfg(test)]
mod tests {
  use super::*;

  # [test]
  fn it_should_parse_a_simple_program() {
    ShadyProgram::parse("#kind sprite").expect("Failed to parse simple program!");
  }
}