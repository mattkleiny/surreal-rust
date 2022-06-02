//! A lisp language implementation for the scripting backend.
//!
//! This implementation compiles down to the shared bytecode and executes on the stack-based machine.

use super::*;

struct LispLanguage {}

impl ScriptLanguage for LispLanguage {
  fn compile(&self, _program: &str) -> crate::Result<BytecodeChunk> {
    todo!()
  }
}

#[cfg(test)]
mod tests {
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
