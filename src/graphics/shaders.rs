// TODO: abstract over shading language, add debugging and profiling/etc?
// TODO: finish implementing the shady language.

use crate::RID;

pub struct ShaderProgram {
  id: RID
}

#[cfg(feature = "shady")]
pub mod shady {
  //! A compilable shader language that permits us to write shader programs
  //! once and theoretically run them anywhere.
  //!
  //! An extension of this would allow composition of shaders via a shader graph.

  use super::*;

  /// Represents a back-end compiler for Shady programs.
  pub trait Compiler {
    type Error;

    fn compile(&mut self, ast: &ShadyProgram) -> Result<ShaderProgram, Self::Error>;
  }

  #[derive(Copy, Clone, Debug, Eq, PartialEq)]
  enum TokenType {
    Empty,
    Identifier,
    True,
    False,
    Constant(Constant),
    Type(Type),
  }

  #[derive(Copy, Clone, Debug, Eq, PartialEq)]
  enum Constant {
    Int,
    Float,
  }

  #[derive(Copy, Clone, Debug, Eq, PartialEq)]
  enum Type {
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

  #[derive(Clone, Debug)]
  enum Expression {
    Operator {
      name: String,
      return_type: Type,
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
  enum Statement {}

  /// A parser for Shady programs.
  ///
  /// Turns raw text into the shady AST.
  #[derive(Clone, Debug)]
  pub struct Parser {}

  /// Possible errors when parsing.
  #[derive(Copy, Clone, Debug, Eq, PartialEq)]
  pub enum ParseError {}

  impl Parser {
    /// Parses a Shady program from the given string representation.
    pub fn parse(raw: impl AsRef<str>) -> Result<ShadyProgram, ParseError> {
      unimplemented!()
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
  }

  #[derive(Clone, Debug)]
  pub struct ShadyProgram {
    kind: ProgramKind,
    statements: Vec<Statement>,
  }

  /// Compile-time compilation of Shady programs.
  ///
  /// The result is the root AST that can later be compiled on-device.
  macro_rules! shady { ($raw:tt) => { Parser::parse_const(stringify!(raw)) } }

  #[cfg(test)]
  mod tests {
    use super::*;

    const TEST_PROGRAM: ShadyProgram = shady!(r"
      #shader_type sprite

      uniform sampler2D palette_tex

      void vertex() {
        VERTEX += vec2(1.0, 2.0);
      }

      void fragment() {
        COLOR = sample(TEXTURE, UV);
      }
    ");
  }
}
