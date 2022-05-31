use std::rc::Rc;

use crate::assets::{Asset, AssetContext, AssetLoader};
use crate::graphics::{GraphicsServer, TextureSampler};
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

  /// Loads a [`ShaderProgram`] from the given raw code.
  pub fn from_string(server: &GraphicsServer, code: &str) -> crate::Result<Self> {
    let program = Self::new(server);

    program.load_from_string(code)?;

    Ok(program)
  }

  /// Retrieves the binding location of the given shader uniform in the underlying program.
  pub fn get_uniform_location(&self, name: &str) -> Option<usize> {
    self.state.server.get_shader_uniform_location(self.state.handle, name)
  }

  /// Sets the given uniform value in the underlying program.
  pub fn set_uniform(&self, location: usize, value: &ShaderUniform) {
    self.state.server.set_shader_uniform(self.state.handle, location, value);
  }

  /// Reloads the [`ShaderProgram`] from the given text.
  pub fn load_from_string(&self, text: &str) -> crate::Result<()> {
    let shaders = parse_glsl_source(&text);

    self.state.server.link_shaders(self.state.handle, shaders)?;

    Ok(())
  }

  /// Reloads the [`ShaderProgram`] from a file at the given virtual path.
  pub fn load_from_path(&self, path: impl AsVirtualPath) -> crate::Result<()> {
    let path = path.as_virtual_path();
    let source_code = path.read_all_text()?;

    if path.extension().ends_with("glsl") {
      // support GLSL shaders natively
      let shaders = parse_glsl_source(&source_code);

      self.state.server.link_shaders(self.state.handle, shaders)?;
    } else {
      // otherwise parse our custom shading language
      let shaders = parser::transpile_to_glsl(&source_code)?;

      self.state.server.link_shaders(self.state.handle, shaders)?;
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
    let code = context.path.read_all_text()?;
    let program = ShaderProgram::from_string(&self.server, &code)?;

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

      result.push(Shader { kind, code: shared_code.clone() });
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

mod parser {
  //! Parser for a simple shading language used to make cross-platform shaders.
  //! 
  //! This language is similar to Godot's shading language and offers a subset of features
  //! from GLSL that makes it simple to build straightforward shader programs productively.
  //! 
  //! The language itself is transpiled to GLSL via a transformation stage.

  struct TokenPosition {
    line: usize,
    column: usize,
  }

  struct Token<'a> {
    span: &'a str,
    position: TokenPosition,
    kind: TokenKind,
  }

  enum TokenKind {
    Number,
  }

  enum UnaryOperator {
    Neg,
    Not,
  }

  enum BinaryOperator {
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

  struct UnaryExpression {
    operator: UnaryOperator,
    operand: Box<Expression>,
  }

  struct BinaryExpression {
    operator: BinaryOperator,
    left: Box<Expression>,
    right: Box<Expression>,
  }

  enum Expression {
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    Constant(Literal),
    Symbol(String),
  }

  impl Expression {
    fn accept(&self, visitor: &mut impl Visitor) {
      match self {
        Expression::Unary(e) => visitor.visit_unary_expression(e),
        Expression::Binary(e) => visitor.visit_binary_expression(e),
        Expression::Constant(e) => visitor.visit_constant_expression(e),
        Expression::Symbol(e) => visitor.visit_symbol_expression(e),
      }
    }
  }

  struct UniformDeclaration {
    name: String,
  }

  struct AttributeDeclaration {
    name: String,
  }

  enum Statement {
    UniformDeclaration(UniformDeclaration),
    AttributeDeclaration(AttributeDeclaration),
    Expression(Expression),
  }

  impl Statement {
    fn accept(&self, visitor: &mut impl Visitor) {
      match self {
        Statement::UniformDeclaration(s) => visitor.visit_uniform_declaration(s),
        Statement::AttributeDeclaration(s) => visitor.visit_attribute_declaration(s),
        Statement::Expression(s) => visitor.visit_expression_statement(s),
      }
    }
  }

  trait Visitor {
    fn visit_uniform_declaration(&mut self, statement: &UniformDeclaration) {}
    fn visit_attribute_declaration(&mut self, statement: &AttributeDeclaration) {}
    fn visit_expression_statement(&mut self, statement: &Expression) {}
    fn visit_unary_expression(&mut self, expression: &UnaryExpression) {}
    fn visit_binary_expression(&mut self, expression: &BinaryExpression) {}
    fn visit_constant_expression(&mut self, expression: &Literal) {}
    fn visit_symbol_expression(&mut self, expression: &String) {}
  }

  struct ShaderStage {
    shader_kind: super::ShaderKind,
    statements: Vec<Statement>,
  }

  struct ShaderDeclaration {
    stages: Vec<ShaderStage>,
  }

  /// Tokenizes the given shader source code.
  fn tokenize(code: &str) -> crate::Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut iterator = code.chars().peekable();

    let mut line = 1;
    let mut column = 1;

    while let Some(&character) = iterator.peek() {
      match character {
        '0'..='9' => {
          iterator.next().expect("Expected peekable character");

          tokens.push(Token { 
            span: &code[..], // TODO: put the right span, here
            position: TokenPosition { line, column },
            kind: TokenKind::Number,
          });
        },
        '+' | '-' | '*' | '/' => {
          todo!()
        },
        ' ' => {
          // skip whitespace
          iterator.next();
        },
        '\n' => {
          column = 1;
          line += 1;
        },
        _ => anyhow::bail!("An unexpected token was encountered: {}", character),
      }
      
      column += 1;
    }

    todo!()
  }

  /// Parses a shader declaration from the given raw source.
  /// 
  /// The resultant shader code is not guaranteed to be correct (not type checked or sanity checked),
  /// it's expected that a later transformation stage will convert the shader and perform type checking.
  fn parse(code: &str) -> crate::Result<ShaderDeclaration> {
    /// Recursive descent style parser for our simple shading language.
    struct ParseContext<'a> {
      tokens: Vec<Token<'a>>,
    }

    impl<'a> ParseContext<'a> {
      fn new(code: &'a str) -> crate::Result<ParseContext<'a>> {
        let tokens = tokenize(code)?;

        Ok(ParseContext { tokens })
      }

      fn parse_declaration(&mut self) -> crate::Result<ShaderDeclaration> {
        todo!()
      }

      fn parse_stage(&mut self) -> crate::Result<ShaderStage> {
        todo!()
      }

      fn next_token(&mut self) -> crate::Result<Token> {
        if self.tokens.is_empty() {
          anyhow::bail!("Unexpected end of input");
        }

        Ok(self.tokens.remove(0))
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

    impl Visitor for Transpiler {}

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

    #[test]
    fn tokenize_should_recognize_basic_program() {
      let tokens = tokenize("1 + 2 / 3").expect("Failed to tokenize");

      assert_eq!(tokens.len(), 5);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_should_process_valid_program() {
    let result = parse_glsl_source(r"
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
    ");

    assert_eq!(result.len(), 2);
    assert_eq!(result[0].kind, ShaderKind::Vertex);
    assert!(result[0].code.trim().starts_with("#version 330 core"));
    assert!(result[0].code.contains("gl_Position"));
    assert_eq!(result[1].kind, ShaderKind::Fragment);
    assert!(result[1].code.trim().starts_with("#version 330 core"));
    assert!(result[1].code.contains("gl_Frag"));
  }
}