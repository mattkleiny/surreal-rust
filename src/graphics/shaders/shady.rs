//! A compilable shader language that permits us to write shader programs
//! once and theoretically run them anywhere.
//!
//! The compilation backend for a program is the SPIR-V format (via the `rspirv` crate).
//! This allows us to target any graphics platform with the same shader language, and
//! compile changes online and via hot-reloading.
//!
//! An extension of this might also allow constructions of shaders via a shader graph.

type ParseResult<T> = std::result::Result<T, ParseError>;

/// A high-level shady program.
#[derive(Debug)]
pub struct ShadyProgram {
  pub kind: ProgramKind,
  pub modules: Vec<Module>,
}

impl ShadyProgram {
  pub fn parse(raw: impl AsRef<str>) -> ParseResult<ShadyProgram> {
    Parser::parse(raw.as_ref())
  }

  pub fn accept(&self, visitor: &mut impl Visitor) {
    for module in &self.modules {
      visitor.visit_module(module);
    }
  }
}

#[derive(Debug)]
pub enum ConstantType {
  Int,
  Float,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Operator {
  Equal,
  NotEqual,
  Less,
  LessEqual,
  Greater,
  GreaterEqual,
}

#[derive(Debug)]
pub enum Precision {
  Default,
  Low,
  Medium,
  High,
}

#[derive(Debug)]
pub enum Interpolation {
  Flat,
  Smooth,
}

#[derive(Debug)]
pub enum ProgramKind {
  Sprite,
  Mesh,
  Compute,
}

#[derive(Debug)]
pub enum IntrinsicKind {
  VertexOutput,
  FragmentOutput,
}

#[derive(Debug)]
pub enum MethodBinding {
  VertexBody,
  FragmentBody,
}

#[derive(Debug)]
pub enum Token {
  Unknown,
  Identifier,
  True,
  False,
  Constant(ConstantType),
  Type(RuntimeType),
}

#[derive(Debug)]
pub enum Expression {
  Unknown,
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
  FunctionCall {
    name: String,
    parameters: Vec<Expression>,
  },
  Intrinsic {
    kind: IntrinsicKind,
    parameters: Vec<Expression>,
  },
}

#[derive(Debug)]
pub enum Statement {
  Unknown,
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

#[derive(Debug)]
pub enum Module {
  Shared {
    statements: Vec<Statement>,
  },
  Shader {
    kind: ProgramKind,
    statements: Vec<Statement>,
  },
}

/// Visitation pattern for the shady AST.
pub trait Visitor {
  fn visit_module(&mut self, module: &Module);
  fn visit_statement(&mut self, statement: &Statement);
  fn visit_expression(&mut self, expression: &Expression);
}

/// A tokenizer for Shady programs.
///
/// Converts strings into Shady `Token`s.
struct Tokenizer<'a> {
  input: &'a str,
  index: usize,
}

impl<'a> Tokenizer<'a> {
  pub fn tokenize(input: &'a str) -> ParseResult<Vec<Token>> {
    Self::new(input).extract_tokens()
  }

  fn new(input: &'a str) -> Self {
    Self {
      input,
      index: 0,
    }
  }

  fn extract_tokens(&mut self) -> ParseResult<Vec<Token>> {
    let mut result = Vec::new();

    while let Some(token) = self.extract_token()? {
      result.push(token);
    }

    Ok(result)
  }

  fn extract_token(&mut self) -> ParseResult<Option<Token>> {
    unimplemented!()
  }
}

/// A parser for Shady programs.
///
/// Turns raw text into the shady AST.
struct Parser {
  tokens: Vec<Token>,
  position: usize,
}

impl Parser {
  /// Parses a Shady program from the given string representation.
  pub fn parse(raw: &str) -> ParseResult<ShadyProgram> {
    let tokens = Tokenizer::tokenize(raw)?;
    let mut parser = Self::new(tokens);

    parser.parse_tokens()
  }

  fn new(tokens: Vec<Token>) -> Self {
    Self {
      tokens,
      position: 0,
    }
  }

  fn parse_tokens(&mut self) -> ParseResult<ShadyProgram> {
    unimplemented!()
  }
}

/// Possible errors when parsing.
#[derive(Debug)]
pub enum ParseError {
  InvalidModule,
  InvalidStatement,
  InvalidSymbol,
  NoKindSpecified,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parser_should_parse_a_simple_program() {
    const PROGRAM: &'static str = r"
      #kind sprite

      void vertex() {
        POSITION += vec2(0,TIME);
      }

      void fragment() {
        COLOR = rgba(1,1,1,1);
      }
    ";

    ShadyProgram::parse(PROGRAM)
        .expect("Failed to parse simple program!");
  }
}