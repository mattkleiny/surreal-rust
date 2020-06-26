// TODO: abstract over shading language, add debugging and profiling/etc?
// TODO: finish implementing the shady language.
// TODO: consider using PEST for shady implementation?

use crate::RID;

pub struct Shader {
  id: RID
}

#[cfg(feature = "shady")]
pub mod shady {
  //! A compilable shader language that permits us to write shader programs
  //! once and theoretically run them anywhere.

  use super::*;

  /// Represents a back-end compiler for Shady programs.
  pub trait Compiler {
    type Error;

    fn compile(&mut self, ast: &AST) -> Result<Shader, Self::Error>;
  }

  #[derive(Copy, Clone, Debug, Eq, PartialEq)]
  pub enum ParseError {}

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

  #[derive(Clone, Debug)]
  enum Node {
    Operator {
      name: String,
      return_type: Type,
      precision: Precision,
      operator: Operator,
      arguments: Vec<Node>,
    },
    Variable {
      name: String,
      is_const: bool,
    },
  }

  #[derive(Clone, Debug)]
  pub struct AST {
    nodes: Vec<AST>
  }

  struct Parser {}

  /// Parses a Shady program from the given string representation.
  fn parse(raw: impl AsRef<str>) -> Result<Vec<Node>, ParseError> {
    unimplemented!()
  }

  /// Parses the given raw shady program into it's AST representation.
  ///
  /// Any error during the process are lifted to compile errors.
  const fn parse_const(raw: &'static str) -> AST {
    AST { nodes: Vec::new() }
  }

  /// Compile-time compilation of Shady programs.
  ///
  /// The result is the root AST that can later be compiled on-device.
  macro_rules! shady { ($raw:tt) => { parse_const(stringify!(raw)) } }

  #[cfg(test)]
  mod tests {
    use super::*;

    const TEST_PROGRAM: AST = shady!(r"
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
