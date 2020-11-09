use super::shady::*;

type CompileResult<T> = std::result::Result<T, CompileError>;

/// A compiler for SPIR-V shader programs.
pub struct SpirvCompiler {
  builder: SpirvBuilder
}

impl SpirvCompiler {
  pub fn compile(program: &ShadyProgram) -> CompileResult<SpirvProgram> {
    let mut compiler = SpirvCompiler {
      builder: SpirvBuilder::new()
    };

    for statement in &program.statements {
      compiler.visit_statement(statement);
    }

    Ok(compiler.builder.build())
  }
}

impl super::shady::Visitor<()> for SpirvCompiler {
  fn visit_statement(&mut self, statement: &Statement) -> () {
    match statement {
      Statement::Unknown => {}
      Statement::Empty => {}
      Statement::KindSpecification { .. } => {}
      Statement::MethodDefinition { .. } => {}
    }

    unimplemented!()
  }

  fn visit_expression(&mut self, expression: &Expression) -> () {
    unimplemented!()
  }
}


/// A SPIR-V compiled shader program.
pub struct SpirvProgram(Vec<u8>);

/// A procedural builder for SPIR-V programs.
struct SpirvBuilder {
  raw: Vec<u8>,
}

impl SpirvBuilder {
  pub fn new() -> Self {
    Self { raw: Vec::new() }
  }

  pub fn build(self) -> SpirvProgram {
    SpirvProgram(self.raw)
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