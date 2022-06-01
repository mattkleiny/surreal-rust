use std::rc::Rc;

use crate::assets::{Asset, AssetContext, AssetLoader};
use crate::io::AsVirtualPath;
use crate::maths::{Matrix2x2, Matrix3x3, Matrix4x4, Vector2, Vector3, Vector4};

use super::*;

/// Different types of shaders supported by the engine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ShaderKind {
  Vertex,
  Fragment,
}

/// Defines a single shader kernel in a shader program.
pub struct Shader {
  pub kind: ShaderKind,
  pub code: String,
}

/// Representation of a single value that can be used in a shader.
#[derive(Clone)]
pub enum ShaderUniform {
  Integer(u32),
  Floating(f32),
  Point2(Vector2<i32>),
  Point3(Vector3<i32>),
  Point4(Vector4<i32>),
  Vector2(Vector2<f32>),
  Vector3(Vector3<f32>),
  Vector4(Vector4<f32>),
  Matrix2x2(Matrix2x2<f32>),
  Matrix3x3(Matrix3x3<f32>),
  Matrix4x4(Matrix4x4<f32>),
  Texture(Texture, usize, Option<TextureSampler>),
}

/// Represents a single compiled shader program.
#[derive(Clone)]
pub struct ShaderProgram {
  state: Rc<ShaderProgramState>,
}

/// The internal state for a [`ShaderProgram`] .
struct ShaderProgramState {
  server: GraphicsServer,
  handle: GraphicsHandle,
}

impl ShaderProgram {
  /// Creates a new blank [`ShaderProgram`] on the GPU.
  pub fn new(server: &GraphicsServer) -> Self {
    Self {
      state: Rc::new(ShaderProgramState {
        server: server.clone(),
        handle: server.create_shader(),
      }),
    }
  }

  /// Loads a [`ShaderProgram`] from the given raw 'glsl' code.
  pub fn from_glsl(server: &GraphicsServer, code: &str) -> crate::Result<Self> {
    let program = Self::new(server);

    program.load_glsl(code)?;

    Ok(program)
  }

  /// Loads a [`ShaderProgram`] from the given raw 'shade' code.
  pub fn from_shade(server: &GraphicsServer, code: &str) -> crate::Result<Self> {
    let program = Self::new(server);

    program.load_shade(code)?;

    Ok(program)
  }

  /// Retrieves the binding location of the given shader uniform in the underlying program.
  pub fn get_uniform_location(&self, name: &str) -> Option<usize> {
    let server = &self.state.server;

    server.get_shader_uniform_location(self.state.handle, name)
  }

  /// Sets the given uniform value in the underlying program.
  pub fn set_uniform(&self, location: usize, value: &ShaderUniform) {
    let server = &self.state.server;

    server.set_shader_uniform(self.state.handle, location, value);
  }

  /// Reloads the [`ShaderProgram`] from the given 'glsl' program code.
  pub fn load_glsl(&self, text: &str) -> crate::Result<()> {
    let server = &self.state.server;
    let shaders = parse_glsl_source(&text);

    server.link_shaders(self.state.handle, shaders)?;

    Ok(())
  }

  /// Reloads the [`ShaderProgram`] from the given 'shade' program code.
  pub fn load_shade(&self, text: &str) -> crate::Result<()> {
    let server = &self.state.server;
    let shaders = parser::transpile_to_glsl(text)?;

    server.link_shaders(self.state.handle, shaders)?;

    Ok(())
  }

  /// Reloads the [`ShaderProgram`] from a file at the given virtual path.
  pub fn load_from_path(&self, path: impl AsVirtualPath) -> crate::Result<()> {
    let path = path.as_virtual_path();
    let source_code = path.read_all_text()?;

    if path.extension().ends_with("glsl") {
      self.load_glsl(&source_code)?;
    } else {
      self.load_shade(&source_code)?;
    }

    Ok(())
  }
}

impl GraphicsResource for ShaderProgram {
  /// Retrieves the handle for the given [`ShaderProgram`].
  fn handle(&self) -> GraphicsHandle {
    self.state.handle
  }
}

impl Drop for ShaderProgramState {
  /// Deletes the [`ShaderProgram`] from the GPU.
  fn drop(&mut self) {
    self.server.delete_shader(self.handle);
  }
}

/// An [`AssetLoader`] for shader programs
pub struct ShaderProgramLoader {
  pub server: GraphicsServer,
}

impl Asset for ShaderProgram {
  type Loader = ShaderProgramLoader;
}

impl AssetLoader<ShaderProgram> for ShaderProgramLoader {
  fn load(&self, context: &AssetContext) -> crate::Result<ShaderProgram> {
    let program = ShaderProgram::new(&self.server);
    let source_code = context.path.read_all_text()?;

    if context.path.extension().ends_with("glsl") {
      program.load_glsl(&source_code)?;
    } else {
      program.load_shade(&source_code)?;
    }

    Ok(program)
  }
}

/// Implements uniform value transformation for common shader uniforms.
macro_rules! implement_uniform {
  ($type:ty, $value:ident) => {
    impl From<$type> for ShaderUniform {
      fn from(value: $type) -> Self {
        ShaderUniform::$value(value.clone())
      }
    }
  };
}

implement_uniform!(u32, Integer);
implement_uniform!(f32, Floating);
implement_uniform!(Vector2<i32>, Point2);
implement_uniform!(Vector3<i32>, Point3);
implement_uniform!(Vector4<i32>, Point4);
implement_uniform!(Vector2<f32>, Vector2);
implement_uniform!(Vector3<f32>, Vector3);
implement_uniform!(Vector4<f32>, Vector4);
implement_uniform!(&Matrix2x2<f32>, Matrix2x2);
implement_uniform!(&Matrix3x3<f32>, Matrix3x3);
implement_uniform!(&Matrix4x4<f32>, Matrix4x4);

impl From<&Texture> for ShaderUniform {
  fn from(texture: &Texture) -> Self {
    ShaderUniform::Texture(texture.clone(), 0, None)
  }
}

/// Parses the given raw GLSL source and performs some basic pre-processing.
///
/// Allows for the following basic transformations:
///
/// * Multiple shader types per file (separated with #shader_type directives).
/// * Shared code amongst each shader definition by placing it prior to the #shader_type directives.
fn parse_glsl_source(source: &str) -> Vec<Shader> {
  let mut result = Vec::with_capacity(2); // usually 2 shaders per file
  let mut shared_code = String::new();

  for line in source.lines() {
    if line.trim().starts_with("#shader_type") {
      // determine shader type
      let kind = match line.split_whitespace().nth(1) {
        Some("vertex") => ShaderKind::Vertex,
        Some("fragment") => ShaderKind::Fragment,
        _ => continue,
      };

      result.push(Shader {
        kind,
        code: shared_code.clone(),
      });
    } else if let Some(shader) = result.last_mut() {
      // build up the active shader unit
      shader.code.push_str(line);
      shader.code.push('\n');
    } else {
      // build up the shared code unit
      shared_code.push_str(line);
      shared_code.push('\n');
    };
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_should_process_valid_program() {
    let result = parse_glsl_source(
      r"
      #version 330 core

      // shared code
      uniform mat4 u_projectionView;
      uniform vec2 u_resolution;
      uniform vec4 u_color;

      #shader_type vertex

      layout(location = 0) in vec2 a_position;
      layout(location = 1) in vec2 a_tex_coord;
      layout(location = 2) in vec4 a_color;

      out vec2 v_uv;
      out vec4 v_color;

      void main() {
        v_uv    = a_uv;
        v_color = a_color * u_color;

        gl_Position = vec4(a_position, 0.0, 1.0) * u_projectionView;
      }

      #shader_type fragment

      uniform sampler2d u_texture;

      in vec2 v_uv;
      in vec4 v_color;

      void main() {
        gl_FragColor = texture(u_texture, v_uv) * v_color;
      }
    ",
    );

    assert_eq!(result.len(), 2);
    assert_eq!(result[0].kind, ShaderKind::Vertex);
    assert!(result[0].code.trim().starts_with("#version 330 core"));
    assert!(result[0].code.contains("gl_Position"));
    assert_eq!(result[1].kind, ShaderKind::Fragment);
    assert!(result[1].code.trim().starts_with("#version 330 core"));
    assert!(result[1].code.contains("gl_Frag"));
  }
}

mod parser {
  //! Parser for a simple shading language used to make cross-platform shaders.
  //!
  //! This language is similar to Godot's shading language and offers a subset of features
  //! from GLSL that makes it simple to build straightforward shader programs productively.
  //!
  //! The language itself is transpiled to GLSL via a transformation stage.

  use std::collections::VecDeque;

  #[derive(Copy, Clone, Debug)]
  struct TokenPosition {
    index: usize,
    line: usize,
    column: usize,
  }

  #[derive(Debug)]
  struct Token<'a> {
    span: &'a str,
    position: TokenPosition,
    kind: TokenKind,
  }

  #[derive(Debug)]
  enum TokenKind {
    Plus,
    Minus,
    Times,
    Divide,
    Number,
  }

  enum UnaryOp {
    Neg,
    Not,
  }

  enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
  }

  enum Literal {
    U32(u32),
    I32(i32),
    F32(f32),
    String(String),
  }

  struct UnaryExpr {
    operator: UnaryOp,
    operand: Box<Expression>,
  }

  struct BinaryExpr {
    operator: BinaryOp,
    left: Box<Expression>,
    right: Box<Expression>,
  }

  enum Expression {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
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

  struct UniformDecl {
    name: String,
  }

  struct AttributeDecl {
    name: String,
  }

  struct ConstantDecl {
    name: String,
  }

  struct FunctionDecl {
    name: String,
    body: Vec<Statement>,
  }

  enum Statement {
    UniformDecl(UniformDecl),
    AttributeDecl(AttributeDecl),
    ConstantDecl(ConstantDecl),
    FunctionDecl(FunctionDecl),
    Expression(Expression),
  }

  impl Statement {
    fn accept(&self, visitor: &mut impl Visitor) {
      match self {
        Statement::UniformDecl(declaration) => visitor.visit_uniform_decl(declaration),
        Statement::AttributeDecl(declaration) => visitor.visit_attribute_decl(declaration),
        Statement::ConstantDecl(declaration) => visitor.visit_constant_decl(declaration),
        Statement::FunctionDecl(function) => visitor.visit_function_declaration(function),
        Statement::Expression(expression) => visitor.visit_expression_statement(expression),
      }
    }
  }

  /// A visitor pattern for shader AST nodes.
  ///
  /// The base implementation will walk down the tree recursively.
  trait Visitor: Sized {
    fn visit_uniform_decl(&mut self, _declaration: &UniformDecl) {
      // no-op
    }

    fn visit_attribute_decl(&mut self, _declaration: &AttributeDecl) {
      // no-op
    }

    fn visit_constant_decl(&mut self, _declaration: &ConstantDecl) {
      // no-op
    }

    fn visit_function_declaration(&mut self, function: &FunctionDecl) {
      for statement in &function.body {
        statement.accept(self);
      }
    }

    fn visit_expression_statement(&mut self, expression: &Expression) {
      expression.accept(self);
    }

    fn visit_unary_expression(&mut self, expression: &UnaryExpr) {
      expression.operand.accept(self);
    }

    fn visit_binary_expression(&mut self, expression: &BinaryExpr) {
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

  /// Tokenizes the given shader source code.
  fn tokenize(code: &str) -> crate::Result<VecDeque<Token>> {
    let mut tokens = VecDeque::new();
    let mut iterator = code.chars().peekable();

    let mut position = TokenPosition {
      index: 0,
      line: 1,
      column: 1,
    };

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
        let tokens = tokenize(code)?;

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
  /// This is a fallible process, as some minor type checking will occur.
  pub fn transpile_to_glsl(code: &str) -> crate::Result<Vec<super::Shader>> {
    /// A transpiling visitor for shader programs.
    struct Transpiler {
      output: String,
    }

    impl Visitor for Transpiler {
      fn visit_uniform_decl(&mut self, declaration: &UniformDecl) {
        self.output += "uniform ";
        self.output += &declaration.name;
        self.output += "\n";
      }
    }

    // parse declaration and turn each stage into a discrete GLSL shader.
    let declaration = parse(code)?;
    let mut results = Vec::new();

    for stage in &declaration.stages {
      let mut transpiler = Transpiler {
        output: String::new(),
      };

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
    fn it_should_tokenize_example_program() {
      // tokenize(EXAMPLE_PROGRAM).expect("Failed to tokenize");

      let tokens = tokenize("1 + 2 / 3 * 4").expect("Failed to parse simple expression");

      assert_eq!(tokens.len(), 7);
    }

    #[test]
    fn it_should_parse_and_transpile_example_program() {
      transpile_to_glsl(EXAMPLE_PROGRAM).expect("Failed to transpile");
    }
  }
}
