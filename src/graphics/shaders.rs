/// Different types fo shaders supported by the engine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ShaderKind {
  Vertex,
  Fragment,
}

/// A managed ID for OpenGL shaders.
struct ShaderHandle(u32);

impl ShaderHandle {
  pub fn new(kind: ShaderKind) -> Self {
    Self(unsafe {
      gl::CreateShader(match kind {
        ShaderKind::Vertex => gl::VERTEX_SHADER,
        ShaderKind::Fragment => gl::FRAGMENT_SHADER,
      })
    })
  }
}

impl Drop for ShaderHandle {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteShader(self.0);
    }
  }
}

/// Represents a single raw shader program.
pub struct Shader {
  handle: ShaderHandle,
  kind: ShaderKind,
}

impl Shader {
  pub fn new(kind: ShaderKind, source: &impl ToString) -> Self {
    Self {
      handle: ShaderHandle::new(kind),
      kind,
    }
  }
}

/// A managed ID for OpenGL shader programs.
#[derive(Debug, Eq, PartialEq)]
struct ProgramHandle(u32);

impl ProgramHandle {
  pub fn new() -> Self {
    Self(unsafe { gl::CreateProgram() })
  }
}

impl Drop for ProgramHandle {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteProgram(self.0);
    }
  }
}

/// Represents a single compiled shader program.
#[derive(Debug, Eq, PartialEq)]
pub struct Program {
  handle: ProgramHandle,
}

impl Program {
  pub fn new() -> Self {
    Self {
      handle: ProgramHandle::new(),
    }
  }

  pub fn link(&mut self, shaders: &[Shader]) {
    unsafe {
      for shader in shaders {
        gl::AttachShader(self.handle.0, shader.handle.0);
      }

      gl::LinkProgram(self.handle.0);
    }
  }
}

#[cfg(feature = "shady")]
pub mod shady {
  //! A compilable shader language that permits us to write shader programs
  //! once and theoretically run them anywhere.
  //!
  //! The compilation backend for a program is the SPIR-V format (via the `rspirv` crate).
  //! This allows us to target any graphics platform with the same shader language, and
  //! compile changes online and via hot-reloading.
  //!
  //! An extension of this might also allow constructions of shaders via a shader graph.

  #[derive(Clone, Debug)]
  pub struct ShadyProgram {
    kind: ProgramKind,
    statements: Vec<Statement>,
  }

  #[derive(Copy, Clone, Debug, Eq, PartialEq)]
  enum ConstantType {
    Int,
    Float,
  }

  #[derive(Copy, Clone, Debug, Eq, PartialEq)]
  enum RuntimeType {
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
  enum Operator {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
  }

  #[derive(Copy, Clone, Debug, Eq, PartialEq)]
  enum Precision {
    Default,
    Low,
    Medium,
    High,
  }

  #[derive(Copy, Clone, Debug, Eq, PartialEq)]
  enum Interpolation {
    Flat,
    Smooth,
  }

  #[derive(Copy, Clone, Debug, Eq, PartialEq)]
  enum ProgramKind {
    Sprite,
    Mesh,
    Compute,
  }

  #[derive(Copy, Clone, Debug, Eq, PartialEq)]
  enum MethodBinding {
    VertexBody,
    FragmentBody,
  }

  #[derive(Clone, Debug)]
  enum Token {
    Unknown,
    Identifier,
    True,
    False,
    Constant(ConstantType),
    Type(RuntimeType),
  }

  #[derive(Clone, Debug)]
  enum Expression {
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
  enum Statement {
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

  /// A parser for Shady programs.
  ///
  /// Turns raw text into the shady AST.
  #[derive(Clone, Debug)]
  pub struct Parser {
    position: usize,
    last_token: Token,
  }

  /// Possible errors when parsing.
  #[derive(Copy, Clone, Debug, Eq, PartialEq)]
  pub enum ParseError {}

  impl Parser {
    /// Parses a Shady program from the given string representation.
    pub fn parse(raw: impl AsRef<str>) -> Result<ShadyProgram, ParseError> {
      let mut parser = Self {
        position: 0,
        last_token: Token::Unknown,
      };

      parser.parse_string(raw.as_ref())
    }

    /// Parses the given raw shady program into it's AST representation.
    ///
    /// Failures are emitted as compilation errors.
    pub const fn parse_const(raw: &'static str) -> ShadyProgram {
      ShadyProgram {
        kind: ProgramKind::Sprite,
        statements: Vec::new(),
      }
    }

    fn parse_string(&mut self, input: &str) -> Result<ShadyProgram, ParseError> {
      unimplemented!()
    }
  }

  /// Compile-time compilation of Shady programs.
  ///
  /// The result is the root AST that can later be compiled on-device.
  #[allow(unused_macros)]
  macro_rules! shady {
    ($raw:tt) => { Parser::parse_const(stringify!(raw)) }
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    const TEST_PROGRAM: ShadyProgram = shady!(r"
      #shader_type sprite
      #include 'palettes.shady'

      uniform sampler2D _ColorPalette;

      void fragment() {
        COLOR = sample_palette(_ColorPalette, sample(TEXTURE, UV));
      }
    ");
  }
}