//! Compiler support for the scripting system.
//!
//! The compiler that we implement here will take a mid-level IR
//! and lower it to a shared stack-based bytecode that can be evaluated
//! by the virtual machine.
//!
//! Languages that implement this IR are simpler to produce and similarly
//! receive all of the shared benefits of this compiler (such as it's
//! optimizations).

use super::*;

/// Compiles the given mid-level IR into a bytecode program.
pub fn compile() -> crate::Result<BytecodeProgram> {
  let mut compiler = Compiler::new();

  compiler.compile()
}

/// A compiler that can compile a source code string into a bytecode program.
struct Compiler {}

impl Compiler {
  /// Creates a new compiler.
  pub fn new() -> Self {
    Self {}
  }

  /// Compiles the given source code into a bytecode program.
  pub fn compile(&mut self) -> crate::Result<BytecodeProgram> {
    todo!()
  }
}
