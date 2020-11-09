use super::shady::*;

type CompileResult<T> = std::result::Result<T, CompileError>;

/// A compiler for SPIR-V shader programs.
pub struct SpirvCompiler;

impl SpirvCompiler {
  pub fn compile(program: &ShadyProgram) -> CompileResult<SpirvProgram> {
    let mut _compiler = Self::new();
    let mut _result = SpirvProgram::new();

    for statement in &program.statements {
      match statement {
        Statement::Unknown => {}
        Statement::Empty => {}
        Statement::KindSpecification { .. } => {}
        Statement::MethodDefinition { .. } => {}
      }
    }

    unimplemented!()
  }

  fn new() -> Self {
    Self {}
  }
}

/// A SPIR-V compiled shader program.
pub struct SpirvProgram {
  binary: Vec<u8>,
}

impl SpirvProgram {
  fn new() -> Self {
    Self {
      binary: Vec::new()
    }
  }
}

#[derive(Debug)]
pub enum CompileError {
  Unknown,
  InvalidProgram,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_compile_a_simple_program() {
    let program = ShadyProgram::parse("TEST PROGRAM")
      .expect("Failed to parse program!");

    let program = SpirvCompiler::compile(&program)
      .expect("Failed to compile program!");
  }
}