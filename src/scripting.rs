//! Scripting support for Surreal.

use rlua::prelude::*;

use crate::diagnostics::*;

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
    // execute the block of code in-context on the lua interpreter
    self.interpreter.context(|context| {
      let result = context.load(code.as_str()).exec();

      // try not to panic, if possible
      match result {
        Err(error) => warn!("Script failed: {}", error),
        _ => {}
      }
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

  #[test]
  fn it_should_not_panic_for_bad_expression() {
    let mut engine = LuaScriptEngine::new();

    engine.execute(&"print Hello, World!".to_string());
  }
}