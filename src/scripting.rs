//! Scripting support for Surreal.

use hlua::*;

use super::*;

/// A scripting engine for Lua.
pub struct ScriptEngine<'lua> {
  lua: Lua<'lua>,
}

impl<'lua> ScriptEngine<'lua> {
  pub fn new() -> Self {
    Self {
      lua: Lua::new(),
    }
  }

  /// Executes the given code on the engine.
  pub fn execute(&mut self, code: &String) -> Result<()> {
    match self.lua.execute(code.as_str()) {
      Ok(()) => Ok(()),
      Err(error) => Err(format!("Script error: {}", error))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_execute_basic_lua_instructions() {
    let mut engine = ScriptEngine::new();

    assert!(engine.execute(&"print 'Hello, World!'".to_string()).is_ok())
  }

  #[test]
  fn it_should_not_panic_for_bad_expression() {
    let mut engine = ScriptEngine::new();

    assert!(engine.execute(&"print Hello, World!".to_string()).is_err());
  }
}