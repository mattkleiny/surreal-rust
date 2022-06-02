//! Scripting language implementation for the 'Lox'language.
//!
//! Based on the work from the excellent book, 'Crafting Interpreters'.

use super::*;

struct LoxLanguage {}

impl ScriptLanguage for LoxLanguage {
  fn compile(&self, _program: &str) -> crate::Result<BytecodeChunk> {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_should_lex_a_simple_lox_program() {
    todo!()
  }

  #[test]
  fn it_should_parse_a_simple_lox_program() {
    todo!();
  }

  #[test]
  fn it_should_compile_a_simple_lox_program() {
    todo!()
  }
}
