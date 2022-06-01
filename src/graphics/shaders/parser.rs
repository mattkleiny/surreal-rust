//! Parser for a simple shading language used to make cross-platform shaders.
//!
//! This language is similar to Godot's shading language and offers a subset of features
//! from GLSL that makes it simple to build straightforward shader programs productively.
//!
//! The language itself is transpiled to GLSL via a transformation stage.

use std::{collections::VecDeque, fmt::Display};

/// Represents a unique position of a token inside of the source code.
#[derive(Copy, Clone, Debug)]
struct TokenPosition {
  index: usize,
  line: usize,
  column: usize,
}

/// Represents a single token in the source code.
///
/// A token is a span of the source code string determined to be of a
/// particular kind, ready for pasring.
#[derive(Debug)]
struct Token<'a> {
  span: &'a str,
  position: TokenPosition,
  kind: TokenKind,
}

/// Different kinds of tokens supported by the parser.
#[derive(Debug)]
enum TokenKind {
  Plus,
  Minus,
  Times,
  Divide,
  Number,
}

/// Tokenises the given source code.
fn tokenise(code: &str) -> crate::Result<VecDeque<Token>> {
  let mut tokens = VecDeque::new();
  let mut iterator = code.chars().peekable();

  let mut position = TokenPosition { index: 0, line: 1, column: 1 };

  while let Some(&character) = iterator.peek() {
    // emits a single token into the output, and advances the iterator
    let mut emit = |token, length| {
      position.index += length;
      position.column += length;

      for _ in 0..length {
        iterator.next().expect("Expected a valid token");
      }

      tokens.push_back(Token {
        span: &code[..],
        position,
        kind: token,
      });
    };

    match character {
      // single-line values
      '+' => emit(TokenKind::Plus, 1),
      '-' => emit(TokenKind::Minus, 1),
      '*' => emit(TokenKind::Times, 1),
      '/' => emit(TokenKind::Divide, 1),

      // numerical values
      '0'..='9' => {
        while let Some(true) = iterator.next().map(|c| c.is_numeric()) {
          position.index += 1;
          position.column += 1;
        }

        tokens.push_back(Token {
          span: &code[..],
          position,
          kind: TokenKind::Number,
        });
      }

      // white space and new lines
      ' ' => {
        // skip whitespace
        iterator.next();

        // track line position
        position.index += 1;
        position.column += 1;
      }
      '\n' => {
        iterator.next();

        // track line position
        position.index += 1;
        position.column = 1;
        position.line += 1;
      }
      _ => anyhow::bail!("An unexpected token was encountered: {}", character),
    }
  }

  Ok(tokens)
}

/// Represents a literal value in the language.
#[derive(Debug, Display, PartialEq)]
enum Literal {
  U32(u32),
  I32(i32),
  F32(f32),
  String(String),
}

/// Different kinds of variables supported by the language.
enum LiteralKind {
  Bool,
  Int,
  Float,
  Vec(usize),
  Mat(usize),
}

impl Display for LiteralKind {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      LiteralKind::Bool => write!(formatter, "bool"),
      LiteralKind::Int => write!(formatter, "int"),
      LiteralKind::Float => write!(formatter, "float"),
      LiteralKind::Vec(size) => write!(formatter, "vec{}", size),
      LiteralKind::Mat(size) => write!(formatter, "mat{}", size),
    }
  }
}

/// Different supported unary operators.
enum UnaryOperator {
  Neg,
  Not,
}

/// A unary expression in the shader source.
struct UnaryExpression {
  operator: UnaryOperator,
  operand: Box<Expression>,
}

/// Different supported binary operators.
enum BinaryOperator {
  Add,
  Sub,
  Mul,
  Div,
}

/// A binary expression in the shader source.
struct BinaryExpression {
  operator: BinaryOperator,
  left: Box<Expression>,
  right: Box<Expression>,
}

/// An expression in the shaders source.
enum Expression {
  Unary(UnaryExpression),
  Binary(BinaryExpression),
  Constant(Literal),
  Symbol(String),
}

impl Expression {
  fn accept(&self, visitor: &mut impl Visitor) {
    match self {
      Expression::Unary(expression) => visitor.visit_unary_expression(expression),
      Expression::Binary(expression) => visitor.visit_binary_expression(expression),
      Expression::Constant(constant) => visitor.visit_constant_expression(constant),
      Expression::Symbol(symbol) => visitor.visit_symbol_expression(symbol),
    }
  }
}

/// Declares a uniform value.
struct UniformDeclaration {
  name: String,
  kind: LiteralKind,
  default_value: Option<Literal>,
}

/// Declares an attribute value.
struct AttributeDeclaration {
  name: String,
}

/// Declares a constant value.
struct ConstantDeclaration {
  name: String,
  kind: LiteralKind,
  value: Literal,
}

/// Declares a function.
struct FunctionDeclaration {
  name: String,
  body: Vec<Statement>,
}

/// Declares a struct.
struct StructDeclaration {
  name: String,
  fields: Vec<FieldDeclaration>,
}

/// Declares a single field in a struct.
struct FieldDeclaration {
  name: String,
}

/// A statement in a shader program.
///
/// Statements can either be inside of a body (function, struct, etc),
/// or global (such as uniforms, attributes, etc).
enum Statement {
  UniformDeclaration(UniformDeclaration),
  AttributeDeclaration(AttributeDeclaration),
  ConstantDeclaration(ConstantDeclaration),
  FunctionDeclaration(FunctionDeclaration),
  StructDeclaration(StructDeclaration),
  Expression(Expression),
}

impl Statement {
  fn accept(&self, visitor: &mut impl Visitor) {
    match self {
      Statement::UniformDeclaration(declaration) => visitor.visit_uniform_decl(declaration),
      Statement::AttributeDeclaration(declaration) => visitor.visit_attribute_decl(declaration),
      Statement::ConstantDeclaration(declaration) => visitor.visit_constant_decl(declaration),
      Statement::FunctionDeclaration(declaration) => visitor.visit_function_decl(declaration),
      Statement::StructDeclaration(declaration) => visitor.visit_struct_decl(declaration),
      Statement::Expression(expression) => visitor.visit_expression(expression),
    }
  }
}

/// A visitor pattern for shader AST nodes.
///
/// The base implementations will walk down the tree recursively.
trait Visitor: Sized {
  fn visit_uniform_decl(&mut self, _declaration: &UniformDeclaration) {
    // no-op
  }

  fn visit_attribute_decl(&mut self, _declaration: &AttributeDeclaration) {
    // no-op
  }

  fn visit_constant_decl(&mut self, _declaration: &ConstantDeclaration) {
    // no-op
  }

  fn visit_function_decl(&mut self, declaration: &FunctionDeclaration) {
    for statement in &declaration.body {
      statement.accept(self);
    }
  }

  fn visit_struct_decl(&mut self, declaration: &StructDeclaration) {
    for field in &declaration.fields {
      self.visit_field_declaration(field);
    }
  }

  fn visit_field_declaration(&mut self, _declaration: &FieldDeclaration) {
    // no-op
  }

  fn visit_expression(&mut self, expression: &Expression) {
    expression.accept(self);
  }

  fn visit_unary_expression(&mut self, expression: &UnaryExpression) {
    expression.operand.accept(self);
  }

  fn visit_binary_expression(&mut self, expression: &BinaryExpression) {
    expression.left.accept(self);
    expression.right.accept(self);
  }

  fn visit_constant_expression(&mut self, _expression: &Literal) {
    // no-op
  }

  fn visit_symbol_expression(&mut self, _expression: &String) {
    // no-op
  }
}

/// Indicates a discrete shader stage in a single shader program.
///
/// Since we're abstracting over OpenGL, this usually implies a single shader kind 'vertex', 'fragment', etc.
/// However, for more advanced shader pipelines this stage could represent something else (such as lighting or SDF).
struct ShaderStage {
  shader_kind: super::ShaderKind,
  statements: Vec<Statement>,
}

/// A declaration for a whole shader program, with it's independent stages.
///
/// A top-level 'kind' parameter is used to indicate to consumers which type of shader this is, and allow it's
/// transformation for use in specific circumstances.
struct ShaderDeclaration {
  kind: String,
  stages: Vec<ShaderStage>,
}

/// Parses a shader declaration from the given raw source.
///
/// The resultant shader code is not guaranteed to be correct (not type checked or sanity checked),
/// it's expected that a later transformation stage will convert the shader and perform type checking.
fn parse(code: &str) -> crate::Result<ShaderDeclaration> {
  /// Recursive descent style parser for our simple shading language.
  struct ParseContext<'a> {
    tokens: VecDeque<Token<'a>>,
  }

  impl<'a> ParseContext<'a> {
    fn new(code: &'a str) -> crate::Result<ParseContext<'a>> {
      let tokens = tokenise(code)?;

      Ok(ParseContext { tokens })
    }

    fn parse_declaration(&mut self) -> crate::Result<ShaderDeclaration> {
      todo!()
    }

    fn next_token(&mut self) -> crate::Result<Token> {
      if let Some(token) = self.tokens.pop_front() {
        Ok(token)
      } else {
        anyhow::bail!("Unexpected end of input")
      }
    }

    fn peek_token(&mut self) -> crate::Result<&Token> {
      if self.tokens.is_empty() {
        anyhow::bail!("Unexpected end of input");
      }

      Ok(&self.tokens[0])
    }
  }

  // parse declaration and recursively consume all tokens
  let mut context = ParseContext::new(code)?;
  let declaration = context.parse_declaration()?;

  Ok(declaration)
}

/// Transpiles the given shader declaration down into standrad GLSL code.
///
/// This is a fallible process, as some minor type checking can occur.
pub fn transpile_to_glsl(code: &str) -> crate::Result<Vec<super::Shader>> {
  use std::fmt::Write;

  /// A transpiling visitor for shader programs.
  ///
  /// The output of this transpiler is a string represent the entire code for a single shader stage.
  struct Transpiler {
    output: String,
  }

  impl Visitor for Transpiler {
    fn visit_uniform_decl(&mut self, declaration: &UniformDeclaration) {
      write!(self.output, "uniform {} {};", declaration.kind, declaration.name).unwrap();
    }

    fn visit_constant_decl(&mut self, declaration: &ConstantDeclaration) {
      write!(self.output, "const {} {} = {};", declaration.kind, declaration.name, declaration.value).unwrap();
    }
  }

  // parse declaration and turn each stage into a discrete GLSL shader.
  let declaration = parse(code)?;
  let mut results = Vec::new();

  for stage in &declaration.stages {
    let mut transpiler = Transpiler { output: String::new() };

    for statement in &stage.statements {
      statement.accept(&mut transpiler);
    }

    results.push(super::Shader {
      kind: stage.shader_kind,
      code: transpiler.output,
    });
  }

  Ok(results)
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_PROGRAM: &'static str = r"
    #shader_type sprite

    uniform mat4 projectionView = mat4(1.0);
    uniform sampler2d texture;

    attribute vec2 position;
    attribute vec2 uv;
    attribute vec4 color;

    void vertex() {
      VERTEX = vec4(position, 0.0, 1.0) * projectionView;
    }

    void fragment() {
      COLOR = TEXTURE(texture, uv) * color;
    }
  ";

  #[test]
  fn it_should_tokenise_example_program() {
    tokenise(EXAMPLE_PROGRAM).expect("Failed to tokenize");
  }

  #[test]
  fn it_should_parse_and_transpile_example_program() {
    transpile_to_glsl(EXAMPLE_PROGRAM).expect("Failed to transpile");
  }
}
