/// Different types fo shaders supported by the engine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ShaderKind {
  Vertex,
  Fragment,
}

/// Represents a single compiled shader program.
#[derive(Debug)]
pub struct ShaderProgram {}

impl ShaderProgram {
  pub fn new() -> Self {
    Self {}
  }

  pub fn get_uniform_location(&self, name: &str) -> Option<usize> {
    Some(0)
  }

  pub fn set_uniform<T>(&self, location: usize, value: &T) {
    todo!()
  }
}
