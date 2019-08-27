//! Scripting support for Surreal.

use rlua::prelude::*;
pub use rlua::prelude::LuaFunction;

use crate::diagnostics::*;

// TODO: implement an in-game console based on the script engine (lua).
// TODO: support 'interactive debugging' using an in-game console.
// TODO: build a console utility using imgui that will allow execution of arbitrary commands/display log output.

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

      let trace = context.create_function(|_, message: LuaString| { trace!("{}", message.to_str()?); Ok(()) })?;
      let debug = context.create_function(|_, message: LuaString| { debug!("{}", message.to_str()?); Ok(()) })?;
      let warn = context.create_function(|_, message: LuaString| { warn!("{}", message.to_str()?); Ok(()) })?;
      let error = context.create_function(|_, message: LuaString| { error!("{}", message.to_str()?); Ok(()) })?;

      globals.set("trace", trace)?;
      globals.set("debug", debug)?;
      globals.set("warn", warn)?;
      globals.set("error", error)?;

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
    where F: FnOnce(LuaContext) -> R {
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