//! Scripting support for Surreal.

use rlua::Lua;

use super::*;

/// A scripting engine for Lua.
pub struct ScriptEngine {
  lua: Lua,
}

impl ScriptEngine {
  pub fn new() -> Self {
    Self {
      lua: Lua::new(),
    }
  }

  /// Executes the given code on the engine.
  pub fn execute(&mut self, code: &String) -> Result<()> {
    self.lua.context(|context| {
      if let Err(error) = context.load(code.as_str()).exec() {
        return Err(format!("Script error: {}", error));
      }

      Ok(())
    })
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