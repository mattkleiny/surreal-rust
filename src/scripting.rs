//! Scripting support for Surreal.

use rlua::prelude::*;

use super::*;

/// An engine for script execution.
pub trait ScriptEngine {
  /// Executes the given code on in the engine.
  fn execute(&mut self, code: &String);
}

/// A scripting engine for Lua.
pub struct LuaScriptEngine {
  interpreter: Lua,
}

impl LuaScriptEngine {
  pub fn new() -> Self {
    Self {
      interpreter: Lua::new(),
    }
  }
}

impl ScriptEngine for LuaScriptEngine {
  fn execute(&mut self, code: &String) {
    self.interpreter.context(|context| {
      context.load(code.as_str()).exec().unwrap();
    });
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_execute_basic_lua_instructions() {
    let mut engine = LuaScriptEngine::new();
    engine.execute(&"print 'Hello, World!'".to_string());
  }
}