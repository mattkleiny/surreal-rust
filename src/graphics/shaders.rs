/// Different types fo shaders supported by the engine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ShaderKind {
  Vertex,
  Fragment,
}

/// Represents a single raw shader program.
pub struct Shader {
  handle: ShaderHandle,
  kind: ShaderKind,
}

impl Shader {
  pub fn new(kind: ShaderKind, source: &impl AsRef<str>) -> Self {
    Self {
      handle: ShaderHandle::new(kind),
      kind,
    }
  }
}

/// A managed ID for OpenGL shaders.
#[derive(Debug, Eq, PartialEq)]
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

  pub fn bind(&self) {
    unsafe {
      gl::UseProgram(self.handle.0);
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
