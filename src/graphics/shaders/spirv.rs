//! A backend for compiling SPIR-V shaders from source.
//!
//! This implementation provides support for Shady-based programs but also allows procedurally
//! creating SPIR-V shaders (for other language variants, for example).
//!
//! The resultant `SpirvProgram` should then be linkable directly on the underlying graphics API.

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

    program.accept(&mut compiler);

    Ok(compiler.builder.build())
  }
}

impl super::shady::Visitor for SpirvCompiler {
  fn visit_module(&mut self, module: &Module) {
    match module {
      Module::Shared { .. } => {}
      Module::Shader { .. } => {}
    }

    unimplemented!()
  }

  fn visit_statement(&mut self, statement: &Statement) {
    match statement {
      Statement::Unknown => {}
      Statement::KindSpecification { .. } => {}
      Statement::MethodDefinition { .. } => {}
    }

    unimplemented!()
  }

  fn visit_expression(&mut self, expression: &Expression) {
    match expression {
      Expression::Unknown => {}
      Expression::Operator { .. } => {}
      Expression::Variable { .. } => {}
      Expression::FunctionCall { .. } => {}
      Expression::Intrinsic { .. } => {}
    }

    unimplemented!()
  }
}


/// A SPIR-V compiled shader program.
pub struct SpirvProgram(Vec<u32>);

/// A procedural builder for SPIR-V programs.
struct SpirvBuilder {
  builder: rspirv::dr::Builder,
}

impl SpirvBuilder {
  pub fn new() -> Self {
    Self { builder: rspirv::dr::Builder::new() }
  }

  pub fn build(self) -> SpirvProgram {
    use rspirv::binary::Assemble;

    let module = self.builder.module();
    let assembled = module.assemble();

    SpirvProgram(assembled)
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