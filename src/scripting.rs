//! Scripting support for Surreal.

use rlua::{Context, Lua};
pub use rlua::Function;

use super::*;

mod bindings;

/// A scripting engine for Lua.
pub struct ScriptEngine {
  lua: Lua,
}

impl ScriptEngine {
  pub fn new() -> Self {
    Self {
      lua: Self::create_interpreter(),
    }
  }

  /// Creates a new interpreter for the script engine.
  fn create_interpreter() -> Lua {
    let lua = Lua::new();

    // initialize the global table
    let result: rlua::Result<()> = lua.context(|context| {
      let globals = context.globals();

      let vec2_factory = context.create_function(|_, (x, y): (i32, i32)| {
        Ok(maths::Vec2i::new(x, y))
      })?;

      globals.set("Vec2", vec2_factory)?;

      Ok(())
    });

    match result {
      Err(error) => panic!("{}", error),
      _ => {}
    }

    lua
  }

  /// Sets the memory limit on the interpreter.
  pub fn set_memory_limit(&mut self, limit: Option<usize>) {
    self.lua.set_memory_limit(limit);
  }

  /// Evaluates the given code in the script runtime and returns the result.
  pub fn evaluate<F, R>(&mut self, body: F) -> R
    where F: FnOnce(Context) -> R {
    self.lua.context(body)
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_evaluate_basic_instructions_without_fault() {
    let mut engine = ScriptEngine::new();

    engine.evaluate(|context| {
      context.load("print 'Hello, World!'").exec().unwrap();
    });
  }
}