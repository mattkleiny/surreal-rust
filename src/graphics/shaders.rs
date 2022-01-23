pub use compiler::*;
pub use parser::*;

use crate::graphics::GraphicsHandle;

/// Different types fo shaders supported by the engine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ShaderKind {
  Vertex,
  Fragment,
}

/// Represents a single compiled shader program.
#[derive(Debug, Eq, PartialEq)]
pub struct ShaderProgram {
  handle: GraphicsHandle,
}

impl ShaderProgram {
  pub fn new() -> Self {
    todo!()
  }
}

mod parser {
  /// Represents the result of a fallible execution in the shader parser.
  pub type ParseResult<T> = anyhow::Result<T>;

  /// A parser for shader programs.
  pub trait ShaderParser {
    fn parse_raw(&mut self, raw: &str) -> ParseResult<ShaderDeclaration>;
  }

  /// A declaration of a shader program, in AST form.
  pub struct ShaderDeclaration {
    source_path: String,
    compilation_unit: ShaderCompilationUnit,
  }

  /// A single compilation unit in a shader program.
  pub struct ShaderCompilationUnit {
    globals: Vec<GlobalDeclaration>,
    functions: Vec<FunctionDeclaration>,
    stages: Vec<ShaderStage>,
  }

  /// A global top-level declaration for the entire shader program.
  pub enum GlobalDeclaration {
    Include,
    Uniform,
    Varying,
    Constant,
  }

  /// Different stages of the shader pipeline.
  pub enum ShaderStage {
    Vertex,
    Fragment,
    Geometry,
  }

  /// Declares a single function.
  pub struct FunctionDeclaration {}
}

mod compiler {
  /// A compiler for parsed shaders that converts programs into executable code on the platform.
  pub trait ShaderCompiler {}
}
