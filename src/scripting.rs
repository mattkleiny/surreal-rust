//! Scripting support for Surreal.

use rlua::{Lua, UserData, UserDataMethods};

use crate::diagnostics::*;

use super::*;
use crate::maths::Vec2i;

/// A scripting engine for Lua.
pub struct ScriptEngine {
  lua: Lua,
}

impl ScriptEngine {
  pub fn new() -> Self {
    Self {
      lua: Self::create_interpreter()
    }
  }

  /// Creates a new interpreter for the script engine.
  fn create_interpreter() -> Lua {
    let lua = Lua::new();

    // initialize the global table
    lua.context(|context| {
      let _globals = context.globals();

      // TODO: include path/etc.
    });

    lua
  }

  /// Sets the memory limit on the interpreter.
  pub fn set_memory_limit(&mut self, limit: Option<usize>) {
    self.lua.set_memory_limit(limit);
  }

  /// Advances the interpreter by a single frame.
  pub fn tick(&mut self, delta_time: f64) {
    self.lua.context(|context| {
      let globals = context.globals();

      match globals.set("delta_time", delta_time) {
        Err(error) => warn!("Script error: {}", error),
        _ => {}
      };
    })
  }

  /// Executes the given code on the engine.
  pub fn execute(&mut self, code: &String) -> Result<()> {
    self.lua.context(|context| {
      match context.load(code.as_str()).exec() {
        Err(error) => Err(format!("Script error: {}", error)),
        Ok(_) => Ok(())
      }
    })
  }
}

impl UserData for Vec2i {
  fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
    methods.add_method("magnitude", |_, vec, ()| { Ok(vec.x + vec.y) });
    methods.add_method("length", |_, vec, ()| { Ok(vec.x + vec.y) });
    methods.add_method("distance", |_, vec, ()| { Ok(vec.x + vec.y) });
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