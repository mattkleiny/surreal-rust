//! A lisp language implementation for the scripting backend.
//!
//! This implementation compiles down to the shared bytecode and executes on the stack-based machine.

use super::*;

/// A Lisp language implementation for the scripting backend.
struct LispLanguage {}

impl ScriptLanguage for LispLanguage {
  fn compile(&self, _program: &str) -> crate::Result<BytecodeProgram> {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_lex_a_simple_lisp_program() {
    todo!()
  }

  #[test]
  fn it_should_parse_a_simple_lisp_program() {
    todo!();
  }

  #[test]
  fn it_should_compile_a_simple_lisp_program() {
    todo!()
  }
}
