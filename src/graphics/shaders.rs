use crate::RID;

pub struct Shader {
  id: RID
}

pub struct ShaderProgram {
  id: RID
}

#[cfg(feature = "shady")]
pub mod shady {
  //! A compilable shader language that permits us to write shader programs
  //! once and theoretically run them anywhere.

  // TODO: finish implementing this

  use super::*;

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

  pub trait Compiler {
    type Error;

    fn compile(&mut self) -> Result<ShaderProgram, Self::Error>;
  }

  fn parse(raw: impl AsRef<str>) -> Result<Vec<Node>, ParseError> {
    unimplemented!()
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    const TEST_PROGRAM: &'static str = r"
      #shader_type sprite

      uniform sampler2D palette_tex

      void vertex() {
        VERTEX += vec2(1.0, 2.0);
      }

      void fragment() {
        COLOR = sample(TEXTURE, UV);
      }
    ";

    #[test]
    fn it_should_parse_a_simple_program() {
      parse(TEST_PROGRAM).unwrap();
    }
  }
}
